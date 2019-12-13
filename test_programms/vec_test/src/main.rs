fn main() {
	let b: usize = 8;
	let e: u32 = 8; //max 8^10
	let mut v = vec![1;b.pow(e)];
	while v.len() > 1 {
		let mut i = 0;
		let mut k = 0;
		while k < v.len() {
			let mut sum = 0;
			//fasse immer base packete zusammen
			for j in 0..b {
				sum += v[k+j];
			}
			v[i] = sum;
	
			k += b;
			i += 1;
		}
		v.truncate(v.len() / b);
	}
	println!("sum is {}", v[0]);
}
