use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpStream},
    time::Duration,
    thread::sleep,
};

pub fn ping(ip: Ipv4Addr) -> Result<(), ()> {
    let socket = SocketAddrV4::new(ip, 3389); // RDP port number
    let mut attempts = 2;

    while attempts > 0 {
        attempts -= 1;

        let stream = TcpStream::connect_timeout(
            &SocketAddr::from(socket),
            Duration::from_secs(1)
        );
        
        match stream {
            Ok(_) => return Ok(()),
            Err(_) => {},
        }

        if attempts == 1 {
            sleep(Duration::from_secs(1));
        }
    }

    Err(())
}