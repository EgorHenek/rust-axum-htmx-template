use bingo::{config::load_config, setup_tracing, start_main_server, start_metrics_server};

#[tokio::main]
async fn main() {
    let config = load_config().unwrap();
    setup_tracing();

    let (_main_server, _metrics_server) =
        tokio::join!(start_main_server(config), start_metrics_server(config));
}
