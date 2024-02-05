use std::net::Ipv4Addr;
use std::sync::mpsc;
use std::thread;

use crate::ip::Ipv4u32;

#[derive(Debug, Clone, Copy)]
pub struct Subnet {
    pub ip: Ipv4Addr,
    pub mask: Ipv4Addr
}

impl std::fmt::Display for Subnet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mask32 = self.mask.to_u32();

        let mut detector: u32 = 1 << 31;
        let mut cidr_count: u32 = 0;

        while (detector | mask32) == mask32 {
            cidr_count += 1;
            detector >>= 1;
        }
        
        write!(f, "{}/{}", self.ip, cidr_count)
    }
}

pub struct QueryResult {
    pub ip: Ipv4Addr,
    pub users: Vec<String>
}

type PossibleHost = Option<QueryResult>;

pub fn query_subnet(subnet: Subnet) -> Vec<QueryResult> {
    let hosts = crate::ip::gen_hosts(subnet.ip, subnet.mask);
    let mut results: Vec<QueryResult> = Vec::new();
    let (tx, rx) = mpsc::channel::<PossibleHost>();


    for host in hosts {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            //println!("Attempting to create thread for host {}", host);
            if let Err(_) = tx_clone.send(query_host(host)) {
                println!("Failed to create thread for host {}!", host);
            }
        });
    }

    drop(tx);
    //println!("Finished creating threads for subnet {}", subnet);

    for receiver in rx {
        if let Some(host) = receiver {
            results.push(host);
        }
    }

    results
}

pub fn query_host(ip: Ipv4Addr) -> Option<QueryResult> {
    if let Ok(_) = crate::ping::ping(ip) {
        let query_result = crate::query::query_user(ip);
        match query_result {
            Some(users) => {
                println!("Host: {}, Users: {:?}", ip, users);
                return Some(QueryResult{ip, users});
            }

            None => {
                println!("Host: {}, Users: ?", ip);
                return Some(QueryResult{ip, users: vec!["Unknown".to_string()]});
            }
        }
    }

    None
}