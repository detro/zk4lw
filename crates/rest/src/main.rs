// mod zk4lw;

// use actix_web::{web, App, HttpServer};
// use zk4lw::client::ZK4LWClient;
// use zk4lw::cmd::mntr::ZK4LWMonitor;

// async fn handle_conf() -> impl Responder {
//     HttpResponse::Ok().body("conf response")
// }
//
// async fn handle_ruok() -> impl Responder {
//     HttpResponse::Ok().body("ruok response!")
// }

// async fn handle_mntr() -> String {
//     let client = ZK4LWClient::new("localhost", 21812);

//     let res: zk4lw::cmd::mntr::ZK4LWMonitorResponse = client.execute::<ZK4LWMonitor>().unwrap();

//     format!("{:?}", res)
// }

// #[actix_rt::main]
fn main() {
    // HttpServer::new(|| {
    //     App::new()
    //         // .route("/cmd/conf", rest::get().to(handle_conf))
    //         // .route("/cmd/ruok", rest::get().to(handle_ruok))
    //         .route("/cmd/mntr", rest::get().to(handle_mntr))
    // })
    // .bind("127.0.0.1:8080")?
    // .run()
    // .await
}
