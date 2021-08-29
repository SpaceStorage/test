use std::collections::LinkedList;

fn main() {
	let mut list1 = LinkedList::new();
	list1.push_back('a');
	
	let mut list2 = LinkedList::new();
	list2.push_back('b');
	list2.push_back('c');
	
	list1.append(&mut list2);
	
	let mut iter = list1.iter();

	println!("{:?}", iter)
}
