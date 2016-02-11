fn main() {
    println!("\
I am at a rough estimate thirty billion times more intelligent than you. \
Let me give you an example. Think of a number, any number.");
    let mut answer = String::new();
    std::io::stdin().read_line(&mut answer).unwrap();
    println!("Wrong. You see?");
}
