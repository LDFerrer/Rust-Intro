enum IpAddKind {
    V4(String),
    V6(String)
}

enum Message {
    Quit,
    Move { x:i32, y:i32 },
    Write( String ),
    ChangeColor( i32, i32, i32 ),
}

impl Message {
    fn call( &self ){
        println!( "I'm inside Call!" );
    }
}

fn main() {
    let four = IpAddKind::V4;
    let six = IpAddKind::V6;

    let home = IpAddKind::V4( String::from("127.0.0.1") );
    let loopback = IpAddKind::V6( String::from("::1") );

    let m = Message::Write( String::from("Hello") );
    m.call();
}
