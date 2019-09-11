

fn main() {
	let data: &[&str] = &["123", "456", "789", "000"];
	
	let string = cluStrConcat::array_string_concat(data);
	println!("{:?}, capacity: {}", string, string.capacity());
	println!("old {:?}", data);
	
	println!("{:?}", string.as_bytes());
}
