
pub extern crate cluConstData;

/// A more advanced version of the concat macro from std, supports constants.
///```
///const A: &'static str = "123";
///const B: &'static str = "456";
///const C: &'static str = "789";
///
///let str = cluStrConcat::str_concat!(A, B, C, ".");
///assert_eq!(str, "123456789.");
///```
#[macro_export]
macro_rules! str_concat {
	[$e:expr] => ($e);
	[$e:expr, $($a:expr),*] => {{
		$crate::cluConstData::const_single_data!(&'static str = $e, $($a),*)
	}};
	
	[@let $e:expr] => ($e);
	[@let $e:expr, $($a:expr),*] => {{
		$crate::cluConstData::let_single_data!(&'static str = $e, $($a),*)
	}};
}

/// A more advanced version of the concat macro from std, supports constants.
#[macro_export]
macro_rules! concat {
	($($tt:tt)*) => (str_concat!($($tt)*));
}



#[cfg(test)]
mod tests {
	#[allow(unused_imports)]
	use super::*;
	
	
	#[test]
	fn one_concat_macros() {
		assert_eq!(str_concat!("."), ".");
	}
	#[test]
	fn two_concat_macros() {
		assert_eq!(str_concat!(".", ".."), "...");
	}
	
	#[test]
	fn full_concat_macros() {
		const A: &'static str = "123";
		const B: &'static str = "456";
		const C: &'static str = "789";
		
		let str = str_concat!(A, B, C, ".");
		assert_eq!(str, "123456789.");
	}
}