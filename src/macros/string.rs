

use std::marker::PhantomData;
use std::ops::DerefMut;
use std::ops::Deref;
use std::mem::MaybeUninit;

pub trait ArrayStringConcat<'a, T>/* where T: AsRef<str>, <Self::IntoIter as Iterator>::Item :AsRef<str>*/ {
	fn iter<'i>(&'i self) -> std::slice::Iter<'i, T>;
	
	type IntoIter: Iterator;
	fn into_iter(self) -> Self::IntoIter;
	
	fn len(&self) -> usize;
}


impl<'a, T, IT> ArrayStringConcat<'a, T> for &'a dyn ArrayStringConcat<'a, T, IntoIter = IT> where IT: Iterator {
	fn iter<'i>(&'i self) -> std::slice::Iter<'i, T> {
		(**self).iter()
	}
	
	type IntoIter = std::slice::Iter<'a, T>;
	fn into_iter(self) -> Self::IntoIter {
		(*self).iter()
	}
	
	fn len(&self) -> usize {
		(**self).len()
	}
}


impl<'a, T> ArrayStringConcat<'a, T> for Vec<T> {
	#[inline(always)]
	fn iter<'i>(&'i self) -> std::slice::Iter<'i, T> {
		(**self).iter()
	}
	
	type IntoIter = std::vec::IntoIter<T>;
	fn into_iter(self) -> Self::IntoIter {
		IntoIterator::into_iter(self)	
	}
	
	#[inline(always)]
	fn len(&self) -> usize {
		(**self).len()
	}
}

impl<'a, T> ArrayStringConcat<'a, T> for &'a [T] {
	#[inline(always)]
	fn iter<'i>(&'i self) -> std::slice::Iter<'i, T> {
		(*self).iter()
	}
	
	type IntoIter = std::slice::Iter<'a, T>;
	fn into_iter(self) -> Self::IntoIter {
		IntoIterator::into_iter(self)
	}
	
	#[inline(always)]
	fn len(&self) -> usize {
		(*self).len()
	}
}



pub struct ArrayStringConcatIter<'a, Data, T>(ShadowLifeTime<'a, Data>, std::slice::Iter<'a, T>);

impl<'a, Data, T> Iterator for ArrayStringConcatIter<'a, Data, T> {
	type Item = &'a T;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item> {
		self.1.next()
	}
}

impl<'a, Data, T> Deref for ArrayStringConcatIter<'a, Data, T> {
	type Target = std::slice::Iter<'a, T>;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.1
	}
}

impl<'a, Data, T> DerefMut for ArrayStringConcatIter<'a, Data, T> {
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.1
	}
}




pub struct ShadowLifeTime<'a, T: 'a>(T, PhantomData<&'a T>);
		
impl<'a, T> ShadowLifeTime<'a, T> {
	#[inline(always)]
	pub const fn new(data: T) -> Self {
		ShadowLifeTime(data, PhantomData)
	}
}

impl<'a, T> ShadowLifeTime<'a, T> {
	#[inline(always)]
	pub unsafe fn new_lifetime<'n: 'a>(&'a self) -> &'n T {
		std::mem::transmute(self)
	}
	
	#[inline(always)]
	pub unsafe fn new_mut_lifetime<'n: 'a>(&'a mut self) -> &'n mut T {
		std::mem::transmute(self)
	}
}

impl<'a, T> Deref for ShadowLifeTime<'a, T> {
	type Target = T;
	
	#[inline(always)]
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<'a, T> DerefMut for ShadowLifeTime<'a, T> {	
	#[inline(always)]
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}





impl<'a, T> ArrayStringConcat<'a, T> for [T] where Self: Sized + 'a {
	#[inline(always)]
	fn iter<'i>(&'i self) -> std::slice::Iter<'i, T> {
		<[T]>::iter(self)
	}
	
	type IntoIter = ArrayStringConcatIter<'a, Self, T>;
	fn into_iter(self) -> Self::IntoIter {
		
		struct __ArrayStringConcatIter<'a, Data: 'a, T: 'a>(ShadowLifeTime<'a, Data>, MaybeUninit<std::slice::Iter<'a, T>>, PhantomData<&'a Data>);
		
		impl<'a, Data, T> __ArrayStringConcatIter<'a, Data, T> {
			#[inline(always)]
			const fn new(data: Data) -> Self {
				__ArrayStringConcatIter(ShadowLifeTime::new(data), MaybeUninit::uninit(), PhantomData)
			}
		}
		
		let mut data: __ArrayStringConcatIter<Self, T> = __ArrayStringConcatIter::new(self);
		//data.1 = MaybeUninit::new(unsafe { data.0.new_lifetime() }.iter());
		data.1 = MaybeUninit::new(unsafe { data.0.new_lifetime() }.iter());
		
		ArrayStringConcatIter(data.0, unsafe { data.1.assume_init() })
	}
	
	#[inline(always)]
	fn len(&self) -> usize {
		<[T]>::len(self)
	}
}













