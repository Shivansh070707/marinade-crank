mod crank;
use std::process::Command;
fn main(){
    let validator_vote = "H4QVPxS7napq3NEYxqLhxbKi9nJ8s56dD2EQZGsyZ3sb".to_string();
    crank::run(validator_vote);
    let mut command = Command::new("bash");
    // Pass the script name as an argument
    command.arg("./target/debug/marinade-crank --vote-account H4QVPxS7napq3NEYxqLhxbKi9nJ8s56dD2EQZGsyZ3sb --simulate --keypair ./id.json --cluster https://api.mainnet-beta.solana.com");
    // Execute the command
    // This will download a file called ncbi_dataset.zip in the current directory
    command.output().expect("Failed to execute command");
}
// use actix_web::{post, web, App, HttpServer, Responder};
// use cron::Schedule;
// use std::collections::HashMap;
// use std::str::FromStr;
// use std::sync::{Arc, Mutex};
// use tokio::task;
// use tokio::time::{sleep, Duration};
// use chrono::Utc;
// use serde::Deserialize;
// use uuid::Uuid;

// #[derive(Deserialize)]
// struct UserRequest {
//     user_id: String,
// }

// struct AppState {
//     jobs: Mutex<HashMap<String, tokio::task::JoinHandle<()>>>,
// }

// async fn job(user_id: String) {
//     println!("Job for user {} is running...", user_id);
//     let validator_vote = "H4QVPxS7napq3NEYxqLhxbKi9nJ8s56dD2EQZGsyZ3sb".to_string();
//     crank::run(validator_vote);
//     // Add your job logic here
// }

// async fn start_scheduler(user_id: String) {
//     let expression = "*/60 * * * * * *"; // Every 30 seconds
//     let schedule = Schedule::from_str(expression).unwrap();
//     let mut iter = schedule.upcoming(Utc);

//     loop {
//         let next = iter.next().unwrap();
//         let now = Utc::now();
//         let duration = next - now;
//         sleep(Duration::from_secs(duration.num_seconds() as u64)).await;
//         job(user_id.clone()).await;
//     }
// }

// #[post("/post")]
// async fn start_cron_job(
//     data: web::Data<AppState>,
//     user_request: web::Json<UserRequest>,
// ) -> impl Responder {
//     let user_id = user_request.user_id.clone();

//     let mut jobs = data.jobs.lock().unwrap();
//     if !jobs.contains_key(&user_id) {
//         let user_id_clone = user_id.clone();
//         let handle = tokio::spawn(async move {
//             start_scheduler(user_id_clone).await;
//         });

//         jobs.insert(user_id, handle);
//         "Cron job started, running every 30 seconds".to_string()
//     } else {
//         "Cron job is already running".to_string()
//     }
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let app_state = web::Data::new(AppState {
//         jobs: Mutex::new(HashMap::new()),
//     });

//     HttpServer::new(move || {
//         App::new()
//             .app_data(app_state.clone())
//             .service(start_cron_job)
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }
