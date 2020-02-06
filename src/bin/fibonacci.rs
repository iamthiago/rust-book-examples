
fn main() {
    for n in 1..11 {
        println!("fibonacci of {} is {}", n, fibonacci(n));
    }
}

fn fibonacci(x: u32) -> u32 {
    match x {
        0 => 1,
        1 => 1,
        _ => fibonacci(x - 1) + fibonacci(x - 2),
    }
}