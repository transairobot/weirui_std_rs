pub mod host;
pub mod utils;
#[macro_export]
macro_rules! println {
    () => {
        weirui_std_rs::host::host_func::write_console("\n").unwrap();
    };
    ($($arg:tt)*) => {{
        let x = core::format_args!($($arg)*);
        weirui_std_rs::host::host_func::write_console(&format!("{}\n", x)).unwrap();
    }};
}
