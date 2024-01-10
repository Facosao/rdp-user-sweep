use std::{
    net::{TcpStream, SocketAddr, SocketAddrV4, AddrParseError},
    time::Duration,
    str::FromStr,
    thread::sleep,
    process::{Command, Output},
};

fn ping(addr: &str) -> Result<(), AddrParseError> {
    let addr_port = format!("{}:3389", addr); // RDP port number
    
    for _ in 0..4 {
        let stream = TcpStream::connect_timeout(
            &SocketAddr::from(SocketAddrV4::from_str(&addr_port)?), 
            Duration::from_secs(3)
        );
        
        match stream {
            Ok(_) => println!("Response from {}", addr_port),
            Err(_) => println!("Connection timeout."),
        }

        sleep(Duration::from_secs(1));
    }

    Ok(())
}

fn query(addr: &str) -> Result<Output, std::io::Error> {
    let query = Command::new(r"C:\Windows\System32\quser.exe")
        .arg(format!("/server:{}", addr))
        .output()?;

    println!("status: {}", query.status);
    println!("stdout: {}", String::from_utf8_lossy(&query.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&query.stderr));

    Ok(query)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let _ = ping(&args[1]);
    let _ = query(&args[1]);
}