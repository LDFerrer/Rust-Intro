use std::fs::File;
fn main() {
    let f = File::open( "hello.txt" ).expect("We don't have the file yet!");

    // let foo = match f {
    //     Ok( file ) => file, 
    //     Err( error ) => {
    //         panic!( "File wasn't found!" );
    //     },
    // };
}
