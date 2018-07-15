extern crate clap;

use std::io::Read;
use std::io::Write;
use clap::{Arg, App};

pub struct Settings {
    pub non_blocking: bool,
    pub warm_up_count: u64,
    pub msg_count: u64,
    pub sleep_time: u64,
    pub ponger_addr: String,
    pub pinger_addr: String,
    pub tcp: bool,
    pub udp: bool,
}

pub fn parse_settings() -> Settings {

    let matches = App::new("Ping/Pong")
                          .version("1.0")
                          .author("Stephan Dollberg <stephan.dollberg@gmail.com>")
                          .about("Latency testing")
                          .arg(Arg::with_name("non_blocking")
                               .long("--poll")
                               .short("p")
                               .help(""))
                          .arg(Arg::with_name("tcp")
                               .long("--tcp")
                               .short("t")
                               .required_unless("udp")
                               .help(""))
                          .arg(Arg::with_name("udp")
                               .long("--udp")
                               .short("u")
                               .required_unless("tcp")
                               .help(""))
                          .arg(Arg::with_name("warmup_messages")
                               .long("--warmup-messages")
                               .short("w")
                               .takes_value(true)
                               .help(""))
                          .arg(Arg::with_name("messages")
                               .long("--messages")
                               .short("m")
                               .takes_value(true)
                               .help(""))
                          .arg(Arg::with_name("sleep_time")
                               .long("--sleep-time")
                               .short("s")
                               .takes_value(true)
                               .help(""))
                          .arg(Arg::with_name("ponger")
                               .long("--ponger-addr")
                               .short("o")
                               .takes_value(true)
                               .help(""))
                          .arg(Arg::with_name("pinger")
                               .long("--pinger-addr")
                               .short("i")
                               .takes_value(true)
                               .help(""))
                          .get_matches();

    return Settings{
        non_blocking: matches.is_present("non_blocking"),
        warm_up_count: matches.value_of("warmup_messages").unwrap_or("1000").parse::<u64>().unwrap(),
        msg_count: matches.value_of("messages").unwrap_or("1000").parse::<u64>().unwrap(),
        sleep_time: matches.value_of("sleep_time").unwrap_or("0").parse::<u64>().unwrap(),
        ponger_addr: matches.value_of("remote").unwrap_or("localhost:10000").to_string(),
        pinger_addr: matches.value_of("local").unwrap_or("localhost:10001").to_string(),
        tcp: matches.is_present("tcp"),
        udp: matches.is_present("udp"),
    };
}

pub trait Sender {
    fn send_data(&mut self, &[u8]) -> std::io::Result<usize>;
    fn recv_data(&mut self, &mut [u8]) -> std::io::Result<usize>;
    fn set_busy(&self, busy: bool) -> std::io::Result<()>;
}

impl Sender for std::net::TcpStream {
    fn send_data(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        return self.write(buf);
    }

    fn recv_data(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        return self.read(buf);
    }

    fn set_busy(&self, busy: bool) -> std::io::Result<()> {
        self.set_nodelay(busy)?;
        self.set_nonblocking(busy)?;
        return Ok(());
    }
}

impl Sender for std::net::UdpSocket {
    fn send_data(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        return self.send(buf);
    }

    fn recv_data(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        return self.recv(buf);
    }

    fn set_busy(&self, busy: bool) -> std::io::Result<()> {
        self.set_nonblocking(busy)?;
        return Ok(());
    }
}

pub fn read_busy_until_some<Socket: Sender>(sock: &mut Socket, mut buf: &mut [u8]) -> std::io::Result<usize> {
    loop {
        return match sock.recv_data(&mut buf) {
            Ok(bytes_read) => Ok(bytes_read),
            Err(ref err) if err.kind() == std::io::ErrorKind::WouldBlock => continue,
            Err(ref err) if err.kind() == std::io::ErrorKind::ConnectionReset => Ok(0),
            Err(err) => Err(err),
        };
    }
}
