// test4.rs
// This test covers the sections:
// - Modules
// - Macros

// Write a macro that passes the test! No hints this time, you can do it!

macro_rules! my_macro {
    () => {
        String::from(format!("Ooops!"));
    };
    ($v:expr) => {
        String::from(format!("Hello {}", $v));
    };
}

fn main() {
    if my_macro!("world!") != String::from("Hello world!") {
        panic!("Oh no! Wrong output!");
    }
}
