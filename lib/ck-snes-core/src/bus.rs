use crate::num::U24;

/// Represents the current status of the bus.
///
/// By sharing mutable access to this status, components can communicate.
///
/// We don't represent the bus as pins directly, but instead as distinct states.
#[derive(Debug, Clone, Copy)]
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

impl Bus {
    /// Set the bus to be asking for a read at a particular address.
    pub fn ask_read(&mut self, addr: U24) {
        *self = Self::AskRead(addr)
    }

    /// Set the bus to be replying to a read request, with the data.
    pub fn reply_read(&mut self, data: u8) {
        // We should only be replying to an actual ask.
        debug_assert!(
            matches!(*self, Self::AskRead(_)),
            "unexpected bus state: {:?}",
            *self
        );
        *self = Self::ReplyRead(data)
    }
}

impl Default for Bus {
    fn default() -> Self {
        // That way uninitialized reads panic on debug.
        // Maybe that causes issues with real games.
        Self::ReplyWrite
    }
}
