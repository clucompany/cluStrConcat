
#[macro_use]
extern crate cluStrConcat;

fn main() {
	let str = str_concat!("1", "2", "3");
	assert_eq!(str, "123");
	
	const A: &'static str = "A";
	let str = str_concat!(A, A, A, ".");
	assert_eq!(str, "AAA.");
}

