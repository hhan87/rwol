use std::env;
use std::net::{UdpSocket, ToSocketAddrs};

fn main() -> std::io::Result<()> {
    // 명령줄 인자 가져오기
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <INTERFACE_IP> <MAC_ADDRESS>", args[0]);
        std::process::exit(1);
    }
    let interface_ip = &args[1];
    let mac_address = &args[2];

    // WOL 패킷 생성
    let mut packet = [0u8; 102];
    packet[0..6].fill(0xFF); // 처음 6바이트는 FF
    let mac_bytes = mac_str_to_bytes(mac_address);
    for i in 1..17 {
        packet[i * 6..(i + 1) * 6].copy_from_slice(&mac_bytes);
    }

    // UDP 소켓 생성 및 패킷 전송
    let socket_addr = format!("{}:0", interface_ip);
    let socket = UdpSocket::bind(socket_addr)?;
    socket.set_broadcast(true)?;
    socket.send_to(&packet, "255.255.255.255:9")?;
    println!("Packet sent!");
    Ok(())
}

// MAC 주소 문자열을 바이트 배열로 변환
fn mac_str_to_bytes(mac_str: &str) -> [u8; 6] {
    mac_str.split(':')
           .map(|s| u8::from_str_radix(s, 16).unwrap())
           .collect::<Vec<_>>()
           .try_into()
           .unwrap_or_else(|_| [0; 6])
}

