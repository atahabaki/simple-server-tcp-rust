use crossbeam_utils::thread;
use std::{
	io::{Read, Write},
	net::{TcpListener, TcpStream},
};

use crate::libhttp::header::HTTPResponseHeader;

use super::header::HTTPHeader;

pub struct HTTPServer {
	address: String,
	static_folder: String,
}

impl HTTPServer {
	pub fn new(address: String, static_folder: String) -> Self {
		let mut path = static_folder;
		if path.ends_with('/') {
			path = path[0..path.len() - 1].to_string();
		}
		HTTPServer {
			address,
			static_folder: path,
		}
	}
	pub fn start(&self) {
		if let Ok(listener) = TcpListener::bind(&self.address) {
			println!("Connection established: {}", self.address);
			for stream in listener.incoming() {
				match stream {
					Ok(stream) => {
						thread::scope(|s| {
							let handle = s.spawn(|_| {
								self.handle_req(&stream);
							});
							handle.join().unwrap();
						})
						.unwrap();
					}
					Err(e) => {
						eprintln!("Unable to connect: {}", e);
					}
				}
			}
		}
	}
	fn handle_req(&self, mut stream: &TcpStream) {
		let mut headers: Vec<HTTPHeader> = Vec::new();
		let mut header_data: String = "".into();
    let mut crlf_count = 0;
    let mut body = String::new();
		loop {
			if header_data.len() > 2 && &header_data[0..2] == "\r\n" {
        match &headers[0] {
          HTTPHeader::Post { path , version } => {
            if crlf_count == 1 {
              crlf_count = 0;
              break;
            }
            else {
              crlf_count = 1;
            }
          },
          HTTPHeader::Get{ path, version} => {
            break;
          },
          _ => {
            eprintln!("This method doesn't supported!..");
            break;
          }
        }
			}
      if crlf_count == 0 {
        if let Some(data) = self.read_req(&stream) {
          header_data = data;
        }
        if let Some(header) = HTTPHeader::from_string(header_data.clone()) {
          headers.push(header);
        }
      }
      else {
        if let Some(data) = self.read_req(&stream) {
          body = data;
        }
        else {
          body = "".into();
        }
      }
		}
		println!("HEADERS: {:?}", headers);
		match &headers[0] {
			HTTPHeader::Get { path, version } => {
				let mut _path = String::from(self.static_folder.clone());
				_path.push_str(path);
				if path.ends_with('/') {
					_path.push_str("index.html");
				}
				println!("{:?}", _path);
				if let Ok(contents) = std::fs::read_to_string(_path) {
					let mut res_headers: Vec<HTTPResponseHeader> = Vec::new();
					res_headers.push(HTTPResponseHeader::Status {
						version: version.clone(),
						status: "200 OK".into(),
					});
					self.send_res(&mut stream, contents, res_headers);
				} else {
					let mut res_headers: Vec<HTTPResponseHeader> = Vec::new();
					res_headers.push(HTTPResponseHeader::Status {
						version: version.clone(),
						status: "404 NOT FOUND".into(),
					});
					self.send_res(&mut stream, "".into(), res_headers);
				};
			}
			HTTPHeader::Post { path, version } => {
        let mut res_headers: Vec<HTTPResponseHeader> = Vec::new();
        res_headers.push(HTTPResponseHeader::Status {
          version: version.clone(),
          status: "200 OK".into(),
        });
        self.send_res(&mut stream, body, res_headers);
      }
			_ => {}
		}
	}

	fn send_res(&self, mut stream: &TcpStream, res: String, headers: Vec<HTTPResponseHeader>) {
		match stream.write(res.as_bytes()) {
			Ok(_) => println!("Response sent"),
			Err(e) => println!("Failed sending response: {}", e),
		}
	}

	fn read_req(&self, mut stream: &TcpStream) -> Option<String> {
		let mut buffer = [0u8; 4096];
		match stream.read(&mut buffer) {
			Ok(_) => {
				let data = String::from_utf8_lossy(&buffer);
				Some(data.into())
			}
			Err(e) => {
				eprintln!("Could not read the stream: {}", e);
				None
			}
		}
	}
}
