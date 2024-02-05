use phf::phf_map; // Compile-time static hashmaps

// Define some functions to be called
fn foo() {
    println!("This is foo");
}

fn bar() {
    println!("This is bar");
}

fn baz() {
    println!("This is baz");
}

// Create a static map of keys to function pointers
static BUILTINS: phf::Map<&'static str, fn()> = phf_map! {
    "foo" => foo,
    "bar" => bar,
    "baz" => baz,
};

pub fn is_builtin(key: &str) -> Option<fn()> {
    BUILTINS.get(key).cloned()
}
