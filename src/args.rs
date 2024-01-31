use std::{net::Ipv4Addr, str::FromStr};
use crate::ip::Ipv4u32;
use crate::subnet::Subnet;

pub fn parse_args() -> Vec<Subnet> {
    let args = std::env::args().into_iter();
    let arg = args.last().expect("No argument passed to the program!");
    let mut subnets: Vec<Subnet> = Vec::new();

    let file = std::fs::read_to_string(&arg);
    match file {
        Ok(file_string) => {
            // Try to read and parse text file with multiple subnets.

            for line in file_string.lines() {
                if let Ok(subnet) = parse_subnet(line) {
                    subnets.push(subnet);
                }
            }
        }

        Err(_) => {
            // Try and and see if arg is a single subnet.

            if let Ok(subnet) = parse_subnet(&arg.as_str()) {
                subnets.push(subnet);
            }
        }
    }

    subnets
}

// Parse an IPv4 subnet in the format 123.123.123.123/12
fn parse_subnet(subnet: &str) -> Result<Subnet, Box<dyn std::error::Error>> {
    let strings: Vec<&str> = subnet.split('/').collect();
    let addr_str: &&str = strings.first().ok_or("_")?;
    let mask_cidr_str: &&str = strings.last().ok_or("_")?;

    let addr: Ipv4Addr = Ipv4Addr::from_str(*addr_str)?;
    let mask: Ipv4Addr;

    if let Ok(mask_cidr) = u8::from_str(*mask_cidr_str) {
        if mask_cidr > 32 {
            return Err("_".into());
        }

        let mut mask_u32: u32 = 0;

        for i in ((32 - mask_cidr)..32).rev() {
            mask_u32 |= 1_u32.wrapping_shl(i as u32);
        }

        mask = Ipv4Addr::from_u32(mask_u32);
    } else {
        return Err("_".into());
    }

    //println!("addr = {}", addr);
    //println!("mask = {}", mask);
    Ok(Subnet{ip: addr, mask})
}