use tokio::net::UdpSocket;
use std::{io, collections::{HashMap}, net::SocketAddr};

struct Player {
    pub name: String,
    pub x: f64,
    pub y: f64,
}

impl Player {
    fn as_bytes(&self) -> Vec<u8> {
        let mut result = self.name.as_bytes().to_vec();
        result.append(&mut self.x.to_be_bytes().to_vec());
        result.append(&mut self.y.to_be_bytes().to_vec());
        result
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("127.0.0.1:8080").await?;

    let mut clients: HashMap<SocketAddr, Player> = HashMap::new();

    let mut buf = [0; 1024];
    loop {
        let (len, addr) = sock.recv_from(&mut buf).await?;
        println!("{:?} bytes received from {:?}", len, addr);

        if len == 26 { // NOTE: names have maximum length of 10 characters
            if let Ok(name) = std::str::from_utf8(&buf[..10]) {
                let x = f64::from_be_bytes((&buf[10..18]).try_into().unwrap());
                let y = f64::from_be_bytes((&buf[18..26]).try_into().unwrap());
                if let Some(player) = clients.get_mut(&addr) {
                    player.x = x;
                    player.y = y;
                } else {
                    let player = Player{name: name.to_string(), x, y};
                    clients.insert(addr, player);
                }
    
                for client in &clients {
                    if *client.0 != addr {
                        let len = sock.send_to(&client.1.as_bytes(), addr).await?;
                        println!("{:?} bytes sent", len);
                    }
                }
            } // else nothing
        } // else nothing
    }
}