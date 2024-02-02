//use std::{future::IntoFuture, net::Ipv4Addr};
//use tokio::task::JoinHandle;
use std::sync::mpsc;

use subnet::QueryResult;

pub mod ip;
pub mod ping;
pub mod query;
pub mod template;
pub mod args;
pub mod subnet;

//use crate::subnet::QueryResult;

fn main() {
    let subnets = args::parse_args();
    let (tx, rx) = mpsc::channel::<Vec<QueryResult>>();

    for subnet in subnets {
        let tx_clone = tx.clone();
        std::thread::spawn(move || {
            //println!("Attempting to create thread for subnet {}", subnet);
            if let Err(_) = tx_clone.send(crate::subnet::query_subnet(subnet)) {
                println!("Failed to create thread for subnet {}", subnet);
            }
        });

        //println!("Subnet: {}", subnet.ip);
        
        //for host in result {
        //    println!("ip: {}, user: {:?}", host.ip, host.users);
        //}       
    }

    drop(tx);
    //println!("Finished creating threads for subnets!");

    for receiver in rx {
        for host in receiver {
            println!("host: {}, users: {:?}", host.ip, host.users);
        }
    }

    /*
    println!(" USERNAME              SESSIONNAME        ID  STATE   IDLE TIME  LOGON TIME");
    println!(" nome.sobrenome                            1  Disco        1:06  09/01/2024 08:22");
    println!(" fulano.ciclano        console             2  Ativo        1:06  09/01/2024 13:24");
     */
}