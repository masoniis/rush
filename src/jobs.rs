use nix::unistd::Pid;
use std::cell::RefCell;
use std::fmt;

thread_local! { // Mutable thread local static initialization
    // static JOB_LIST: JobList = JobList { jobs: Vec::new() };
    pub static JOB_LIST: RefCell<JobList> = RefCell::new(JobList::new());
}

pub fn add_job(jid: Pid, bg: bool) {
    JOB_LIST.with(|x| x.borrow_mut().add_job(jid, bg));
    // JOB_LIST.with(|x| x.add_job(jid, bg));
}

pub fn fg_job() -> Option<Job> {
    let job = JOB_LIST.with(|x| x.borrow().fg_job());
    match job {
        Some(job) => Some(job),
        None => None,
    }
}

pub fn job_list() {
    let jobs = JOB_LIST.with(|x| x.borrow().clone());
    println!("{}", jobs)
}

#[derive(Clone)]
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

#[derive(Clone)]
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
