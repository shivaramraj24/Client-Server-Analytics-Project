use analytics_lib::csv::read_input_csv_file;

use server::start_server;

// Each defined rpc generates an async fn that serves the RPC
#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: cargo run -- [dataset]");
        println!("  [dataset]: either grades or albums");
        return;
    }

    let dataset = Box::new(read_input_csv_file(&format!("../{}.csv", args[1])));
    start_server(Box::leak(dataset)).await;
}