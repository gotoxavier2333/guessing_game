use rand::Rng;
use std::cmp::Ordering;
use std::io;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

macro_rules! my_macro{
    () => {

    };
}

fn main() {
    guess_game();
    const_println();
    let_mut_println();
    shadowing();
    data_types();
    tuple();
    arrays();
    another_function(6);
    println!("This plus_one fn is {}", plus_one(7));
    control();
    loop_counter();
    loop_break();
    my_macro!();
}

fn let_mut_println() {
    let x = 6;
    /* x = 7; */
    println!("This var x is {x}");
    let mut y = 9;
    println!("This var y is {y}");
    y = 6;
    println!("This var y is {y}");
}

fn const_println() {
    println!("This const is {THREE_HOURS_IN_SECONDS}");
}

fn data_types() {
    let guess: u32 = "42".parse().expect("This let is not a number");
    /* let guess2 = "42".parse().expect("This let is not number"); */
    println!("This guess is {guess}");
    /* println!("This guess2 is {guess2}"); */
}

fn another_function(x: i32) {
    let x = x * 2;
    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn control() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn loop_counter() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

}

fn loop_break() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

/**
 * 这里使用shadow非常有用
 * 在变量名称不变的情况下，用户的输入可以从字符串（json格式）转变成数字或者枚举，
 * 而不需要重新写一个新的变量名称
 */
fn shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("This shadow x is {x}");
    }
    println!("This shadow x is {x}");
}

fn tuple() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y ,z) = tup;
    println!("tup's x is {x}");
    println!("tup's y is {y}");
    println!("tup's z is {z}");
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    println!("a is {a}");
    println!("b is {b}");
    println!("c is {c}");
}

fn arrays() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

}

fn guess_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=2);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}