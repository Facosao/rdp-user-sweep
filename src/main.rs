use std::sync::mpsc;
use subnet::QueryResult;

pub mod ip;
pub mod ping;
pub mod query;
pub mod template;
pub mod args;
pub mod subnet;

fn main() {
    let subnets = args::parse_args();
    let (tx, rx) = mpsc::channel::<QueryResult>();

    for subnet in subnets {
        let tx_clone = tx.clone();
        std::thread::spawn(move || {
            crate::subnet::query_subnet(subnet, tx_clone);
        });
    }

    drop(tx);

    for receiver in rx {
        println!("Host: {}, Users: {:?}", receiver.ip, receiver.users);
    }
}