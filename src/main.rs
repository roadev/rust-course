fn main() {
    let number_1 = 120;
    let number_2 = 321;

    let sum = number_1 + number_2;

    loop {
        println!("Please enter de result of {} + {}: ", number_1, number_2);

        let mut sum_user = String::new();
        std::io::stdin().read_line(&mut sum_user).unwrap();

        let sum_user_int : i32 = sum_user.trim().parse().unwrap();

        if sum_user_int == sum {
            println!("Nice! the result {} is correct", sum);
            break;
        } else {
            println!("The result {} is incorrect, try again", sum_user_int);
        }
    }

}