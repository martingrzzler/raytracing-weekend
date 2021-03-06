use std::{
	cell::RefCell,
	io::{self, Write},
};

pub struct ProgressBar {
	current: RefCell<i32>,
	total: i32,
}

impl ProgressBar {
	pub fn from(total: i32) -> Self {
		Self {
			current: RefCell::new(0),
			total,
		}
	}

	fn calc(&self) -> f64 {
		(*self.current.borrow() as f64 / self.total as f64) * 100.0
	}

	fn increment(&self) {
		let curr = self.current.as_ptr();
		unsafe {
			*curr = *curr + 1;
		}
	}

	pub fn inc(&self) {
		self.increment();
		eprint!("\rProgressBar: {:.2}%", self.calc());
		io::stderr().flush().unwrap();
	}
}

#[test]
fn test_increment() {
	let p = ProgressBar::from(10);
	p.increment();
	assert_eq!(*p.current.borrow(), 1);
}

#[test]
fn test_calc() {
	let p = ProgressBar::from(10);
	p.increment();
	p.increment();

	assert_eq!(p.calc(), 20.0);
}
