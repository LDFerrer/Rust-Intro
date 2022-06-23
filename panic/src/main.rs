// $Env:RUST_BACKTRACE=1
// $Env:RUST_BACKTRACE=0

fn main() {
    // panic!("Crash in burn!");

    let v = vec![1,2,3];

    v[99];
}
