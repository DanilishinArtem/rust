use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop 
    {
        println!("Please, type a number");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error in reading line");
        let guess: u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guess number: {}", guess);
    
        match guess.cmp(&secret_number)
        {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => 
            {
                println!("You win");
                break;
            }
        }
    }
}
