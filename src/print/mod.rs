#![allow(warnings)]

mod utils;
use std::fmt;
use std::fs::File;
use std::io::{self, BufReader};
use std::net::IpAddr;
use std::path::Display;

enum HostStatus {
    Active,
    Inactive,
}

impl fmt::Display for HostStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HostStatus::Active => write!(f, "Active"),
            HostStatus::Inactive => write!(f, "Inactive"),
        }
    }
}

struct HostEntry {
    status: HostStatus,
    ip: IpAddr,
    name: String,
}

pub fn hello_world() {
    println!("Hello, world!");
}

pub fn format_print() {
    println!("{}", format!("The number is {}", 42));
}

pub fn print_contents(contents: std::io::Lines<BufReader<File>>) {
    let mut host_entries = Vec::<HostEntry>::new();
    for line in contents.map_while(Result::ok) {
        if let Ok(matched) = utils::extract_lines_with_numbers(&line) {
            if (matched.contains('#')) {
                let ip: IpAddr = matched.split_whitespace().collect::<Vec<&str>>()[1]
                    .parse()
                    .expect("Invalid IP address");
                let name = matched.split_whitespace().collect::<Vec<&str>>()[2].to_string();

                host_entries.push(HostEntry {
                    status: HostStatus::Inactive,
                    ip: ip,
                    name: name,
                });
            } else {
                let ip: IpAddr = matched.split_whitespace().collect::<Vec<&str>>()[0]
                    .parse()
                    .expect("Invalid IP address");
                let name = matched.split_whitespace().collect::<Vec<&str>>()[1].to_string();

                host_entries.push(HostEntry {
                    status: HostStatus::Active,
                    ip: ip,
                    name: name,
                });
            }
        }
    }

    println!(
        "{}",
        host_entries
            .iter()
            .map(|e| format!("{}: {} is {}", e.ip, e.name, e.status))
            .collect::<Vec<String>>()
            .join("\n")
    );
}
