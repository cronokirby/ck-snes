//! Contains utilities for working with SNES assembly.
//!
//! This is useful in the emulator itself, providing a central location for disassembling
//! ROM data into actual instructions, but it also allows us to easily create test
//! ROMs using mnemonics, instead of bytes.
#![no_std]
use core::iter;

/// A code identifying what kind of operation we have.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    // Do nothing.
    Nop,
    Undefined,
}

/// Identifies how the operand is accessed.
///
/// This mirrors the structure of [Operand].
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum OperandMode {
    /// No operand, or operand hard-coded into the instruction.
    Implicit,
}

/// The operand of an operation, and how it was accessed.
///
/// This is like [OperandMode], but with actual data.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    /// No operand, or hard-coded operand.
    Implicit,
}

impl Operand {
    /// Convert
    fn mode(self) -> OperandMode {
        use Operand as O;
        use OperandMode as OM;

        match self {
            O::Implicit => OM::Implicit,
        }
    }

    fn assemble(self) -> impl Iterator<Item = u8> {
        use Operand::*;

        match self {
            Implicit => iter::empty(),
        }
    }
}

impl From<Operand> for OperandMode {
    fn from(value: Operand) -> Self {
        value.mode()
    }
}

/// Like [Op], but without the actual data.
///
/// This is useful in an emulator, because often we need to figure out how
/// to fetch the actual operand based on the mode, having only read one byte.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct OpHeader {
    /// What operation is this?
    pub code: OpCode,
    /// How is the operand accessed?
    pub mode: OperandMode,
}

impl OpHeader {
    /// Assembler an op header into a single code.
    pub fn assemble(self) -> u8 {
        use OpCode::*;

        match (self.code, self.mode) {
            (Nop, _) => 0x00,
            (Undefined, _) => 0xFF,
        }
    }
}

/// An operation, along with the data needed to perform it.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Op {
    /// What operation is this?
    pub code: OpCode,
    /// What operand does the operation use?
    pub operand: Operand,
}

impl Op {
    fn assemble_one(&self) -> impl Iterator<Item = u8> {
        iter::once(self.header().assemble()).chain(self.operand.assemble())
    }
}

impl Op {
    /// Convert this operation into its header.
    pub fn header(self) -> OpHeader {
        OpHeader {
            code: self.code,
            mode: self.operand.into(),
        }
    }

    /// Assemble a sequence of operations into ROM data.
    pub fn assemble(ops: &[Self]) -> impl Iterator<Item = u8> {
        ops.iter().flat_map(Self::assemble_one)
    }
}

impl From<Op> for OpHeader {
    fn from(value: Op) -> Self {
        value.header()
    }
}
