// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let some_value: Option<i32> = Some(42);

    if let Some(value) = some_value {
        println!("The value is {}", value);
    } else {
        println!("No value");
    }
}
