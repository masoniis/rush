// use nix::sys::wait::wait;
// use nix::unistd::ForkResult::{Child, Parent};
// use nix::unistd::{execve, fork, getpid};
use nix::sys::signal;
use std::io;
use std::{thread, time};

fn main() {
    let sigint = signal::SigAction::new(
        signal::SigHandler::Handler(handle_sigint),
        signal::SaFlags::empty(),
        signal::SigSet::empty(),
    );

    loop {
        println!("Doing work");
        // let mut input_text = String::new();
        //
        // io::stdin()
        //     .read_line(&mut input_text)
        //     .expect("failed to read from stdin");
    }
    // let pid = fork();
    //
    // println!("Hello from process with pid: {}", pid);

    // match pid.expect("Fork Failed: Unable to create child process!") {
    //     Child => println!(
    //         "Hello from child process with pid: {} and parent pid:{}",
    //         getpid(),
    //         getppid()
    //     ),
    //     Parent { child } => {
    //         wait().expect("Wait Failed: Unable to wait for child process!");
    //         println!(
    //             "Hello from parent process with pid: {} and child pid:{}",
    //             getpid(),
    //             child
    //         );
    //     }
    // }
}

extern "C" fn handle_sigint(_: i32) {
    println!("Received SIGINT");
}
