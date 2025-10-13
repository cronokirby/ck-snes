use crate::bus::Bus;

#[derive(Debug)]
enum MuOp {
    Read,
    Exec,
}

impl Default for MuOp {
    fn default() -> Self {
        Self::Read
    }
}

#[derive(Debug, Default)]
pub struct Cpu {
    instr: MuOp,
    pc: u16,
    pbr: u8,
}

impl Cpu {
    pub fn tick(&mut self, bus: &mut Bus) {
        use MuOp::*;

        match self.instr {
            Read => {
                bus.ask_read((self.pc, self.pbr).into());
                self.instr = Exec
            }
            Exec => {
                let data = match *bus {
                    Bus::ReplyRead(data) => data,
                    _ => return,
                };
                match data {
                    // NOP
                    0x00 => {
                        self.instr = Read;
                        self.pc = self.pc.wrapping_add(1);
                    }
                    op_code => unimplemented!("0x{:02X}", op_code),
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    /// Run the CPU, starting at byte 0 in the ROM.
    ///
    /// This will exit as soon as the CPU reads past the end of the ROM.
    fn run_cpu(rom: &[u8]) -> Cpu {
        let mut cpu = Cpu::default();
        let mut bus = Bus::default();
        loop {
            cpu.tick(&mut bus);
            match bus {
                Bus::AskRead(at) => {
                    let at = usize::from(at);
                    if at >= rom.len() {
                        break;
                    }
                    bus.reply_read(rom[at]);
                }
                Bus::AskWrite(_, _) => {
                    unimplemented!("ROM write");
                }
                _ => {}
            }
        }
        cpu
    }

    #[test]
    fn test_run_cpu() {
        let cpu = run_cpu(&[0x00]);
        assert_eq!(cpu.pc, 1);
    }
}
