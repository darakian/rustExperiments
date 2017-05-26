fn main(){
	//println!("Hello rust");
	let mut my_random = k_random::WikiLsfr::new(4);
	for i in 1..100{
	println!("{}", my_random.sample());
	}
}

mod k_random{
	pub struct WikiLsfr{
		state: u16,
	}

	impl WikiLsfr{

		pub fn new(seed: u16) -> WikiLsfr{
		WikiLsfr{state: seed}
		}

		pub fn step(&mut self){
		let tmp: u16 = ((self.state >> 0) ^ (self.state >> 2) ^ (self.state >> 3) ^ (self.state >> 5) ) & 1;
		self.state = (self.state >> 1) | (tmp << 15);
		}

		pub fn sample(&mut self) -> u16{
		let current = self.state;
		self.step();
		return current;
		}
	}
}

