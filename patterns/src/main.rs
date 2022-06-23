fn main() {
    let name = String::from( "ram" );

    println!( "Character at position 4 is: {}", 
        match name.chars().nth(1){
            Some(c) => c.to_string(),
            None => "No character found".to_string(),
        }
    );
}
