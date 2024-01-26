use std::net::Ipv4Addr;
use tokio::task::JoinHandle;

#[derive(Debug, Clone, Copy)]
pub struct Subnet {
    pub ip: Ipv4Addr,
    pub mask: Ipv4Addr
}

pub struct QueryResult {
    pub ip: Ipv4Addr,
    pub users: Vec<String>
}

pub async fn query_subnet(subnet: Subnet) -> Vec<JoinHandle<Option<QueryResult>>> {
    let hosts = crate::ip::gen_hosts(subnet.ip, subnet.mask);
    let mut results: Vec<JoinHandle<Option<QueryResult>>> = Vec::new();

    for host in hosts {
        results.push(tokio::task::spawn_blocking(move || {
            query_host(host)
        }));
    }

    return results;
}

pub fn query_host(ip: Ipv4Addr) -> Option<QueryResult> {
    if let Ok(_) = crate::ping::ping(ip) {
        let query_result = crate::query::query_user(ip);
        match query_result {
            Some(users) => {
                return Some(QueryResult{ip, users});
            }

            None => {
                return Some(QueryResult{ip, users: vec!["Unknown".to_string()]});
            }
        }
    }

    None
}