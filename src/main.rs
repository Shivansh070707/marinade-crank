mod crank;
use std::process::Command;
fn main(){
    let validator_vote = "H4QVPxS7napq3NEYxqLhxbKi9nJ8s56dD2EQZGsyZ3sb".to_string();
    crank::run(validator_vote);
}// use actix_web::{web, App, HttpResponse, HttpServer, Responder};
// use std::thread;
// mod crank; // Import the crank crate

// async fn index() -> impl Responder {
//     HttpResponse::Ok().body("Hello, world!")
// }

// async fn handle_post(req_body: String) -> impl Responder {
//     // Assuming req_body contains the validator vote
//     let validator_vote = "H4QVPxS7napq3NEYxqLhxbKi9nJ8s56dD2EQZGsyZ3sb".to_string();
//     thread::spawn(|| {
//         crank::run(validator_vote)
//     }).join().expect("Thread panicked");
//     HttpResponse::Ok().body("Validator vote processed")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(index))
//             .route("/", web::post().to(handle_post)) 
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }
