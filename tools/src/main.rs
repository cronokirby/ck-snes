use std::io::{self, Write};

const BAUD_RATE: u32 = 115200;

fn read_line(buf: &mut String) -> anyhow::Result<&str> {
    buf.clear();
    let len = io::stdin().read_line(buf)?;
    Ok(buf[..len].trim_end())
}

fn read_index(buf: &mut String) -> anyhow::Result<Option<usize>> {
    Ok(read_line(buf)?.parse().ok())
}

fn read_hex_byte(buf: &mut String) -> anyhow::Result<Option<u8>> {
    Ok(u8::from_str_radix(read_line(buf)?, 16).ok())
}

fn main() -> anyhow::Result<()> {
    let ports = serialport::available_ports()?;
    println!("Choose a port:");
    for (i, port) in ports.iter().enumerate() {
        println!("{i}\t{port:?}");
    }
    let mut input_buf = String::with_capacity(256);
    let which_port = loop {
        print!("> ");
        io::stdout().flush()?;
        let Some(attempt) = read_index(&mut input_buf)? else {
            continue;
        };
        if attempt >= ports.len() {
            continue;
        }
        break attempt;
    };
    let mut port = serialport::new(&ports[which_port].port_name, BAUD_RATE).open()?;
    println!("Control LEDs:");
    loop {
        print!("> ");
        io::stdout().flush()?;
        let Some(attempt) = read_hex_byte(&mut input_buf)? else {
            continue;
        };
        port.write_all(&[attempt])?;
    }
}
