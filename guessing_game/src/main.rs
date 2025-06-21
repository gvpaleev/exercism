use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().
        gen_range(1..=100);

    println!("secret_number: {secret_number}");
   
    loop{

        let mut guess = String::new();
        println!("input your guess.");


         io::stdin()
        .read_line(&mut guess)
        .expect("Filed to read line");

       let guess : u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_) => continue,
        };



        println!("You guessed: {guess}");



        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too smail"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }


        }
    }
    

}
