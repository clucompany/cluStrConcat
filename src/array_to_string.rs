
use alloc::string::ToString;
use alloc::string::String;
use alloc::vec::Vec;
use core::marker::PhantomData;
use core::ops::DerefMut;
use core::ops::Deref;

#[doc(hidden)]
pub trait ArrayString<'a> where <Self::IntoIter as Iterator>::Item : AsRef<str> {
	type Item: AsRef<str>;
	
	fn iter<'i>(&'i self) -> core::slice::Iter<'i, Self::Item>;
	
	type IntoIter: Iterator;
	fn into_iter(self) -> Self::IntoIter;
	
	fn len(&self) -> usize;
}


impl<'a, S, IT, I: 'a, I2> ArrayString<'a> for &'a S 
	where S: ArrayString<'a, Item = I, IntoIter = IT>, IT: Iterator<Item = I2>, I: AsRef<str>, I2: AsRef<str> {
		
	type Item = I;
	
	#[inline(always)]
	fn iter<'i>(&'i self) -> core::slice::Iter<'i, Self::Item> {
		S::iter(self)
	}
	
	type IntoIter = core::slice::Iter<'a, Self::Item>;
	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		S::iter(self)
	}
	
	#[inline(always)]
	fn len(&self) -> usize {
		S::len(self)
	}
}

impl<'a, T, T2, IT> ArrayString<'a> for &'a dyn ArrayString<'a, Item = T, IntoIter = IT> where T: AsRef<str>, IT: Iterator<Item = T2>, T2: AsRef<str> {
	type Item = T;
	
	#[inline(always)]
	fn iter<'i>(&'i self) -> core::slice::Iter<'i, Self::Item> {
		(**self).iter()
	}
	
	type IntoIter = core::slice::Iter<'a, Self::Item>;
	#[inline(always)]
	fn into_iter(self) -> Self::IntoIter {
		(*self).iter()
	}
	
	#[inline(always)]
	fn len(&self) -> usize {
		(**self).len()
	}
}


impl<'a, T> ArrayString<'a> for Vec<T> where T: AsRef<str> {
	type Item = T;
	
	#[inline(always)]
	fn iter<'i>(&'i self) -> core::slice::Iter<'i, T> {
		(**self).iter()
	}
	
	type IntoIter = alloc::vec::IntoIter<T>;
	fn into_iter(self) -> Self::IntoIter {
		IntoIterator::into_iter(self)
	}
	
	#[inline(always)]
	fn len(&self) -> usize {
		(**self).len()
	}
}

impl<'a, T> ArrayString<'a> for &'a [T] where T: AsRef<str> {
	type Item = T;
	
	#[inline(always)]
	fn iter<'i>(&'i self) -> alloc::slice::Iter<'i, Self::Item> {
		(*self).iter()
	}
	
	type IntoIter = alloc::slice::Iter<'a, Self::Item>;
	fn into_iter(self) -> Self::IntoIter {
		IntoIterator::into_iter(self)
	}
	
	#[inline(always)]
	fn len(&self) -> usize {
		(*self).len()
	}
}


pub fn array_to_string<'l, A: ArrayString<'l>>(array: A) -> String where <A::IntoIter as Iterator>::Item :AsRef<str> {
	let array_len = array.len();
	if array_len == 0 {
		return String::new();
	}

	let mut size = 0;
	for a in array.iter() {
		size += a.as_ref().len();
	}
	if size == 0 {
		return String::new();
	}
	
	unsafe {
		__raw_array_to_string(array, size)
	}
}

pub unsafe fn unsafe_array_to_string<'l, A: ArrayString<'l>>(array: A, capacity: usize) -> String where <A::IntoIter as Iterator>::Item :AsRef<str> {
	__raw_array_to_string(array, capacity)
}


#[inline(always)]
unsafe fn __raw_array_to_string<'l, A: ArrayString<'l>>(array: A, capacity: usize) -> String  where <A::IntoIter as Iterator>::Item :AsRef<str> {
	let mut vec = Vec::with_capacity(capacity);
	
	{
		let mut a_ptr = vec.as_mut_ptr();
		
		let mut data_len: usize;
		let mut data_str: &str;
		for data in array.into_iter() {
			data_str = data.as_ref();
			data_len = data_str.len();
			
			
			core::ptr::copy_nonoverlapping(data_str.as_ptr(), a_ptr, data_len);
			
			a_ptr = a_ptr.add(data_len);
		}
	}
	vec.set_len(capacity);
	
	String::from_utf8_unchecked(vec)
}



