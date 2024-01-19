use std::{
    net::{TcpStream, SocketAddr, SocketAddrV4, AddrParseError},
    time::Duration,
    str::FromStr,
    thread::sleep,
    process::Command,
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

fn query(addr: &str) -> Result<String, std::io::Error> {
    let query = Command::new(r"C:\Windows\System32\quser.exe")
        .arg(format!("/server:{}", addr))
        .output()?;

    Ok(String::from_utf8_lossy(&query.stdout).to_string())
}

fn parse(raw_user: String) -> Vec<String> {
    let splices = raw_user.split_whitespace();
    let mut users: Vec<String> = Vec::new(); 

    for splice in splices {
        if splice.contains(".") {
            users.push(splice.to_string());
        }
    }

    users
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let _ = ping(&args[1]);
    let raw_user = query(&args[1]);

    if raw_user.is_ok() {
        let response = parse(raw_user.unwrap());
        for item in response {
            println!("user: {}", item);
        }
    }

    /*
    println!(" USERNAME              SESSIONNAME        ID  STATE   IDLE TIME  LOGON TIME");
    println!(" nome.sobrenome                            1  Disco        1:06  09/01/2024 08:22");
    println!(" fulano.ciclano        console             2  Ativo        1:06  09/01/2024 13:24");
     */
}