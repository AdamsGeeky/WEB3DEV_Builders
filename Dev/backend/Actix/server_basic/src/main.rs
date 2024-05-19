use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Good to go lets get rusty !!!")
}

#[post("/api")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("hey lets get rusty!")
}
// main
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // console notification startup
    println!("\x1B[0;32m🚀 Starting Actix Web server at http://127.0.0.1:5000\x1B[0m");

    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(echo)
            .route("/api", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
    .map(|_| {
        // on shutdown
        println!("\x1B[0;31m🛑 Actix Web server stopped\x1B[0m");
    })
}
