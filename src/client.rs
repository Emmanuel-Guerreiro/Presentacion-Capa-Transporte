use std::io::{Read, Write};
use std::net::TcpStream;

use colored::*;

use crate::{Cli, ClientCommand};

pub fn init_client(cli: Cli) {
    if cli.cmd == ClientCommand::Server {
        panic!("can't happen, we're running in client mode")
    }
    // startup_client_logs();
    match send(&cli.cmd) {
        Ok(response) => println!(
            "{} {} {} {}",
            "[INFO]".bold(),
            &cli.cmd.to_string().bright_blue(),
            "| Result: ",
            response.to_string().bright_yellow()
        ),
        Err(err) => eprintln!("❌ server error '{}'", err),
    }

    println!("{} {}", "[INFO]".bold(), "Closing connection".bright_red());
}

fn send(command: &ClientCommand) -> std::io::Result<String> {
    let command = command.to_string();
    let mut stream = TcpStream::connect("127.0.0.1:1999")?;
    new_stream_log(&stream)?;
    stream.write_all(command.as_bytes())?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response)
}

fn new_stream_log(stream: &TcpStream) -> std::io::Result<()> {
    println!(
        "{} {} {}",
        "[INFO]".bold(),
        "- Connected to",
        stream.peer_addr()?.to_string().to_string().blue()
    );
    Ok(())
}
