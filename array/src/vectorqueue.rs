pub fn vqueue() {
	//let length = 4usize;
	let mut arr = vec![];
	arr.push(12);
	arr.push(120);
	arr.push(45);
	arr.push(50);
	arr.push(6345);
	let item = arr.first();
	println!("{:?}: {}", item, arr.len());
}
