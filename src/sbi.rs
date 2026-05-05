//! Supervisor Binary Interface.
//!
//! This module contains utilities to interact with the [Supervisor Binary Interface].
//!
//! [Supervisor Binary Interface]: https://docs.riscv.org/reference/sbi/index.html

use core::arch::asm;

pub const EID_DEBUG_CONSOLE: usize = 0x4442434e;
pub const FID_CONSOLE_WRITE: usize = 0x0;

/// Error code returned by an SBI function.
///
/// See the [Standard SBI Errors] for more details.
///
/// [Standard SBI Errors]: https://docs.riscv.org/reference/sbi/binary-encoding.html#table_standard_sbi_errors
#[derive(PartialEq, Eq)]
#[repr(isize)]
pub enum SbiError {
    Success = 0,
    ErrFailed = -1,
    ErrNotSupported = -2,
    ErrInvalidParam = -3,
    ErrDenied = -4,
    ErrInvalidAddress = -5,
    ErrAlreadyAvailable = -6,
    ErrAlreadyStarted = -7,
    ErrAlreadyStopped = -8,
    ErrNoShmem = -9,
    ErrInvalidState = -10,
    ErrBadRange = -11,
    ErrTimeout = -12,
    ErrIo = -13,
    ErrDeniedLocked = -14,
}

impl TryFrom<isize> for SbiError {
    // Return the incorrect value so the caller knows the rejected value.
    type Error = isize;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Success),
            -1 => Ok(Self::ErrFailed),
            -2 => Ok(Self::ErrNotSupported),
            -3 => Ok(Self::ErrInvalidParam),
            -4 => Ok(Self::ErrDenied),
            -5 => Ok(Self::ErrInvalidAddress),
            -6 => Ok(Self::ErrAlreadyAvailable),
            -7 => Ok(Self::ErrAlreadyStarted),
            -8 => Ok(Self::ErrAlreadyStopped),
            -9 => Ok(Self::ErrNoShmem),
            -10 => Ok(Self::ErrInvalidState),
            -11 => Ok(Self::ErrBadRange),
            -12 => Ok(Self::ErrTimeout),
            -13 => Ok(Self::ErrIo),
            -14 => Ok(Self::ErrDeniedLocked),
            _ => Err(value),
        }
    }
}

/// Data returned by an SBI function, composed of an error code and a value.
///
/// See the [SBI Specification] for more details.
///
/// [SBI Specification]: https://docs.riscv.org/reference/sbi/binary-encoding.html#4-1-binary-encoding
pub struct SbiRet {
    pub error: SbiError,
    pub value: usize, // According to the spec, it SHOULD be `usize` but it COULD also be `isize` depending on the function.
}

impl SbiRet {
    pub fn is_success(&self) -> bool {
        self.error == SbiError::Success
    }
}

/// Calls an SBI function.
///
/// # Parameters
/// - `eid`: SBI Extension ID.
/// - `fid`: SBI Function ID.
/// - `arg0..arg5`: Arguments to be passed to the called SBI function.
///
/// See the [SBI Specification] for more details on the calling convention.
///
/// [SBI Specification]: https://docs.riscv.org/reference/sbi/binary-encoding.html#4-1-binary-encoding
#[allow(clippy::too_many_arguments)]
pub fn call(
    eid: usize,
    fid: usize,
    arg0: usize,
    arg1: usize,
    arg2: usize,
    arg3: usize,
    arg4: usize,
    arg5: usize,
) -> SbiRet {
    let error: isize;
    let value: usize;

    unsafe {
        asm!(
            "ecall",
            inlateout("a0") arg0 => error,
            inlateout("a1") arg1 => value,
            in("a2") arg2,
            in("a3") arg3,
            in("a4") arg4,
            in("a5") arg5,
            in("a6") fid,
            in("a7") eid
        );
        // Unlike in the C code, we do not need a "memory clobber" here,
        // because in Rust, the `asm!` macro assumes by default that
        // memory will be read/written.
    }

    // This call to `unwrap` cannot panic otherwise it means
    // we got an error code that's not part of the SBI spec.
    let error = error.try_into().unwrap();

    SbiRet { error, value }
}
