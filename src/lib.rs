//Copyright 2019 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

//#Ulin Project 1819

#![allow(non_snake_case)]

#[macro_use]
mod macros {
	#[macro_use]
	mod str_concat;
	pub use self::str_concat::*;
	
	#[macro_use]
	mod string;
	pub use self::string::*;
}

pub use self::macros::*;

//Expects stabilization.