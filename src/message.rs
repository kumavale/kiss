/// Do nothing
pub fn do_nothing() {
    eprintln!("nothing happened...");
}

/// Expect signal
pub fn expect_signal() {
    eprintln!("kiss must specify a signal");
    help();
}

/// Not enough argument
pub fn not_enough_arg(opt: &str) {
    eprintln!("kiss {opt}: expect signal name");
    help();
}

/// Invalid options
pub fn invalid_option(opt: &str) {
    eprintln!("invalid option: `{opt}`");
    help();
}

/// List signal names defined by this OS
pub fn list() {
    use nix::sys::signal::Signal;
    for sig in Signal::iterator() {
        print!("{} ", &sig.as_str()[3..]);
    }
    println!();
}

/// Show help
pub fn help() {
    println!("kiss: send a signal to a process

Usage: kiss <OPTIONS> [PID]...

Options:
  -s, --signal <SIGNAL>  Specify the signal
  -l, --list             List signal names
  -h, --help             Show help
  -v, --version          Show version
");
}

/// Show version
pub fn version() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("kiss version {}", VERSION);
}
