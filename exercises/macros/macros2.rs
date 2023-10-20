// macros2.rs
//
// Execute `rustlings hint macros2` or use the `hint` watch subcommand for a
// hint.


//#[macro_use]
//#[warn(unused_attributes)]
#[macro_use]
#[warn(unused_attributes)]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
