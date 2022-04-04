use std::collections::{HashMap};

#[derive(Debug)]
struct SomeData {
	item1 : Vec::<u32>,
	item2 : HashMap::<String,u32>,
}



impl SomeData {

	pub fn new() -> SomeData {
		let v = Vec::<u32>::new();
		let hm = HashMap::<String,u32>::new();
		SomeData {
			item1: v,
			item2: hm
		}
	}

	pub fn push2(&mut self,val : u32) {
		self.item1.push(val);
		self.item1.push(val);
	}

	pub fn add_val(&mut self,key: &str, val : u32) {
		self.item2.insert(key.to_string(),val);
	}
}


fn main() {
    println!("Hello, world!");
	let mut x = SomeData::new();
	x.item1.push(1);
	x.item1.push(2);
	x.item2.insert("Test".to_string(),1);
	x.push2(4);
	x.add_val("test",4);
	let z = x.item2.get(&"test".to_string()).unwrap().clone();
	x.push2(5);
	println!("{:?} {}",x.item1,z);
}
