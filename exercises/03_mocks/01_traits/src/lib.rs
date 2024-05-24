pub struct PrintlnLogger;
pub struct TestLogger;

pub trait Logger {
	fn log(&self, msg: &str);
}

impl Logger for PrintlnLogger {
	fn log(&self, msg: &str) {
		println!("{}", msg);
	}
}

impl Logger for TestLogger {
	fn log(&self, _msg: &str) {}
}

pub fn square<T: Logger>(x: i32, logger: T) -> i32 {
	let y = x * x;
	logger.log(&format!("{}^2 == {}", x, y));
	y
}

#[cfg(test)]
mod tests {
	use super::{square, TestLogger};

	#[test]
	fn square_works() {
		assert_eq!(square(2, TestLogger), 4);
	}
}
