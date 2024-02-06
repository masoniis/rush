use nix::sys::signal;
use nix::sys::wait::{waitpid, WaitPidFlag};
use nix::unistd::{execvp, fork, Pid};
use std::ffi::CString;
use std::io::{self, Write};

use crate::jobs::add_job;
// use modern_rust::signal::SigHandler; <- Look into this to see if better than unsafe nix::sys:signal

mod builtins;
mod jobs;

fn main() {
    let _sigint = signal::SigAction::new(
        signal::SigHandler::Handler(handle_sigint),
        signal::SaFlags::empty(),
        signal::SigSet::empty(),
    );

    let _sigchld = signal::SigAction::new(
        signal::SigHandler::Handler(handle_sigchld),
        signal::SaFlags::empty(),
        signal::SigSet::empty(),
    );

    // Register the signal for action
    unsafe {
        signal::sigaction(signal::SIGINT, &_sigint).unwrap();
        signal::sigaction(signal::SIGCHLD, &_sigchld).unwrap();
    }

    let mut jobs = jobs::JobList::new(); // Initialize the joblist

    // Enter the shell perma-loop
    loop {
        print!("rsh> ");
        io::stdout()
            .flush()
            .expect("Failed to properly flush stdout"); // Ensure that print is immediate

        let mut input = String::new();
        let mut bg = false;

        io::stdin()
            .read_line(&mut input)
            .expect("!! failed to read input");
        input = input.trim().to_string(); // Remove trailing whitespace

        if input.ends_with("&") {
            input = input[0..input.len() - 2].to_string(); // Remove the & from the input
            bg = true;
        }

        // let input_split = input.split_whitespace().collect::<Vec<&str>>();
        let input_split = input.split_whitespace(); //Iterator over input

        if (fork().unwrap()).is_child() {
            jobs.add_job(nix::unistd::getpid(), bg); // Add the job to the JobList

            let inputs: Vec<CString> = input_split // convert the iterator into a vector
                .map(|s| CString::new(s).unwrap()) // map each &str to a CString
                .collect();

            let cmd = CString::new(inputs[0].clone()).unwrap();
            let args = inputs
                .iter() // iterate over references
                .map(|x| x.as_ref()) // convert each CString to a &CStr
                .collect::<Vec<_>>(); // collect the results into a vector

            match builtins::is_builtin(cmd.to_str().unwrap()) {
                Some(func) => {
                    // If builtin is found
                    func();
                    return; // Kill child after running builtin
                }
                None => {
                    // No builtin, execute command
                    match execvp(&cmd, &args) {
                        Ok(_) => (),
                        Err(_errmsg) => {
                            println!("rsh: command not found -> {}", cmd.into_string().unwrap(),);
                            return; // Kill child with failed command
                        }
                    }
                }
            }
        } else {
            if bg {
                continue;
            } else {
                wait_fg(&jobs); // Idle main thread until child process is done
            }
        }
        println!(); // Add an extra line after each command
    }
}

// Idle main thread until child foreground process is done
fn wait_fg(jobs: &jobs::JobList) {
    let jid: Option<Pid> = match jobs.fg_job() {
        Some(job) => Some(job.get_jid()),
        None => None,
    };

    let _res = nix::sys::wait::waitpid(jid, None);
    // println!("Foreground process done, id: {:?}", res);
}

extern "C" fn handle_sigint(signal: i32) {
    println!("Received SIGINT, {}", signal);
    unsafe {
        nix::libc::exit(1);
    }
}

extern "C" fn handle_sigchld(_: i32) {
    loop {
        match waitpid(Pid::from_raw(-1), Some(WaitPidFlag::WNOHANG)) {
            Ok(_e) => {
                // println!("Process {} terminated.", e.pid().unwrap());
            }
            Err(_e) => {
                // println!("Exit reap: {:?}", e);
                break; // No child left to terminate
            }
        }
    }
}
