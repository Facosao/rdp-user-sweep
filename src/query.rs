use std::{net::Ipv4Addr, process::Command};

pub fn query_user(addr: Ipv4Addr) -> Option<Vec<String>> {
    //println!("QUERY {}", addr);
    let query = Command::new(r"C:\Windows\System32\quser.exe")
        .arg(format!("/server:{}", addr))
        .output();

    match query {
        Ok(output) => {
            parse(String::from_utf8_lossy(&output.stdout).to_string())
        }

        Err(_) => None
    }
}

fn parse(raw_user: String) -> Option<Vec<String>> {
    let splices = raw_user.split_whitespace();
    let mut users: Vec<String> = Vec::new(); 

    for splice in splices {
        if splice.contains(".") {
            users.push(splice.to_string());
        }
    }

    if users.is_empty() {
        return None;
    }

    Some(users)
}