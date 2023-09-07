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

fn printy_thingy<T>(n: &T) where T:std::fmt::Display{
	println!("wow, thats {}",n);
}

fn closure_test(){
	let x=String::from("lolee");

	let test1=|| printy_thingy(&x);
	test1();
	println!("reusing x:{}",x);

	let test2=move || printy_thingy(&x);
	test2();

	//this isn't allowed
	//println!("reusing x after move:{}",x);
}

fn main() {
	println!("Hello, {}!", foo("world"));

	let n=-7;
	println!("{}th fibbonacci: {}",n,fib(n));

	closure_test();
}