pub fn array_string_concat<'l, T: 'l, A: 'l + ArrayStringConcat<'l, T>>(array: A) -> String where T: AsRef<str>, <A::IntoIter as Iterator>::Item :AsRef<str> {
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
		raw_array_string_concat(array, size)
	}
}

pub unsafe fn unsafe_array_string_concat<'l, T, A: ArrayStringConcat<'l, T>>(array: A, capacity: usize) -> String where T: AsRef<str>, <A::IntoIter as Iterator>::Item :AsRef<str> {
	raw_array_string_concat(array, capacity)
}


#[inline(always)]
unsafe fn raw_array_string_concat<'l, T, A: ArrayStringConcat<'l, T>>(array: A, capacity: usize) -> String  where T: AsRef<str>, <A::IntoIter as Iterator>::Item :AsRef<str> {
	let mut vec = Vec::with_capacity(capacity);
	
	{
		let mut a_ptr = vec.as_mut_ptr();
		
		let mut data_len: usize;
		let mut data_str: &str;
		for data in array.into_iter() {
			data_str = data.as_ref();
			data_len = data_str.len();
			
			
			std::ptr::copy_nonoverlapping(data_str.as_ptr(), a_ptr, data_len);
			
			//a_ptr = a_ptr.offset(data_len as _);
			a_ptr = a_ptr.add(data_len);
		}
	}
	vec.set_len(capacity);
	
	String::from_utf8_unchecked(vec)
}










/*
pub trait ArrayStringConcat<'a> where <Self::IntoIter as Iterator>:: Item: AsRef<str> {
	type Item: AsRef<str>;
	fn iter(&'a self) -> std::slice::Iter<'a, Self::Item>;
	
	type IntoIter: Iterator;
	fn into_iter(self) -> Self::IntoIter;
	
	fn len(&self) -> usize;
}


impl<'l, U> ArrayStringConcat<'l> for &'l U where U: ArrayStringConcat<'l>, <Self::IntoIter as Iterator>:: Item: AsRef<str> {
	type Item = U::Item;
	fn iter(&'l self) -> std::slice::Iter<'l, Self::Item> {
		<U>::iter(self)
	}
	
	type IntoIter = std::slice::Iter<'l, Self::Item>;
	fn into_iter(self) -> Self::IntoIter {
		
	}
	
	fn len(&self) -> usize {
		<U as ArrayStringConcat>::len(self)
	}
}
impl<'l, U> ArrayStringConcat<'l> for &'l mut U where U: ArrayStringConcat<'l> {
	type Item = U::Item;
	
	fn iter<'s>(&'s self) -> std::slice::Iter<'s, Self::Item> {
		U::iter(self)
	}
	
	fn len(&self) -> usize {
		U::len(self)
	}
}




impl<'a, T> ArrayStringConcat<'a> for Vec<T> where T: AsRef<str> {
	type Item = T;
	
	#[inline(always)]
	fn iter<'s>(&'s self) -> std::slice::Iter<'s, Self::Item> {
		(**self).iter()
	}
	
	#[inline(always)]
	fn len(&self) -> usize {
		(**self).len()
	}
}

impl<'a, T> ArrayStringConcat<'a> for [T] where T: AsRef<str> {
	type Item = T;
	
	#[inline(always)]
	fn iter<'s>(&'s self) -> std::slice::Iter<'s, T> {
		<[T]>::iter(self)
	}
	
	#[inline(always)]
	fn len(&self) -> usize {
		<[T]>::len(self)
	}
}

impl<'a, T> ArrayStringConcat<'a> for &'a [T] where T: AsRef<str> {
	type Item = T;
	
	#[inline(always)]
	fn iter<'s>(&'s self) -> std::slice::Iter<'s, T> {
		(**self).iter()
	}
	
	#[inline(always)]
	fn len(&self) -> usize {
		(**self).len()
	}
}






#[cfg(test)]
mod tests {
	#[allow(unused_imports)]
	use super::*;
	
	
	#[test]
	fn one_concat() {
		assert_eq!(array_string_concat(&["1"]), "1");
	}
	#[test]
	fn two_concat() {
		assert_eq!(array_string_concat(&["1", "2"]), "12");
	}
	
	#[test]
	fn full_concat() {
		let array = &[
			"1", "2345", "67", "89", "", "."
		];
		
		assert_eq!(array_string_concat(array), "123456789.");
	}
}

*/