use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct Message {
    content: String,
}

async fn zone_zpex() -> impl Responder {
    HttpResponse::Ok().json(Message {
        content: String::from("Hello from Rust API!"),
    })
}

async fn get_message() -> impl Responder {
    let message = Message {
        content: String::from("Hello from Rust API!"),
    };
    HttpResponse::Ok().json(message)
}

async fn create_message(message: web::Json<Message>) -> impl Responder {
    // Echo back the received message
    HttpResponse::Created().json(message.0)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up logging to file
    let http_log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/var/log/rust_api.log")
        .expect("Failed to open log file");

    let error_log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/var/log/rust_api_error.log")
        .expect("Failed to open error log file");

    Builder::new()
        .target(env_logger::Target::Pipe(Box::new(http_log_file)))
        .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();

    println!("Server starting at http://0.0.0.0:80");
    
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%t '%r' %s %b '%{Referer}i' '%{User-Agent}i' %D"))
            .route("/", web::get().to(zone_zpex))
            .route("/message", web::get().to(get_message))
            .route("/message", web::post().to(create_message))
    })
    .bind("0.0.0.0:80")?
    .run()
    .await
}
