 use std::io;

fn main() {
    // programme wont compile without semicolon
    println!("please input your guess: ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Failed to read input");

    println!("Your guess is : {}", guess);
    let mut foo = 5;
    foo = foo + 10;
    println!("The result of foo is: {}", foo);
}

// cargo with --bin arg allows makes an executable file ie binary
// cargo build build a project
// cargo run  execute a project
// cargo check checks in case of errors
