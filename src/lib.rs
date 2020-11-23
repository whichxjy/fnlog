/*!
Logging with function names.
*/

#[macro_export]
macro_rules! fn_trace {
    ($x:expr $(, $($y:expr),+)?) => {
        log::trace!(concat!("[{}] ", $x), stdext::function_name!() $(, $($y),+)?);
    };
}

#[macro_export]
macro_rules! fn_debug {
    ($x:expr $(, $($y:expr),+)?) => {
        log::debug!(concat!("[{}] ", $x), stdext::function_name!() $(, $($y),+)?);
    };
}

#[macro_export]
macro_rules! fn_info {
    ($x:expr $(, $($y:expr),+)?) => {
        log::info!(concat!("[{}] ", $x), stdext::function_name!() $(, $($y),+)?);
    };
}

#[macro_export]
macro_rules! fn_warn {
    ($x:expr $(, $($y:expr),+)?) => {
        log::warn!(concat!("[{}] ", $x), stdext::function_name!() $(, $($y),+)?);
    };
}

#[macro_export]
macro_rules! fn_error {
    ($x:expr $(, $($y:expr),+)?) => {
        log::error!(concat!("[{}] ", $x), stdext::function_name!() $(, $($y),+)?);
    };
}
