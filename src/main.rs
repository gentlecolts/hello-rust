fn foo(_x: &'static str) -> &'static str {
	_x
}

fn fib(n: i32) -> i32{
	if n<=0{
		return fib(n+2) - fib(n+1);
	} else 	if n<=2 {
		return 1;
	}
	fib(n-1)+ fib(n-2)
}

fn main() {
	println!("Hello, {}!", foo("world"));

	let n=-7;
	println!("{}th fibbonacci: {}",n,fib(n))
}
