use clap::Parser;
use colored::Colorize;

mod cli;
mod config;
mod display;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// List available hosts
    #[arg(short, long)]
    list: bool,

    /// Ping selected host 5 times
    #[arg(short, long)]
    ping: bool,
}

fn main() {
    let args = Args::parse();
    let path = config::get_ssh_config_path();

    match config::read_ssh_config(&path) {
        Ok(hosts) => {
            if hosts.is_empty() {
                println!("{}", "No hosts found.".red());
                return;
            }

            display::list_hosts(&hosts);
            if args.list {
                return;
            }

            let choice = display::select_host(hosts.len());

            if args.ping {
                cli::ping_to_host(&hosts[choice - 1]);
            } else {
                cli::connect_to_host(&hosts[choice - 1]);
            }
        },
        Err(e) => eprintln!("ERROR: {}", e.to_string().red()),
    }
}
