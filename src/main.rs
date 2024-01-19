use std::{
    net::{TcpStream, SocketAddr, SocketAddrV4, AddrParseError},
    time::Duration,
    str::FromStr,
    thread::sleep,
    process::{Command, Output},
};

fn ping(addr: &str) -> Result<(), AddrParseError> {
    let addr_port = format!("{}:3389", addr); // RDP port number
    
    for _ in 0..2 {
        let stream = TcpStream::connect_timeout(
            &SocketAddr::from(SocketAddrV4::from_str(&addr_port)?), 
            Duration::from_secs(1)
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

    /*
    println!(" USERNAME              SESSIONNAME        ID  STATE   IDLE TIME  LOGON TIME");
    println!(" nome.sobrenome                            1  Disco        1:06  09/01/2024 08:22");
    println!(" fulano.ciclano        console             2  Ativo        1:06  09/01/2024 13:24");
     */
}