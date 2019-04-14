// use rand::prelude::*;

pub struct RandomGenerator {
	x: u32
}

impl RandomGenerator {
	pub fn new() -> RandomGenerator {
		RandomGenerator { x: rand::random() }
	}
}

impl Iterator for RandomGenerator {
	type Item = u32;

	fn next(&mut self) -> Option<Self::Item> {
		self.x ^= self.x >> 12;
		self.x ^= self.x << 25;
		self.x ^= self.x >> 27;
		return Some(self.x);
	}
}
