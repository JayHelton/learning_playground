#![allow(unused_doc_comments)]
use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let flags = u16::from_be_bytes([buf[0], buf[1]]);
        /**
         * Protocols found here https://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml
         */
        let proto = u16::from_be_bytes([buf[2], buf[3]]);

        // Ignore everything thats not ipv4
        if proto != 0x800 {
            continue;
        }

        /**
         * No need to parse the packet ourself
         */
        let p = match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(p) => {
                let src = p.source_addr();
                let dst = p.destination_addr();
                let proto = p.protocol();
                eprintln!(
                    "{} -> {} {}b of protocol {:x}",
                    src,
                    dst,
                    p.payload_len(),
                    proto
                );
            }
            Err(e) => {
                eprintln!("ignoring packet: {:?}", e);
            }
        };
    }
}
