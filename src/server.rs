use bcrypt;
use colored::*;
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process;
use std::time::Instant;

type Port = u16;
enum ServerCommand {
    Ping,
    Hash(String),
}
impl ServerCommand {
    fn new(input: &str) -> Option<Self> {
        if input.trim().len() == 0 {
            return None;
        }
        let parts: Vec<&str> = input.trim().split(" ").collect();
        match parts[0].to_uppercase().as_str() {
            "PING" => Some(ServerCommand::Ping),
            "HASH" if parts.len() > 1 => Some(ServerCommand::Hash(parts[1].to_string())),
            _ => None,
        }
    }
}
//Max port number in Linux is 2^16
const PORT: Port = 1999;
const BCRYPT_COST: u32 = 10;

fn startup_server_logs(listener: &TcpListener) -> std::io::Result<()> {
    process::Command::new("clear")
        .spawn()
        .expect("clear command failed to start")
        .wait()
        .expect("failed to wait");
    println!(
        "{} {} {} \n",
        "[STARTUP]".bright_green().bold(),
        "Running in",
        "SERVER MODE".bold().bright_blue()
    );
    let port = listener.local_addr()?.port();
    println!(
        "{} {} {}",
        "[INFO]".bold(),
        "- Listening in port",
        port.to_string().blue()
    );
    Ok(())
}

pub fn init_server() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{PORT}"))?;

    startup_server_logs(&listener)?;

    for stream in listener.incoming() {
        match stream {
            Ok(c) => handle_req(c)?,
            Err(err) => eprintln!("error while receiving req: {}", err),
        }
    }

    Ok(())
}

fn handle_req(mut socket: TcpStream) -> Result<(), Error> {
    let peer_port = socket.peer_addr().unwrap().port();
    println!(
        "{} {} {}",
        "[REQUEST]".blue().bold(),
        "Received connection from",
        peer_port.to_string().yellow()
    );

    let mut buffer = vec![0; 256];
    let read = socket.read(&mut buffer).expect("couldn't read from buffer");
    let request = String::from_utf8_lossy(&buffer[..read]);

    let command = ServerCommand::new(request.as_ref());
    match command {
        Some(ServerCommand::Ping) => {
            println!(
                "{} {} {}",
                "\t[INFO]".bright_cyan(),
                "Mode",
                "PING".bright_green()
            );
            socket.write(b"PONG").unwrap();
        }
        Some(ServerCommand::Hash(plain)) => {
            println!(
                "{} {} {}",
                "\t[INFO]".bright_cyan(),
                "Mode",
                "HASH".bright_green()
            );
            let h = hash(&plain);
            socket.write(format!("{h}").as_bytes()).unwrap();
        }
        None => {
            println!(
                "{} {} {}",
                "\t[ERROR]".bright_red(),
                "Mode",
                "UNKNOWN".bright_green()
            );
            socket
                .write(format!("unknown command: {}\n", request).as_bytes())
                .unwrap();
        }
    }
    socket.flush().unwrap();

    Ok(())
}

fn hash(plain: &str) -> String {
    println!("\t{} {} {}", "[INFO]".bright_cyan(), "Message:", plain);
    let tic = Instant::now();
    let hash = bcrypt::hash(plain, BCRYPT_COST).unwrap();
    let toc = tic.elapsed().as_millis();
    println!("\t{} {} {}", "[INFO]".bright_cyan(), "Result", hash);
    println!(
        "\t{} {} {} {}",
        "[INFO]".bright_cyan(),
        "Elapsed",
        toc.to_string().bright_red(),
        "ms".bright_red()
    );
    hash
}
