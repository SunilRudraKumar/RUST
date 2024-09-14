fn main() {
    println!("Hello, world!");
    println!("{}", isEven(2))
}

fn isEven(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}
