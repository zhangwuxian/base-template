use clap::Parser;
use common::log::init_app_log;
use log::info;
use common::config::manager::init_app_config_manager;

#[derive(Parser, Debug)]
#[command(author="ztom", version, about, long_about = None)]
#[command(next_line_help = true)]
struct ArgsParams {
    #[arg(short, long, default_value = "config/settings.toml")]
    settings: String,
    #[arg(short, long, default_value = "config/configs.toml")]
    conf: String,
}

#[tokio::main]
async fn main() {
    let args = ArgsParams::parse();
    // init app config
    init_app_config_manager(&args.settings, &args.conf).await;
    // init logger
    init_app_log();

    info!("start sample");
}
