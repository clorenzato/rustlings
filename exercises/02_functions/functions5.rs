// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let value:i32 = 3;
    let answer = square(value);

    println!("The square of {} is {}", value, answer);
}

fn square(num: i32) -> i32 {
    num * num
}
