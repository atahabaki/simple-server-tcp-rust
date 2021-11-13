#[derive(Debug, PartialEq)]
pub enum HTTPHeader {
	Get { path: String, version: String },
	Post { path: String, version: String },
	Host { address: String },
	AcceptLanguage { lang: String },
}

impl HTTPHeader {
	fn get_or_post_header(from: String, req_type: &str) -> Option<HTTPHeader> {
		let mut iter = from.split(' ').peekable();
		let count = iter.clone().count();
		let mut path: String = "".into();
		let mut version: String = "".into();
		if count >= 3 {
			while let Some(c) = iter.next() {
				match c {
					"" | " " => continue,
					_ => {
						if c.starts_with("HTTP") {
							let mut ver_iter = c.split("\r\n");
							if let Some(ver) = ver_iter.next() {
								if ver.starts_with("HTTP") {
									version = ver.into();
								}
							}
						} else if c == req_type {
							continue;
						} else {
							path.push_str(c);
						}
					}
				}
			}
			return match req_type {
				"GET" => Some(HTTPHeader::Get { path, version }),
				"POST" => Some(HTTPHeader::Post { path, version }),
				_ => None,
			};
		}
		None
	}

	fn get_header(from: String) -> Option<HTTPHeader> {
		HTTPHeader::get_or_post_header(from, "GET")
	}

	fn post_header(from: String) -> Option<HTTPHeader> {
		HTTPHeader::get_or_post_header(from, "POST")
	}

	fn accept_language_header(from: String) -> Option<HTTPHeader> {
		let mut iter = from.split(':').peekable();
		if let Some(c) = iter.next() {
			if c.trim().to_uppercase() == "ACCEPT-LANGUAGE" {
				if let Some(lang) = iter.next() {
					return Some(HTTPHeader::AcceptLanguage {
						lang: lang.trim().into(),
					});
				}
			}
		}
		None
	}

	fn host_header(from: String) -> Option<HTTPHeader> {
		let mut iter = from.split(':').peekable();
		if let Some(c) = iter.next() {
			if c.trim().to_uppercase() == "HOST" {
				if let Some(address) = iter.next() {
					return Some(HTTPHeader::Host {
						address: address.trim().into(),
					});
				}
			}
		}
		None
	}

	pub fn from_string(from: String) -> Option<HTTPHeader> {
		let from: String = from.trim().into();
		return if from.starts_with("GET") {
			HTTPHeader::get_header(from)
		} else if from.starts_with("POST") {
			HTTPHeader::post_header(from)
		} else if from.to_uppercase().starts_with("HOST:") {
			HTTPHeader::host_header(from)
		} else if from.to_uppercase().starts_with("ACCEPT-LANGUAGE:") {
			HTTPHeader::accept_language_header(from)
		} else {
			None
		};
	}
}

#[cfg(test)]
mod header_test {
	use super::*;
	#[test]
	fn get_http_header_ignore_whitespace() {
		let get_header = "   GET     /welcome.html    HTTP/1.1    ";
		let should = HTTPHeader::Get {
			path: "/welcome.html".into(),
			version: "HTTP/1.1".into(),
		};
		let header = HTTPHeader::from_string(get_header.into());
		assert_eq!(header, Some(should));
	}
	#[test]
	fn post_http_header_ignore_whitespace() {
		let get_header = "   POST     /welcome.html    HTTP/1.1    ";
		let should = HTTPHeader::Post {
			path: "/welcome.html".into(),
			version: "HTTP/1.1".into(),
		};
		let header = HTTPHeader::from_string(get_header.into());
		assert_eq!(header, Some(should));
	}
	#[test]
	fn accept_language_header_ignore_whitespace() {
		let sent_header = "  Accept-Language:   fr  ";
		let header = HTTPHeader::from_string(sent_header.into());
		assert_eq!(
			header,
			Some(HTTPHeader::AcceptLanguage { lang: "fr".into() })
		)
	}
	#[test]
	fn host_header_ignore_whitespace() {
		let sent_header = "  Host: developer.mozilla.org  ";
		let header = HTTPHeader::from_string(sent_header.into());
		assert_eq!(
			header,
			Some(HTTPHeader::Host {
				address: "developer.mozilla.org".into()
			})
		)
	}
}

pub enum HTTPResponseHeader {
	Status { version: String, status: String },
}

impl HTTPResponseHeader {
	pub fn to_string(&self) -> String {
		match self {
			HTTPResponseHeader::Status { version, status } => {
				format!("{} {}", version, status)
			}
		}
	}
}
