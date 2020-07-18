use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn run() {
    // programm wont compile without semicolon

// cargo with --bin arg allows makes an executable file ie binary
// cargo build build a project
// cargo run  execute a project
// cargo check checks in case of errors


    let secret_number = rand::thread_rng().gen_range(5, 20);

    loop {
        println!("please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        let guess: i32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_reason) => continue,
        };

        println!("Your guess is : {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Gotcha");
                break;
            }
        };
    }
}
