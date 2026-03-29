use std::io;

/// This holds all of the data for a SNES game, including code, music, and graphics.
pub struct Rom {
    bytes: Vec<u8>,
}

impl Rom {
    /// Construct a new ROM, given a blob of bytes.
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Decompile this ROM, writing it to some sink.
    ///
    /// This takes in anything implementing [`io::Write`]
    pub fn decompile(&self, writer: &mut impl io::Write) -> anyhow::Result<()> {
        let entry: u16 = 0xfffc;
        let mut i = self.u16(entry);
        loop {
            match self.u8(i) {
                0x78 => writeln!(writer, "  sei")?,
                x => todo!("unknown instr {x:02X} at {i:04X}"),
            }
            i += 1;
        }
    }

    fn translate(&self, at: u16) -> usize {
        at as usize - 0x8000
    }

    fn u8(&self, at: u16) -> u8 {
        self.bytes[self.translate(at)]
    }

    /// Read a u16 from a particular location, panicking upon failure.
    ///
    /// Panicking is usually the right choice, in the context of decompilation,
    /// because the ROM is malformed if this is going to fail, or there's a bug
    /// in our program. Either way, just terminating the program is probably for
    /// the better.
    fn u16(&self, at: u16) -> u16 {
        let addr = self.translate(at);
        u16::from_le_bytes(
            self.bytes[addr..addr + 2]
                .try_into()
                .expect("expected 2 bytes"),
        )
    }
}
