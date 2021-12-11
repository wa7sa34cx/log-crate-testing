mod logging;

// extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn main() {
    log_init!();

    trace!("tracing...");
    info!("such information");
    warn!("o_O");
    error!("much error");

    nested::run();

    info!(env!("CARGO_PKG_NAME"));
}

mod nested {
    pub fn run() {
        trace!("qwerty");
    }
}
