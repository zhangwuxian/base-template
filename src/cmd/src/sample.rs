use clap::Parser;
use common::config::init_app_conf_by_path;
use common::log::init_app_log;
use log::info;

pub const DEFAULT_APP_CONFIG: &str = "config/conf.toml";

#[derive(Parser, Debug)]
#[command(author="ztom", version, about, long_about = None)]
#[command(next_line_help = true)]
struct ArgsParams {
    #[arg(short, long, default_value_t=String::from(DEFAULT_APP_CONFIG))]
    conf: String,
}

#[tokio::main]
async fn main() {
    let args = ArgsParams::parse();
    // init app config
    init_app_conf_by_path(&args.conf);
    // init logger
    init_app_log();

    info!("start sample");
}
