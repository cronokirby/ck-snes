use crate::num::U24;

/// Represents the current status of the bus.
///
/// By sharing mutable access to this status, components can communicate.
///
/// We don't represent the bus as pins directly, but instead as distinct states.
#[derive(Clone, Copy)]
pub enum Bus {
    /// A read request is on the bus.
    AskRead(U24),
    /// A write request is on the bus.
    AskWrite(U24, u8),
    /// The response to a read request is on the bus.
    ReplyRead(u8),
    /// The response to a write request is on the bus.
    ReplyWrite,
}
