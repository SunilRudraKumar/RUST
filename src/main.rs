fn main() {
    println!("Hello, world!");
    println!("{}", isEven(2));
    println!("{}", fib(10))
}

fn isEven(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}

fn fib(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if num == first {
        return first;
    }
    if num == second {
        return second;
    }

    for _ in 1..num - 1 {
        let temp = second;
        second = second + first;
        first = temp;
    }

    return second;
}
