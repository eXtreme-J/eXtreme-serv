// extern crate jemallocator;

// #[global_allocator]
// static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[ntex::web::get("/")]
async fn index() -> impl ntex::web::Responder {
    // "Hello, World!"
    ntex::web::HttpResponse::Ok().body("Hello world!")
}

#[ntex::web::get("/{name}")]
async fn hello(name: ntex::web::types::Path<String>) -> impl ntex::web::Responder {
    format!("Hello {}!", &name)
}

fn init() {
    // console_subscriber::init();
}

#[ntex::main]
async fn start_serv() -> std::io::Result<()> {
    ntex::web::HttpServer::new(|| ntex::web::App::new().service(index).service(hello))
        .bind(("0.0.0.0", 8080))?
        .workers(16)
        .maxconn(4096)
        .maxconnrate(100000)
        .keep_alive(60)
        .run()
        .await
}

fn main() -> std::io::Result<()> {
    init();
    start_serv()
}
