use std::{
	cell::RefCell,
	io::{self, Write},
};

pub struct Progress {
	current: RefCell<i32>,
	total: i32,
}

impl Progress {
	pub fn from(total: i32) -> Self {
		Self {
			current: RefCell::new(0),
			total,
		}
	}

	fn calc(&self) -> f64 {
		(*self.current.borrow() as f64 / self.total as f64) * 100.0
	}

	pub fn print(&self) {
		let curr = self.current.as_ptr();
		unsafe {
			*curr = *curr + 1;
		}
		eprint!("\rProgress: {:.2}%", self.calc());
		io::stderr().flush().unwrap();
	}
}
