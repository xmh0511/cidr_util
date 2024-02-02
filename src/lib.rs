use std::net::Ipv4Addr;

pub trait Cidr {
    /// Convert a subnet mask to the `prefix_len` in the CIDR form `A.B.C.D/prefix_len`.
    fn to_prefix_len(&self) -> u32;

    /// Get IP ranges in CIRD `IP/prefix_len`.
    /// Return (start_ip, end_ip).
    fn ip_range(&self, prefix_len: u32) -> (Ipv4Addr, Ipv4Addr);
}

impl Cidr for Ipv4Addr {
    fn to_prefix_len(&self) -> u32 {
        let n = u32::from_be_bytes(self.octets());
        n.to_prefix_len()
    }

    fn ip_range(&self, prefix_len: u32) -> (Ipv4Addr, Ipv4Addr) {
        let n = u32::from_be_bytes(self.octets());
        n.ip_range(prefix_len)
    }
}

impl Cidr for u32 {
    /// The value of `*self` shall denote a valid IP address
    fn to_prefix_len(&self) -> u32 {
        let mut n = *self;
        let mut i = 0;
        while n > 0 {
            if n & 0x1 != 0 {
                i += 1;
            }
            n >>= 1;
        }
        i
    }

    /// The value of `*self` shall denote a valid IP address
    fn ip_range(&self, prefix_len: u32) -> (Ipv4Addr, Ipv4Addr) {
        let mask = !(0xFFFFFFFFu32 >> prefix_len);
        let n = *self;
        let start = n & mask;
        (
            Ipv4Addr::from(start.to_be_bytes()),
            Ipv4Addr::from((start | !mask).to_be_bytes()),
        )
    }
}

pub trait CidrPrefix {
    fn to_subnet_mask(&self) -> Ipv4Addr;
}

trait Blanket: Copy {}

impl Blanket for u32 {}

impl<T> CidrPrefix for T
where
    T: Blanket,
    u32: std::ops::Shr<T, Output = u32>,
{
    fn to_subnet_mask(&self) -> Ipv4Addr {
        let mask = !(0xFFFFFFFFu32 >> *self);
        Ipv4Addr::from(mask.to_be_bytes())
    }
}

#[cfg(test)]
mod cidr {
    use std::net::Ipv4Addr;

    use crate::{Cidr, CidrPrefix};

    #[test]
    fn test() {
        assert_eq!(24, Ipv4Addr::new(255, 255, 255, 0).to_prefix_len());
        assert_eq!(
            (Ipv4Addr::new(10, 0, 0, 0), Ipv4Addr::new(10, 0, 0, 255)),
            Ipv4Addr::new(10, 0, 0, 0).ip_range(24)
        );
        assert_eq!(24.to_subnet_mask(), Ipv4Addr::new(255, 255, 255, 0));
        assert_eq!(1.to_subnet_mask(), Ipv4Addr::new(128, 0, 0, 0));
    }
}