#[derive(Debug, Clone)]
pub struct MaybeArrayToString<'a, T> where T: ArrayString<'a>, <T::IntoIter as Iterator>::Item :AsRef<str> {
	data: T, 
	_pp: PhantomData<&'a ()>
}

impl<'a, T> MaybeArrayToString<'a, T> where T: ArrayString<'a>, <T::IntoIter as Iterator>::Item :AsRef<str> {
	#[inline]
	pub fn new(a: T) -> Self {
		Self {
			data: a,
			_pp: PhantomData,
		}
	}
	
	#[inline(always)]
	pub fn data(self) -> T {
		self.data
	}
	
	#[inline(always)]
	pub fn as_data(&self) -> &T {
		&self.data	
	}
	
	#[inline(always)]
	pub fn into(self) -> String {
		array_to_string(self.data)
	}
}

impl<'a, T> Deref for MaybeArrayToString<'a, T> where T: ArrayString<'a>, <T::IntoIter as Iterator>::Item :AsRef<str> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl<'a, T> DerefMut for MaybeArrayToString<'a, T> where T: ArrayString<'a>, <T::IntoIter as Iterator>::Item :AsRef<str> {	
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}

impl<'a, T> From<T> for MaybeArrayToString<'a, T> where T: ArrayString<'a>, <T::IntoIter as Iterator>::Item :AsRef<str> {
	#[inline(always)]
	fn from(a: T) -> Self {
		Self::new(a)
	}
}

impl<'a, T> ToString for MaybeArrayToString<'a, T> where T: Copy + ArrayString<'a>, <T::IntoIter as Iterator>::Item :AsRef<str> {
	#[inline(always)]
	fn to_string(&self) -> String {
		array_to_string(self.data)
	}
}



#[cfg(test)]
mod tests {
	#[allow(unused_imports)]
	use super::*;
	
	
	#[test]
	fn one_concat() {
		assert_eq!(array_to_string(&["1"] as &[&str]), "1");
	}
	#[test]
	fn two_concat() {
		assert_eq!(array_to_string(&["1", "2"] as &[&str]), "12");
	}
	
	#[test]
	fn vec_concat() {
		let vec = {
			let mut array = Vec::with_capacity(3);
			array.push("1");
			array.push("2");
			array.push("3");
			array
		};
		
		assert_eq!(array_to_string(&vec), "123");
		assert_eq!(array_to_string(vec), "123");
	}
	
	#[test]
	fn full_concat() {
		let array: &[&str] = &[
			"1", "2345", "67", "89", "", "."
		];
		
		assert_eq!(array_to_string(array), "123456789.");
	}
	
	#[test]
	fn null_concat() {
		assert_eq!(array_to_string(&["", "", "", "", "1"] as &[&str]), "1");
	}
	
	#[test]
	fn string_concat() {
		let array: &[String] = &["1".into(), "2".into(), "3".into()];
		let a_slice: &[&String] = &[array.get(0).unwrap(), array.get(1).unwrap(), array.get(2).unwrap()];
		
		assert_eq!(array_to_string(array), "123");
		assert_eq!(array_to_string(a_slice), "123");
	}
	
	#[test]
	fn check_maybearray_to_string() {
		struct MyData<'l, 'a> {
			data: MaybeArrayToString<'l, &'l [&'a str]>,
		}
		
		impl<'l, 'a> MyData<'l, 'a> {
			#[inline]
			pub fn new(array: &'a [&'a str]) -> Self {
				Self {
					data: array.into(),
				}	
			}
			
			#[inline(always)]
			pub fn as_array(&'l self) -> &'l [&'a str] {
				&self.data
			}
			
			#[inline(always)]
			pub fn into(self) -> String {
				self.data.into()
			}
		}
		
		let data = MyData::new(&["test", "aa", "bb"]);
		assert_eq!(data.as_array(), &["test", "aa", "bb"]);
		assert_eq!(data.into(), "testaabb");
	}
}
