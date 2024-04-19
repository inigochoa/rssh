use colored::Colorize;
use comfy_table::{Table, Cell, presets::NOTHING};
use std::io::{self, Write};
use super::config::HostConfig;

const COLUMNS: usize = 5;

pub fn list_hosts(hosts: &[HostConfig]) {
    println!("{}", "Available hosts:".bold().underline());
    println!("");

    let mut table = Table::new();
    table.load_preset(NOTHING);

    let mut row: Vec<Cell> = Vec::new();

    for (index, host) in hosts.iter().enumerate() {
        let content = format!("{} {}", (index + 1).to_string().bold().cyan(), host.name);
        row.push(Cell::new(&content));

        if 0 == (index + 1) % COLUMNS {
            table.add_row(row);
            row = Vec::new();
        }
    }

    if !row.is_empty() {
        while COLUMNS > row.len() {
            row.push(Cell::new(""));
        }

        table.add_row(row);
    }

    println!("{}", table);
    println!("");
}

pub fn select_host(host_count: usize) -> usize {
    loop {
        let mut choice = String::new();

        print!("{}", "Select a host: ".yellow());

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut choice).expect("Could not read choice");

        match choice.trim().parse::<usize>() {
            Ok(num) if 0 < num && host_count >= num => return num,
            _ => eprintln!("Not a valid choice, please try again"),
        }
    }
}
