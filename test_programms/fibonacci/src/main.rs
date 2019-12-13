fn main() {
    for i in 0..18600 {
        println!("fib({})={}", i, fibonacci(i));    
    }
    
}

fn fibonacci(i: usize) -> u64 {
    let mut fib = vec![1,1];
    while fib.len() <= i {
        let len = fib.len();
        fib.push((fib[len-1] + fib[len-2]) % 100000);
    }
    fib[i]
}
