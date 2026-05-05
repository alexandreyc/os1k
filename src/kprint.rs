//! Printing to the SBI-provided debug console.

use core::fmt::Write;

use crate::sbi;

pub struct DebugConsole;

impl Write for DebugConsole {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let len = s.len();
        let base_lo = s.as_ptr().addr();
        let base_hi = 0usize;
        let ret = sbi::call(
            sbi::EID_DEBUG_CONSOLE,
            sbi::FID_CONSOLE_WRITE,
            len,
            base_lo,
            base_hi,
            0,
            0,
            0,
        );
        if ret.is_success() && ret.value == len {
            Ok(())
        } else {
            Err(core::fmt::Error)
        }
    }
}

/// Prints to the SBI-provided debug console.
#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut writer = $crate::kprint::DebugConsole;
        let args = core::format_args!($($arg)*);
        // We just ignore the error if we can't write to the
        // debug console because there isn't much we can do...
        let _ = writer.write_fmt(args);
    }};
}
