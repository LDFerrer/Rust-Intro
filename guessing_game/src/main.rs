extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    println!( "Welcome to the guessing game!" );

    let secret_number = rand::thread_rng().gen_range( 1, 101 );
    //  Ocultamos esta línea para no saber el número a adivinar.
    println!( "Secret number is = {}", secret_number );

    loop{

        println!("Please import your guess...");

        let mut guess = String::new();

        io::stdin().read_line( &mut guess )
            .expect("Failed to read line!");

        println!( "You guessed: {}", guess );

        let guess: u32 = guess.trim().parse().expect("Type a number");

        match guess.cmp( &secret_number ) {
            Ordering::Less => println!( "Too small!" ), 
            Ordering::Greater => println!( "Too big!" ),
            Ordering::Equal => {
                println!( "You win!" );
                break;
            }
        }
    }
}
