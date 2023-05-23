mod cmd;
mod config;
mod repo;
mod router;
mod service;

use crate::config::Conf;
use cmd::{Cli, Commands, Parser};
use core::panic;
use repo::get_pool;
use service::Service;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // println!("two: {:?}", cli.two);
    // println!("one: {:?}", cli.one);

    // You can check the value provided by positional arguments, or option arguments
    if let Some(two) = cli.two.as_deref() {
        println!("Value for two: {two}");
    }

    if let Some(one) = cli.one.as_deref() {
        println!("Value for one: {one}");
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Start { config }) => {
            if let Some(config) = config.as_deref() {
                println!("config path: {}", config.display());
                let c = Conf::parse(config);
                // println!("config is: {:?}", &c);
                if let Ok(c) = c {
                    println!("config is: {:?}", &c);
                    let pool = get_pool(&c).await;
                    let service = Service::new(c, pool);
                    service.start().await;
                } else {
                    println!("Error parsing config file");
                    panic!("Error parsing config file");
                }
            } else {
                println!("No config file");
                panic!("No config to start");
            }
        }
        None => {}
    }
}
