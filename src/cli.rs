use colored::Colorize;
use std::process::Command;
use super::config::HostConfig;

pub fn connect_to_host(host: &HostConfig) {
    println!("Connecting to {}", &host.name.green());

    Command::new("ssh")
        .arg(&host.name)
        .status()
        .expect("Could not connect via SSH");
}

pub fn ping_to_host(host: &HostConfig) {
    println!("Ping to {}", &host.name.bold().green());

    Command::new("ping")
        .arg("-c 5")
        .arg(&host.ip)
        .status()
        .expect("Could not ping host");
}
