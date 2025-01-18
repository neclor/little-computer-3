mod lc3;
use lc3::*;


fn main() -> () {

	let mut lc3: LC3 = LC3::new();



	let mut registres: [u16; REGISTERS_COUNT] = [0; REGISTERS_COUNT];
	let mut memory: [u16; MEMORY_SIZE] = [0; MEMORY_SIZE];

	init(&mut registres);
	run(&mut registres, &mut memory);
}
