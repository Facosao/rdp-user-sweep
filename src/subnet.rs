use std::net::Ipv4Addr;
use std::sync::mpsc::Sender;
use std::thread;

use crate::ip::Ipv4u32;

#[derive(Debug, Clone, Copy)]
pub struct Subnet {
    pub ip: Ipv4Addr,
    pub mask: Ipv4Addr
}

impl std::fmt::Display for Subnet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.mask.is_broadcast() {
            return write!(f, "255.255.255.255");
        }

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

pub fn query_subnet(subnet: Subnet, tx: Sender<QueryResult>) {
    let hosts = crate::ip::gen_hosts(subnet.ip, subnet.mask);

    for host in hosts {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            query_host(host, tx_clone);
        });
    }

    drop(tx);
}

pub fn query_host(ip: Ipv4Addr, tx: Sender<QueryResult>) {
    if let Ok(_) = crate::ping::ping(ip) {
        let query_result = crate::query::query_user(ip);
        match query_result {
            Some(users) => {
                let _ = tx.send(QueryResult{ip, users});
            }

            None => {
                let _ = tx.send(QueryResult{ip, users: vec!["?".to_string()]});
            }
        }
    }
}