use std::{
    net::{TcpStream, SocketAddr, SocketAddrV4, Ipv4Addr, AddrParseError},
    time::Duration,
    str::FromStr,
    thread::sleep,
    process::Command,
};

trait IPV4as32bit {
    fn from_u32(address: u32) -> Ipv4Addr;
    fn to_u32(address: Ipv4Addr) -> u32;
}

impl IPV4as32bit for Ipv4Addr {
    fn from_u32(address: u32) -> Ipv4Addr {
        let mut octets: [u8; 4] = [0; 4];
        octets[0] = (address & (255 << 24)) as u8;
        octets[1] = (address & (255 << 16)) as u8;
        octets[2] = (address & (255 << 8)) as u8;
        octets[3] = (address & 255) as u8;

        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3])
    }

    fn to_u32(address: Ipv4Addr) -> u32 {
        let addr_octets = address.octets();
        for value in addr_octets {
            println!("value = {}", value);
        }

        let mut addr32: u32;

        addr32 = addr_octets[0].rotate_left(24) as u32;
        addr32 |= addr_octets[1].rotate_left(16) as u32;

        println!("mid stage = {}", addr32);

        addr32 |= addr_octets[2].wrapping_shl(8) as u32;
        addr32 |= addr_octets[3] as u32;

        addr32
    }
}

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

fn is_network_address(address: Ipv4Addr, mask: Ipv4Addr) -> bool {
    let addr_octets = address.octets();
    let mask_octets = mask.octets();

    for i in 0..addr_octets.len() {
        if (addr_octets[i] | mask_octets[i]) != mask_octets[i] {
            return false;
        }
    }

    return true;
}

fn is_broadcast_address(address: Ipv4Addr, mask: Ipv4Addr) -> bool {
    let addr_octets = address.octets();
    let mask_octets = mask.octets();

    for i in 0..addr_octets.len() {
        if (addr_octets[i] | mask_octets[i]) != 255 {
            return false;
        }
    }

    return true;
}

fn gen_hosts(address: Ipv4Addr, mask: Ipv4Addr) -> Vec<Ipv4Addr> {
    let mut addr_octets = address.octets();
    

    
    
    
    println!("start address: {}", address);
    println!("start mask   : {}", mask);

    let mut hosts: Vec<Ipv4Addr> = Vec::new();
    let mut addr32 = Ipv4Addr::to_u32(address);
    let mask32 = Ipv4Addr::to_u32(mask);

    println!("start addr32: {}", addr32);
    println!("start mask32: {}", mask32);
    //addr32 += 1;

    while (addr32 | mask32) != (u32::MAX) {
        hosts.push(Ipv4Addr::from_u32(addr32));
        //println!("current host = {}", Ipv4Addr::from_u32(addr32));
        addr32 += 1;
    }

    hosts
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    let all_hosts = gen_hosts(Ipv4Addr::from_str(&args[1]).unwrap(), Ipv4Addr::from_str(&args[2]).unwrap());
    for host in all_hosts {
        println!("host = {}", host);
    }

    /*

    let _ = ping(&args[1]);
    let raw_user = query(&args[1]);

    if raw_user.is_ok() {
        let response = parse(raw_user.unwrap());
        for item in response {
            println!("user: {}", item);
        }
    }

    */

    /*
    println!(" USERNAME              SESSIONNAME        ID  STATE   IDLE TIME  LOGON TIME");
    println!(" nome.sobrenome                            1  Disco        1:06  09/01/2024 08:22");
    println!(" fulano.ciclano        console             2  Ativo        1:06  09/01/2024 13:24");
     */
}