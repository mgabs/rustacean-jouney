#![allow(dead_code)]
use duct::cmd;
/// TODO:
/// refactor to log errors into a file
/// take config file for commands
/// parse configs
///
/// send notification helper
fn send_notif(text: &str) -> bool {
    let stdout = cmd!("notify-send", text).run();
    stdout.is_ok()
}

/// execute command and return boolean of success/fail
fn start_process(cmd: &str, args: &str) -> bool {
    let stdout = cmd!(cmd, args).stdout_null().run();
    stdout.is_ok()
}

/// check if process is running
/// returns boolean
/// true: is running
/// false : is not running
fn check_process(process: &str) -> bool {
    let check = cmd!("pgrep", process).stdout_null().run();
    check.is_ok()
}

/// restart process
fn restart(process: &str, args: &str) -> bool {
    if check_process(process) {
        println!("process is running: {}", &process);

        let kill_process = cmd!("pkill", process).stdout_null().run();
        if kill_process.is_ok() {
            println!("killed process: {}", &process);
            return start_process(process, args);
        } else {
            println!("failed to kill process: {}", &process);
            return false;
        }
    } else {
        println!("process is not running: {}", &process);
        println!("starting process: {}", &process);

        return start_process(process, args);
    }
}

fn main() {
    // println!("{}", send_notif("Hello from Rust!"));
    // println!("{}", start_process("echo", "hello"));
    //
    let mut process = "conky";
    println!("{:?}", check_process(process));
    process = "awesome";
    println!("{:?}", check_process(process));

    process = "conky";

    println!("{:?}", restart(process, "&"));
}
