use chrono::Utc;
use colored::*;
use std::collections::HashMap;
use std::io::{Error, Write};
use std::net::{TcpListener, TcpStream};

type Port = u16;
type ConnectionsTracker = HashMap<Port, u8>;

//Max port number in Linux is 2^16
//If zero will ask to the OS for a free port
const PORT: Port = 0;

fn main() -> std::io::Result<()> {
    //The TCP connections are blocking in the std implementation, so there is no
    //need for any kind of concurrency checks
    //The reference of connections counting is moved around to simplify the server
    // let mut connections: ConnectionsTracker = HashMap::new();
    let mut connections: HashMap<u16, u8> = HashMap::new();

    let listener = TcpListener::bind(format!("127.0.0.1:{PORT}"))?;

    println!(
        "{} {} {}",
        "[INFO]".bold(),
        "- Listening in port",
        listener.local_addr()?.port().to_string().blue()
    );

    for stream in listener.incoming() {
        match stream {
            Ok(c) => handle_req(c, &mut connections)?,
            Err(err) => eprintln!("error while receiving req: {}", err),
        }
    }

    Ok(())
}

fn curr_time() -> i64 {
    Utc::now().timestamp()
}

fn increment_con(c: &mut ConnectionsTracker, port: Port) {
    c.entry(port).and_modify(|e| *e += 1).or_insert(1);
}

fn format_leaderboard(c: &ConnectionsTracker) -> String {
    let mut xd: Vec<(&Port, &u8)> = c.iter().map(|x| x).collect();
    xd.sort_by(|(_, x), (_, y)| x.partial_cmp(y).unwrap());

    let mut buf = String::new();
    for x in xd.iter() {
        let tmp = String::from(format!("\tPort {} -> {}\n", x.0, x.1));
        buf.push_str(&tmp);
    }

    buf
}

fn handle_req(mut socket: TcpStream, c: &mut ConnectionsTracker) -> Result<(), Error> {
    let peer_port = socket.peer_addr().unwrap().port();
    increment_con(c, peer_port);

    println!(
        "{} {} {}",
        "[NEW]".blue().bold(),
        "Received connection from",
        peer_port.to_string().yellow()
    );
    socket.write(
        format!(
            "Hi {:?} ty for communicating with us.\nThe current time is:{} \nThe leaderboard of request origins: \n{}",
            peer_port,
            curr_time(),
            format_leaderboard(c)
        )
        .as_bytes(),
    )?;
    socket.flush()?;
    Ok(())
}
