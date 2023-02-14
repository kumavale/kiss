use nix::Result;
use nix::sys::signal::Signal;

/// signal name to Signal
pub fn from_str(s: &str) -> Result<Signal> {
    let uppercase = s.to_uppercase();
    if uppercase.starts_with("SIG") {
        uppercase
    } else {
        format!("SIG{}", uppercase)
    }.parse()
}

#[cfg(test)]
mod tests {
    use super::from_str;
    use nix::sys::signal::Signal;

    #[test]
    fn from_str_test() {
        let tests = [
            ("hup",    "HUP",    "SIGHUP",    Ok(Signal::SIGHUP)),
            ("int",    "INT",    "SIGINT",    Ok(Signal::SIGINT)),
            ("quit",   "QUIT",   "SIGQUIT",   Ok(Signal::SIGQUIT)),
            ("ill",    "ILL",    "SIGILL",    Ok(Signal::SIGILL)),
            ("trap",   "TRAP",   "SIGTRAP",   Ok(Signal::SIGTRAP)),
            ("abrt",   "ABRT",   "SIGABRT",   Ok(Signal::SIGABRT)),
            ("bus",    "BUS",    "SIGBUS",    Ok(Signal::SIGBUS)),
            ("fpe",    "FPE",    "SIGFPE",    Ok(Signal::SIGFPE)),
            ("kill",   "KILL",   "SIGKILL",   Ok(Signal::SIGKILL)),
            ("usr1",   "USR1",   "SIGUSR1",   Ok(Signal::SIGUSR1)),
            ("segv",   "SEGV",   "SIGSEGV",   Ok(Signal::SIGSEGV)),
            ("usr2",   "USR2",   "SIGUSR2",   Ok(Signal::SIGUSR2)),
            ("pipe",   "PIPE",   "SIGPIPE",   Ok(Signal::SIGPIPE)),
            ("alrm",   "ALRM",   "SIGALRM",   Ok(Signal::SIGALRM)),
            ("term",   "TERM",   "SIGTERM",   Ok(Signal::SIGTERM)),
            ("stkflt", "STKFLT", "SIGSTKFLT", Ok(Signal::SIGSTKFLT)),
            ("chld",   "CHLD",   "SIGCHLD",   Ok(Signal::SIGCHLD)),
            ("cont",   "CONT",   "SIGCONT",   Ok(Signal::SIGCONT)),
            ("stop",   "STOP",   "SIGSTOP",   Ok(Signal::SIGSTOP)),
            ("tstp",   "TSTP",   "SIGTSTP",   Ok(Signal::SIGTSTP)),
            ("ttin",   "TTIN",   "SIGTTIN",   Ok(Signal::SIGTTIN)),
            ("ttou",   "TTOU",   "SIGTTOU",   Ok(Signal::SIGTTOU)),
            ("urg",    "URG",    "SIGURG",    Ok(Signal::SIGURG)),
            ("xcpu",   "XCPU",   "SIGXCPU",   Ok(Signal::SIGXCPU)),
            ("xfsz",   "XFSZ",   "SIGXFSZ",   Ok(Signal::SIGXFSZ)),
            ("vtalrm", "VTALRM", "SIGVTALRM", Ok(Signal::SIGVTALRM)),
            ("prof",   "PROF",   "SIGPROF",   Ok(Signal::SIGPROF)),
            ("winch",  "WINCH",  "SIGWINCH",  Ok(Signal::SIGWINCH)),
            ("io",     "IO",     "SIGIO",     Ok(Signal::SIGIO)),
            ("pwr",    "PWR",    "SIGPWR",    Ok(Signal::SIGPWR)),
            ("sys",    "SYS",    "SIGSYS",    Ok(Signal::SIGSYS)),
        ];
        for (sig1, sig2, sig3, expect) in tests {
            assert_eq!(from_str(sig1), expect);
            assert_eq!(from_str(sig2), expect);
            assert_eq!(from_str(sig3), expect);
        }
    }
}
