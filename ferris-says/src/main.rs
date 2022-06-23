extern crate ferris_says;

use ferris_says::say;
use std::io::{ stdout, BufWriter };

fn main(){
    println!( "Cargando: printByStringObj();" );
    printByStringObj();
    println!( "Cargando: printByBString();" );
    printByBString();
}

fn printByStringObj(){
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writter = BufWriter::new( stdout.lock() );
    say( message.as_bytes(), width, &mut writter ).unwrap();
}

fn printByBString() {
    let out = b"Hello fellow Rustaceans!";
    let width = 24;

    let mut writter = BufWriter::new ( stdout() );
    say( out, width, &mut writter ).unwrap();
}