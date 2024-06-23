fn main() {
    println!("Please enter your name: ");

    let mut name: String = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    name = name.trim().to_string();

    println!("Please enter your age: ");

    let mut age: String = String::new();
    std::io::stdin().read_line(&mut age).unwrap();

    // Get age from console

    // Convert age to number

    let age_int: u8 = age.trim().parse().unwrap();

    println!("Hi, Welcome {}, you are {} years old", name, age_int);
}
