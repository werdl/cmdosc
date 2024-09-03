use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};
use std::net::{SocketAddrV4, UdpSocket};
use std::str::FromStr;
use std::time::Duration;
use std::{env, f32, thread};

use clap::{Parser, self};

#[derive(Clone, Debug)]
enum OscArg {
    Int(i32),
    Float(f32),
    String(String),
}

impl FromStr for OscArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(i) = s.parse::<i32>() {
            return Ok(OscArg::Int(i));
        }

        if let Ok(f) = s.parse::<f32>() {
            return Ok(OscArg::Float(f));
        }

        Ok(OscArg::String(s.to_string()))
    }
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(short, long, default_value = "8830")]
    recv_port: u16,

    #[clap(short, long, default_value = "8831")]
    send_port: u16,

    #[clap(short = 'a', long, default_value = "127.0.0.1")]
    recv_address: String,

    // network address to send OSC messages to
    send_address: String,

    // OSC message to send
    addr_osc: String,

    // OSC arguments
    args: Vec<OscArg>,
}

fn main() {
    let args = Cli::parse();

    let addr = format!("{}:{}", args.send_address, args.send_port);

    // socket that will send the OSC message to the recv socket
    let our_socket = UdpSocket::bind(addr).unwrap();

    let recv_addr = SocketAddrV4::from_str(&format!("{}:{}", args.recv_address, args.recv_port)).unwrap();

    let osc_msg = OscMessage {
        addr: args.addr_osc,
        args: args.args.iter().map(|arg| match arg {
            OscArg::Int(i) => OscType::Int(*i),
            OscArg::Float(f) => OscType::Float(*f),
            OscArg::String(s) => OscType::String(s.clone()),
        }).collect(),
    };

    let packet = OscPacket::Message(osc_msg);

    let encoded = encoder::encode(&packet).unwrap();

    our_socket.send_to(&encoded, recv_addr).unwrap();
}