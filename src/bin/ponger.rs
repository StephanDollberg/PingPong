extern crate pingpong;

fn pong<Socket: pingpong::Sender>(mut sock: Socket, settings: &pingpong::Settings) {
    if settings.non_blocking {
        sock.set_busy(true).unwrap();
    }

    let mut buf: [u8; 65000] = [0; 65000];

    loop {
        let bytes_read = pingpong::read_busy_until_some(&mut sock, &mut buf).expect("Reading ping failed");

        if bytes_read == 0 {
            return;
        }

        sock.send_data(&buf[0..bytes_read]).expect("Sending pong failed");
    }
}

fn main() {
    let settings = pingpong::parse_settings();

    if settings.tcp {
        let acceptor = std::net::TcpListener::bind(format!("{}", settings.ponger_addr)).unwrap();

        loop {
            let client: std::net::TcpStream = acceptor.accept().unwrap().0;
            pong(client, &settings);
        }
    }
    if settings.udp {
        let sock = std::net::UdpSocket::bind(format!("{}", settings.ponger_addr)).unwrap();
        sock.connect(format!("{}", settings.pinger_addr)).unwrap();
        pong(sock, &settings);
    }
}
