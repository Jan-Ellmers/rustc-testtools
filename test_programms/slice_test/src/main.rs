fn main() {
	let b: usize = 8;
	let e: u32 = 8; //max 8^10
	let mut v = vec![1;b.pow(e)];
	let mut s = &mut v[..];
	while s.len() > 1 {
		let mut i = 0;
		let mut k = 0;
		while k < s.len() {
			let mut sum = 0;
			//fasse immer base packete zusammen
			for j in 0..b {
				sum += s[k+j];
			}
			s[i] = sum;
	
			k += b;
			i += 1;
		}
		let len = s.len();
		s = &mut s[0..(len/b)];
	}
	println!("sum is {}", s[0]);
}
