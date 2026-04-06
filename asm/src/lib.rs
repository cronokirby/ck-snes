use std::{fmt::Display, io};

#[allow(dead_code)]
#[rustfmt::skip]
#[repr(u8)]
#[derive(Clone, Copy, Debug)]
enum Op {
    ADC, AND, ASL, BCC, BCS, BEQ, BIT, BMI,
    BNE, BPL, BRA, BRK, BRL, BVC, BVS, CLC,
    CLD, CLI, CLV, CMP, COP, CPX, CPY, DEC,
    DEX, DEY, EOR, INC, INX, INY, JML, JMP,
    JSL, JSR, LDA, LDX, LDY, LSR, MVN, MVP,
    NOP, ORA, PEA, PEI, PER, PHA, PHB, PHD,
    PHK, PHP, PHX, PHY, PLA, PLB, PLD, PLP,
    PLX, PLY, REP, ROL, ROR, RTI, RTL, RTS,
    SBC, SEC, SED, SEI, SEP, STA, STP, STX,
    STY, STZ, TAX, TAY, TCD, TCS, TDC, TRB,
    TSB, TSC, TSX, TXA, TXS, TXY, TYA, TYX,
    WAI, WDM, XBA, XCE,
}

#[allow(dead_code)]
#[repr(u8)]
#[derive(Clone, Copy)]
enum AddrMode {
    Absolute,
    AbsoluteIndexedIndirect,
    AbsoluteIndirect,
    AbsoluteLong,
    AbsoluteLongIndexed,
    AbsoluteX,
    AbsoluteY,
    Accumulator,
    BlockMove,
    Direct,
    DirectIndexedIndirect,
    DirectIndirect,
    DirectIndirectLong,
    DirectIndirectLongIndexed,
    DirectIndirectIndexed,
    DirectX,
    DirectY,
    Immediate,
    Implied,
    PcRel,
    PcRelLong,
    Stack,
    StackRelative,
    StackRelativeIndirectIndexed,
}

impl Display for AddrMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            [
                "a", "(a,x)", "(a)", "al", "al,x", "a,x", "a,y", "A", "xyc", "d", "(d,x)", "(d)",
                "[d]", "[d],y", "(d),y", "d,x", "d,y", "#", "i", "r", "rl", "s", "d,s", "(d,s),y"
            ][*self as u8 as usize]
        )
    }
}

#[derive(Clone, Copy)]
struct Instruction {
    pub op: Op,
    pub mode: AddrMode,
}

const INSTRUCTION_TABLE: [Instruction; 256] = {
    use AddrMode::*;
    use Op::*;
    const OPCODES: &[u8; 512] = include_bytes!("opcodes.bin");
    let mut out = [Instruction {
        op: WAI,
        mode: Implied,
    }; 256];
    let mut i = 0;
    while i < 256 {
        out[i] = Instruction {
            op: unsafe { core::mem::transmute(OPCODES[2 * i]) },
            mode: unsafe { core::mem::transmute(OPCODES[2 * i + 1]) },
        };
        i += 1;
    }
    out
};

/// Prints a nice human-readable table of all the operations,
pub fn print_instruction_table() {
    for i in 0..16 {
        for j in 0..16 {
            print!("{:<4?}\t", INSTRUCTION_TABLE[(i << 4) | j].op);
        }
        println!("");
        for j in 0..16 {
            print!("{:<4}\t", INSTRUCTION_TABLE[(i << 4) | j].mode);
        }
        println!("\n");
    }
}

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
