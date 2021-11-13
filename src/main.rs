mod libhttp;
use libhttp::server::HTTPServer;

fn main() {
	dotenv::dotenv().ok();
	let addr = std::env::var("ADDR").unwrap();
	let port = std::env::var("PORT").unwrap();
	let static_folder = std::env::var("STATIC_FOLDER").unwrap();
	let address = format!("{}:{}", addr, port);
	let server = HTTPServer::new(address, static_folder);
	server.start();
}
