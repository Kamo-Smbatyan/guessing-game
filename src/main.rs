use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is:{secret_number}");
    println!("Please input your guess.");
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");
    loop{
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess:String= match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed:{}", guess);
        let guess: u32 = guess.trim().parse().expect("Plz type a number");
        match guess.cmp(&secret_number){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal =>{ println!("You Win"); break;},
        }
    }
}
