fn main() {
    println!("Please enter your name: ");

    let mut name: String = String::new();

    std::io::stdin().read_line(&mut name).unwrap();

    println!("Hi, Welcome {}", name);
}
