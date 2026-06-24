extern crate interface;

use analytics_lib::dataset::Dataset;
use interface::RPCInterface;
use futures::{future, prelude::*};
use tarpc::server::{BaseChannel, Channel};
use tarpc::tokio_serde::formats::Json;

pub mod solution;

// This is the type that implements the generated World trait. It is the business logic
// and is used to start the server.
#[derive(Clone)]
pub struct AnalyticsServer {
    dataset: &'static Dataset,
}

impl AnalyticsServer {
    pub fn new(dataset: &'static Dataset) -> AnalyticsServer {
        return AnalyticsServer { dataset };
    }
}

impl RPCInterface for AnalyticsServer {
    async fn hello(self, _context: tarpc::context::Context) -> String {
        return solution::hello();
    }


    async fn slow_rpc(self, _context: tarpc::context::Context) -> analytics_lib::dataset::Dataset {
        return solution::slow_rpc(self.dataset);
    }
    


    async fn fast_rpc(self, _context: tarpc::context::Context, query: analytics_lib::query::Query) -> analytics_lib::dataset::Dataset {
        return solution::fast_rpc(self.dataset, query);
    }
    
}

// Do not modify this code.
pub async fn start_server(dataset: &'static Dataset) {
    // Listen on a TCP address.
    let server_addr = "127.0.0.1:8080";
    let mut listener = tarpc::serde_transport::tcp::listen(server_addr, Json::default).await.unwrap();

    // Configure the listener
    listener.config_mut().max_frame_length(usize::MAX);

    listener
        // Ignore accept errors
        .filter_map(|r| future::ready(r.ok()))
        .map(|transport| BaseChannel::with_defaults(transport))
        // Process incoming channels
        .for_each(|channel| async {
            let server = AnalyticsServer::new(dataset);
            channel.execute(server.serve())
                .for_each(|response| async move {
                    tokio::spawn(response);
                }).await
        })
        .await;
}