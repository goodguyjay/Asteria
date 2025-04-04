mod handler;
mod queue;
mod server;
mod telemetry;
mod utils;

fn main() {
    asn_info!("Starting Asteria Core System...");

    queue::init();
    telemetry::init();

    server::start();
}
