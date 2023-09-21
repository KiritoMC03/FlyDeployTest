use std::sync::Mutex;

use actix_web::{
    get,
    post,
    web,
    App,
    HttpResponse,
    HttpServer,
    Responder,
    middleware::Logger
};

struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("ip: 127.0.0.1:8080");
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState {
                app_name: "Actix web".to_string(),
            }))
            .app_data(counter.clone())
            .service(hello)
            .service(echo)
            .service(web::scope("/app").route("/index.html", web::get().to(index)))
            .route("/hey", web::get().to(manual_hello))
            .route("/mut_state_test", web::get().to(mut_state_test))
    })
        .bind(("127.0.0.1", port))?
        .run();
    println!("Server created. Request listening starting...");
    let res = server.await;
    println!("Server closed.");
    res
}

async fn index(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name;
    format!("Hello {app_name}")
}

async fn mut_state_test(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;

    format!("Requests number: {counter}")
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}