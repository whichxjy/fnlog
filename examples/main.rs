use fnlog::{fn_debug, fn_error, fn_info, fn_trace, fn_warn};

fn hello() {
    fn_trace!("trace in hello");
    fn_debug!("debug in hello");
    fn_info!("info in hello");
    fn_warn!("warn in hello");
    fn_error!("error in hello");
}

fn main() {
    env_logger::init();
    fn_trace!("trace in main");
    fn_debug!("debug in main");
    fn_info!("info in main");
    fn_warn!("warn in main");
    fn_error!("error in main");
    hello();
}
