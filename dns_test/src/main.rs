use std::net::ToSocketAddrs;

fn main() -> std::io::Result<()> {
    let hostname = "google.com";
    let addrs = format!("{}:80", hostname).to_socket_addrs()?;

    println!("IP addresses for {}:", hostname);
    for addr in addrs {
        println!("{}", addr.ip());
    }
    Ok(())
}
