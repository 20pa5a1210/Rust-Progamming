fn main() {
    enum ip_addr_kind {
        v4,
        v6,
    }
    let four = ip_addr_kind::v4;
    let six = ip_addr_kind::v6;
    fn route(ip_kind: ip_addr_kind) {}
    route(ip_addr_kind::v4);

    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));
}
