

fn main() {
	let mut vect = vec![7,8,9];
	println!("{:?}",vect);
	// push add element add element to end of vec
	vect.push(8);
	println!("{:?}",vect);
	//insert add element to vec with postion
	vect.insert(0, 0);
	println!("{:?}",vect);
	//pop remove the last element from vec
	vect.pop();
	println!("{:?}",vect);


}
