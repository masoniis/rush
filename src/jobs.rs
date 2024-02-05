use nix::unistd::Pid;
use std::fmt;

pub struct Job {
    jid: Pid,
    bg: bool,
}

impl Job {
    pub fn new(jid: Pid, bg: bool) -> Job {
        Job { jid, bg }
    }

    pub fn get_jid(self) -> Pid {
        self.jid
    }

    fn clone(&self) -> Job {
        Job {
            jid: self.jid,
            bg: self.bg,
        }
    }
}

pub struct JobList {
    jobs: Vec<Job>,
}

impl JobList {
    pub fn new() -> JobList {
        JobList { jobs: Vec::new() }
    }

    pub fn add_job(&mut self, jid: Pid, bg: bool) {
        self.jobs.push(Job::new(jid, bg));
    }

    pub fn fg_job(&self) -> Option<Job> {
        for job in self.jobs.iter() {
            if !job.bg {
                return Some(job.clone());
            }
        }
        None
    }
}

impl fmt::Display for JobList {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        for job in self.jobs.iter() {
            print!("pid: {}, bg={} ", job.jid, job.bg);
        }
        if self.jobs.len() == 0 {
            return Err(fmt::Error);
        }
        Ok(())
    }
}