use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct HostConfig {
    pub name: String,
    pub ip: String,
}

pub fn get_ssh_config_path() -> String {
    let home_dir = env::var("HOME").expect("Could not get HOME environment variable");
    let path = format!("{home_dir}/.ssh/config");

    return path;
}

pub fn read_ssh_config(path: &str) -> Result<Vec<HostConfig>, io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut hosts = Vec::new();
    let mut current_host = None;

    for line in reader.lines() {
        let line = line?;
        if line.trim().starts_with("Host ") && !line.contains("*") {
            if let Some(host) = current_host.take() {
                hosts.push(host);
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            current_host = Some(HostConfig {
                name: parts[1].to_string(),
                ip: String::new(),
            });
        } else if line.trim().starts_with("HostName ") {
            if let Some(host) = current_host.as_mut() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                host.ip = parts[1].to_string();
            }
        }
    }

    if let Some(host) = current_host {
        hosts.push(host);
    }

    Ok(hosts)
}
