fn foo(_x:&'static str) -> &'static str{
    _x
}

fn main() {
    println!("Hello, {}!",foo("world"));
}
