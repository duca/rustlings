// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = square(3);
    let answer2 = square2(3);
    println!("The square of 3 is {}", answer);
    println!("The square of 3 is {}", answer2);
}

fn square(num: i32) -> i32 {
    num * num
}

fn square2(num: i32) -> i32 {
    return num * num;
}
