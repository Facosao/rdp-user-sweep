use std::net::Ipv4Addr;

pub trait Ipv4u32 {
    fn from_u32(address: u32) -> Ipv4Addr;
    fn to_u32(self) -> u32;
}

impl Ipv4u32 for Ipv4Addr {
    fn from_u32(address: u32) -> Ipv4Addr {
        let mut octets: [u8; 4] = [0; 4];
        octets[0] = ((address & (255 << 24)) >> 24) as u8;
        octets[1] = ((address & (255 << 16)) >> 16) as u8;
        octets[2] = ((address & (255 << 8)) >> 8) as u8;
        octets[3] = (address & 255) as u8;

        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3])
    }

    fn to_u32(self) -> u32 {
        let addr_octets: [u8; 4] = self.octets();
        let mut addr32: u32;

        addr32 = (addr_octets[0] as u32).wrapping_shl(24);
        addr32 |= (addr_octets[1] as u32).wrapping_shl(16);
        addr32 |= (addr_octets[2] as u32).wrapping_shl(8);
        addr32 |= addr_octets[3] as u32;

        addr32
    }
}

fn _is_network_address(address: Ipv4Addr, mask: Ipv4Addr) -> bool {
    let addr_octets: [u8; 4] = address.octets();
    let mask_octets: [u8; 4] = mask.octets();

    for i in 0..addr_octets.len() {
        if (addr_octets[i] | mask_octets[i]) != mask_octets[i] {
            return false;
        }
    }

    return true;
}

pub fn gen_hosts(address: Ipv4Addr, mask: Ipv4Addr) -> Vec<Ipv4Addr> {    
    let mut hosts: Vec<Ipv4Addr> = Vec::new();

    if mask.to_u32() == u32::MAX { // /32 mask
        hosts.push(address);
        return hosts;
    }

    if mask.to_u32() == (u32::MAX) - 1 { // /31 mask (RFC 3021)
        hosts.push(address);
        hosts.push(Ipv4Addr::from_u32(address.to_u32() + 1));
        return hosts;
    }

    let mut addr32 = address.to_u32();
    let mask32 = mask.to_u32();

    addr32 += 1; // Skip network address

    while (addr32 | mask32) != (u32::MAX) {
        hosts.push(Ipv4Addr::from_u32(addr32));
        addr32 += 1;
    }

    hosts
}