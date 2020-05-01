// mod zk4lw;

// use actix_web::{web, App, HttpServer};
// use zk4lw::client::ZK4LWClient;
// use zk4lw::cmd::mntr::ZK4LWMonitor;

/*
conf
New in 3.3.0: Print details about serving configuration.

cons
New in 3.3.0: List full connection/session details for all clients connected to this server. Includes information on numbers of packets received/sent, session id, operation latencies, last operation performed, etc...

crst
New in 3.3.0: Reset connection/session statistics for all connections.

dump
Lists the outstanding sessions and ephemeral nodes. This only works on the leader.

envi
Print details about serving environment

ruok
Tests if server is running in a non-error state. The server will respond with imok if it is running. Otherwise it will not respond at all.

A response of "imok" does not necessarily indicate that the server has joined the quorum, just that the server process is active and bound to the specified client port. Use "stat" for details on state wrt quorum and client connection information.

srst
Reset server statistics.

srvr
New in 3.3.0: Lists full details for the server.

stat
Lists brief details for the server and connected clients.

wchs
New in 3.3.0: Lists brief information on watches for the server.

wchc
New in 3.3.0: Lists detailed information on watches for the server, by session. This outputs a list of sessions(connections) with associated watches (paths). Note, depending on the number of watches this operation may be expensive (ie impact server performance), use it carefully.

wchp
New in 3.3.0: Lists detailed information on watches for the server, by path. This outputs a list of paths (znodes) with associated sessions. Note, depending on the number of watches this operation may be expensive (ie impact server performance), use it carefully.

mntr
New in 3.4.0: Outputs a list of variables that could be used for monitoring the health of the cluster.
*/

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
