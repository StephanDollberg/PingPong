extern crate pingpong;

fn print_stats(mut times: Vec<u32>) {
    times.sort();

    let p50 = times.len() as f64 * 0.5;
    let p95 = times.len() as f64 * 0.95;
    let p99 = times.len() as f64 * 0.99;
    let p999 = times.len() as f64 * 0.999;

    println!("Stats (ns): p50: {} p95: {} p99: {} p999: {} max: {}",
        times[p50 as usize],
        times[p95 as usize],
        times[p99 as usize],
        times[p999 as usize],
        times.last().unwrap(),
    );
}

fn send_single_ping<Socket: pingpong::Sender>(client: &mut Socket, msg: &[u8], recv_buf: &mut [u8]) -> usize {
    client.send_data(&msg).expect("Sending ping failed");
    return pingpong::read_busy_until_some(client, recv_buf).expect("Receiving pong failed");
}

fn ping<Socket: pingpong::Sender>(mut client: Socket, settings: &pingpong::Settings) -> Vec<u32> {
    if settings.non_blocking {
        client.set_busy(true).unwrap();
    }

    let msg_string = "x".to_string().repeat(settings.msg_size as usize);
    let msg: &[u8] = msg_string.as_bytes();
    let mut recv_buf: [u8; 65000] = [0; 65000];

    let mut times = Vec::new();
    times.reserve(settings.msg_count as usize);

    for _ in 0..settings.warm_up_count {
        send_single_ping(&mut client, &msg, &mut recv_buf);
    }

    for _ in 0..settings.msg_count {
        let start = std::time::Instant::now();
        let bytes_read = send_single_ping(&mut client, &msg, &mut recv_buf);
        let end = std::time::Instant::now();

        if bytes_read == 0 {
            return times;
        }

        if bytes_read != msg.len() {
            return times;
        }

        let duration = end.duration_since(start).subsec_nanos();
        times.push(duration);

        std::thread::sleep(std::time::Duration::from_millis(settings.sleep_time));
    }

    return times;
}

fn main() {
    let settings = pingpong::parse_settings();

    if settings.tcp {
        let client = std::net::TcpStream::connect(format!("{}", settings.ponger_addr)).unwrap();
        let times = ping(client, &settings);
        print_stats(times);
    }

    if settings.udp {
        let client = std::net::UdpSocket::bind(format!("{}", settings.pinger_addr)).unwrap();
        client.connect(format!("{}", settings.ponger_addr)).unwrap();
        let times = ping(client, &settings);
        print_stats(times);
    }
}
