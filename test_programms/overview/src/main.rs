fn main() {
	println!("Overview");
    let x = vec![1,2,3,4,5,6,7,8,9];
    let y = [1,2,3,4];
    vectest(&x);
    slicetest(&x[..]);
    arraytest(&y);
}
 
fn vectest(x: &Vec<usize>) {
	let index = 0;
    let y = x[index];
	println!("Overview: Vec: {}", y);
}

fn slicetest(x: &[usize]) {
	let index = 0;
    let y = x[index];
	println!("Overview: Slice: {}", y);
}

fn arraytest(x: &[usize; 4]) {
	let index = 0;
    let y = x[index];
	println!("Overview: Array: {}", y);
}
