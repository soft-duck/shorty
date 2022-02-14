use std::error::Error;
use actix_web::{get, web, App, HttpServer, Responder};

#[get("/s/{test}")]
async fn short(params: web::Path<String>) -> impl Responder {
	let test = params.into_inner();
	format!("Hello {test}")
}

#[get("/{rest}")]
async fn rest(params: web::Path<String>) -> impl Responder {
	let rest = params.into_inner();
	format!("Hello {rest}")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	HttpServer::new(||
		App::new()
			.service(short)
			.service(rest)
	)
		.bind(("127.0.0.1", 8080))?
		.run()
		.await?;

	Ok(())
}
