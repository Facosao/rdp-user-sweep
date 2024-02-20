use std::{net::Ipv4Addr, process::Command};

pub fn query_user(addr: Ipv4Addr) -> Option<Vec<String>> {
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

/*
println!(" USERNAME              SESSIONNAME        ID  STATE   IDLE TIME  LOGON TIME");
println!(" nome.sobrenome                            1  Disco        1:06  09/01/2024 08:22");
println!(" fulano.ciclano        console             2  Ativo        1:06  09/01/2024 13:24");
*/