pub mod host;
pub mod utils;

use host::host_func::*;

#[macro_export]
macro_rules! println {
    () => {
        host::host_func::write_console("\n").unwrap();
    };
    ($($arg:tt)*) => {{
        let x = core::format_args!($($arg)*);
        host::host_func::write_console(&format!("{}\n", x)).unwrap();
    }};
}
