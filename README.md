### Example
````Rust
use cidr_util::{Cidr, CidrPrefix};

fn main(){
    assert_eq!(24, Ipv4Addr::new(255, 255, 255, 0).to_prefix_len());
    assert_eq!(
        (Ipv4Addr::new(10, 0, 0, 0), Ipv4Addr::new(10, 0, 0, 255)),
        Ipv4Addr::new(10, 0, 0, 0).ip_range(24)
    );
    assert_eq!(24.to_subnet_mask(), Ipv4Addr::new(255, 255, 255, 0));
    assert_eq!(1.to_subnet_mask(), Ipv4Addr::new(128, 0, 0, 0));
}
````
