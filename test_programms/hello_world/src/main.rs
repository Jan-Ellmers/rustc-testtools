fn main() {
	let x = 1000000000;
	let y = fibonacci(x);
	println!("y: {:?}", y);
	
}



fn fibonacci(i: usize) -> u128 {
    let mut fib = vec![1,1];
    while fib.len() <= i {
        let len = fib.len();
        fib.push(fib[len-1] + fib[len-2]);
    }
    fib[i]
}
