use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("what is your guess? ");

    let secret_number = rand::thread_rng().gen_range(1..101);

    //println!("the secret number is {}",secret_number);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("that is not a valid number");

        println!("you guessed {}", guess);
        let guess: u32 = guess.trim().parse().expect("please type a number");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}
