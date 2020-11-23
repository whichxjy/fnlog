# fnlog

[![crates.io](https://img.shields.io/crates/v/fnlog.svg)](https://crates.io/crates/fnlog)

Logging with function names.

## Example

```rust
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
```

Assumes the binary is `main`:

```
$ RUST_LOG=trace ./main
[2020-11-23T21:13:44Z TRACE main] [main::main] trace in main
[2020-11-23T21:13:44Z DEBUG main] [main::main] debug in main
[2020-11-23T21:13:44Z INFO  main] [main::main] info in main
[2020-11-23T21:13:44Z WARN  main] [main::main] warn in main
[2020-11-23T21:13:44Z ERROR main] [main::main] error in main
[2020-11-23T21:13:44Z TRACE main] [main::hello] trace in hello
[2020-11-23T21:13:44Z DEBUG main] [main::hello] debug in hello
[2020-11-23T21:13:44Z INFO  main] [main::hello] info in hello
[2020-11-23T21:13:44Z WARN  main] [main::hello] warn in hello
[2020-11-23T21:13:44Z ERROR main] [main::hello] error in hello
```
