use rand::Rng;

fn main() {
    let x = rand::thread_rng().gen_range::<f64>(1.0, 35.0);
    println!("{} celsius is equal to {} fahrenheit", x, celsius_to_fahrenheit(x));
    println!("{} fahrenheit is equal to {} celsius", x, fahrenheit_to_celsius(x));
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0/5.0) + 32.0
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0/9.0
}