//! When the card had no memory for something.
//!
//! A host texture the driver refused with `E_OUTOFMEMORY` is asked for again
//! once the card may have room (`crate::resource` `Hosted::refused_until`);
//! [`allocation_error`] is how a creation says so. Any other error is a
//! refusal for good.
thread_local! {
    /// This thread was refused memory ([`allocation_error`]) since the last
    /// [`take_out_of_memory`].
    static OUT_OF_MEMORY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// A creation failed with `error`: for want of the card's memory
/// (`E_OUTOFMEMORY`) it is noted for the caller; any other error is the
/// caller's own.
pub(crate) fn allocation_error(error: &windows::core::Error) {
    if error.code() == windows::Win32::Foundation::E_OUTOFMEMORY {
        OUT_OF_MEMORY.set(true);
    }
}

/// Whether this thread was refused memory ([`allocation_error`]) since the
/// last call.
pub(crate) fn take_out_of_memory() -> bool {
    OUT_OF_MEMORY.replace(false)
}
