//! Binary entry point for the Bank external rail: a real TCP process,
//! separate from any Query runtime, with controllable exit-proof faults.

use std::io::Write;
use std::net::SocketAddr;

use bank_external_rail::{RailProtocolSupportProfile, RailServer};

#[tokio::main]
async fn main() {
    let bind_addr = parse_bind_addr();
    let protocol_support = parse_protocol_support();
    let server = RailServer::bind_with_protocol_support(bind_addr, protocol_support)
        .await
        .unwrap_or_else(|error| {
            eprintln!("bank-external-rail: failed to bind {bind_addr}: {error}");
            std::process::exit(2);
        });
    let local_addr = server
        .local_addr()
        .expect("bank-external-rail: bound listener reports its own address");

    println!("LISTENING {local_addr}");
    std::io::stdout()
        .flush()
        .expect("bank-external-rail: stdout is writable at startup");

    let error = server.serve().await.unwrap_err();
    eprintln!("bank-external-rail: listener failed: {error}");
    std::process::exit(1);
}

fn parse_protocol_support() -> RailProtocolSupportProfile {
    let value = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "current".to_owned());
    RailProtocolSupportProfile::parse_command_line(&value).unwrap_or_else(|| {
        eprintln!("bank-external-rail: invalid protocol support profile: {value}");
        std::process::exit(2);
    })
}

fn parse_bind_addr() -> SocketAddr {
    std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:0".to_string())
        .parse()
        .unwrap_or_else(|error| {
            eprintln!("bank-external-rail: invalid bind address argument: {error}");
            std::process::exit(2);
        })
}
