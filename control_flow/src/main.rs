fn main() {
    let f_temp = c_to_f(30);
    println!("{}", f_temp);
    let c_temp = f_to_c(32);
    println!("{}", c_temp);
    let fib = fibonacci(5);
    println!("{}", fib);
}

fn c_to_f(temp: i32) -> i32 {
    (temp * 9 / 5) + 32
}

fn f_to_c(temp: i32) -> i32 {
    (temp - 32) * 5 / 9
}

fn fibonacci(n: i32) -> i32 {
    match n {
        1 => 0,
        2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
