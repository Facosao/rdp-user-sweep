use std::{
    net::{TcpStream, SocketAddr, SocketAddrV4, AddrParseError}, time::Duration, str::FromStr, thread::sleep,
};

fn ping(addr: &str) -> Result<(), AddrParseError> {
    //let addr = "172.31.88.18:3389";

    for _ in 0..4 {
        let stream = TcpStream::connect_timeout(
            &SocketAddr::from(SocketAddrV4::from_str(addr)?), 
            Duration::from_secs(3)
        );
        
        match stream {
            Ok(_) => println!("Response from {}", addr),
            Err(_) => println!("Connection timeout."),
        }

        sleep(Duration::from_secs(1));
    }


    Ok(())
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let _ = ping(&args[1]);
}