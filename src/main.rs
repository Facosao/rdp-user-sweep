use std::net::Ipv4Addr;

pub mod ip;
pub mod ping;
pub mod query;
pub mod template;
pub mod args;


fn main() {
    let subnets = args::parse_args();

    for subnet in subnets {
        println!("subnet: {:?}", subnet);
        let hosts = ip::gen_hosts(subnet.0, subnet.1);
        let mut table: Vec<(Ipv4Addr, Vec<String>)> = Vec::new();
        for host in hosts {
            

            println!("host: {}", host);
            if let Ok(_) = ping::ping(host) {
                let query_result = query::query(host);
                match query_result {
                    Some(users) => {
                        table.push((host, users.clone()));
                    }

                    None => {
                        table.push((host, vec!["Unknown".to_string()]));
                    }
                }
            }
        }

        println!("--- table for subnet ---");
        println!("{:?}", table);
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