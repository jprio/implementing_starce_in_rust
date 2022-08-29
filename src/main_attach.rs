mod system_call_names;

use nix::sys::ptrace;
use nix::sys::wait::wait;
use nix::unistd::Pid;

fn main() {
    let pid = <PID_YOU_WANT_TO_TRACE>;

    ptrace::attach(Pid::from_raw(pid))
            .map_err(|e| format!("Failed to ptrace attach {} ({})", pid, e))
            .unwrap();

    run_tracer(Pid::from_raw(pid)) 
}

fn run_tracer(child: Pid) {
    loop {
        wait().unwrap();

        match ptrace::getregs(child) {
            Ok(x) => println!(
                "Syscall name: {:?}",
                system_call_names::SYSTEM_CALL_NAMES[(x.orig_rax) as usize]
            ),
            Err(_) => break,
        };

        match ptrace::syscall(child, None) {
            Ok(_) => continue,
            Err(_) => break,
        }
    }
}