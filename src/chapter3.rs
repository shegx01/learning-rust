fn run3() {
    // control flow

    let arr: [i32; 7]= [5,3,4,5,6,8,9];

    for elem in arr.iter().filter(|x| *x % 2 != 0) {
        println!("The current element is {}", elem)
    }

    // pattern

    // let decision = 5;


    // // short circuiting doesnt work

    // if decision != 0 {
    //     println!("Hi 5")
    // }


    // match decision {
    //     true => println!("Good job"),
    //     false => println!("Huh")
    // }















    // // if statement

    // let a = false;

    // let result = if a == true {
    //     println!("Hoo Hey");
    //     Some(5)
    // } else {
    //     println!("Huh huh");
    //     None
    // };



    // functioons

    //    println!("The result of add is {}", add(23, 34))
    // data types

    // compound types

    // Arrays are fixed length

    // let my_arr: [i32; 3] = [2, 4, 5];

    // error on comma instead of semicolon
    // let my_arr: [i32, 3] = [2, 4, 5];

    // let [first, second, third] = my_arr;

    // println!("Can destructure arrays too {} {} {}", first, second, third);
    // println!("Can access through index {} {} {}", my_arr[0], my_arr[1], my_arr[2]);

    // tuple

    // with destructuring
    // let is important like in JS
    // kinda thrown away for let requirement but its like vardeclaration
    // let tup = (3, 6, "Welcome", [232]);

    // let (round, greetings, round2, [my]) = tup;

    // println!("Exploring {} {} {} {}", round, greetings, round2, my);
    // println!("Exploring {} {} {}", tup.0, tup.1, tup.2);

    // accessing with index

    // // character types

    // let char = 'j';

    // booleans

    // let  a = true;

    // let a  = !a;

    // println!("A is now {}", a);

    // // arithmetics
    // // like in maintream languages
    // let sum = 5 + 10;
    // // subtraction
    // let difference = 95.5 - 4.3;
    // // multiplication
    // let product = 4 * 30;
    // // division
    // let quotient = 56.7 / 32.2;
    // // remainder
    // let remainder = 43 % 5;

    // // floating point numbers
    // let f: f32 = 5.0;
    // println!("this result is a float {}", f)
    // // default for float is f64

    // scalar type
    // sign with sign ie stored in memory as complementary
    // unsign with unsign ie stored as it is
    // can only be types in the specified table

    // let result: i32 = "42".parse().expect("Parsing failed, Not a number");

    // println!("The result is: {}", result);

    // // variables are immutable
    // let x: i32 = 5;
    // // x = 43; error
    // println!("the value of x is: {}", x);

    // shadowing variables
    // let space = "  ";
    // let space = space.len();
    // println!("The size is: {}", space)
    // io::stdin().read_line()
}

// functions

// dont add semicolon to expresssion returning from a function
// fn add(left: i32, right: i32) -> i32 {
//     println!("The values of params are {} and {}", left, right);
//     left + right
// }
