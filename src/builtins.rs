use crate::jobs::job_list;
use phf::phf_map; // Compile-time static hashmaps

// Define some functions to be called
fn jobs() {
    job_list()
}

fn test() {
    println!("Test successful");
}

// Create a static map of keys to function pointers
static BUILTINS: phf::Map<&'static str, fn()> = phf_map! {
    "jobs" => jobs,
    "test" => test,
};

pub fn try_builtin(key: &str) -> Option<&fn()> {
    BUILTINS.get(key)
}
