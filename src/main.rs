
mod lc3;





enum Instructions {
	ADD = 0b0001,
	AND = 0b0101,
	BR = 0b0000,
	JMP = 0b1100,
	JSR = 0b0100,
	LD = 0b0010,
	LDI = 0b1010,
	LDR = 0b0110,
	LEA = 0b1110,
	NOT = 0b1001,
	RTI = 0b1000,
	ST = 0b0011,
	STI = 0b1011,
	STR = 0b0111,
	TRAP = 0b1111,
	RESERVED = 0b1101
}




struct LC3 {






}


enum Registres {
	R0,
	R1,
	R2,
	R3,
	R4,
	R5,
	R6,
	R7,
	PC,
	CC
}


enum ConditionCodes {
	N = 0b1000,
	Z = 0b0100,
	P = 0b0010
}


enum Instructions {
	ADD = 0b0001,
	AND = 0b0101,
	BR = 0b0000,
	JMP = 0b1100,
	JSR = 0b0100,
	LD = 0b0010,
	LDI = 0b1010,
	LDR = 0b0110,
	LEA = 0b1110,
	NOT = 0b1001,
	RTI = 0b1000,
	ST = 0b0011,
	STI = 0b1011,
	STR = 0b0111,
	TRAP = 0b1111,
	RESERVED = 0b1101
}


const PC_START: u16 = 0x3000;
const REGISTERS_COUNT: usize = 10;
const MEMORY_SIZE: usize = u16::MAX as usize;


fn main() -> () {
	let mut registres: [u16; REGISTERS_COUNT] = [0; REGISTERS_COUNT];
	let mut memory: [u16; MEMORY_SIZE] = [0; MEMORY_SIZE];

	init(&mut registres);
	run(&mut registres, &mut memory);
}


fn init(registres: &mut [u16; REGISTERS_COUNT]) -> () {
	registres[Registres::PC as usize] = PC_START;
	registres[Registres::CC as usize] = ConditionCodes::Z as u16;
}


fn run(registres: &mut [u16; REGISTERS_COUNT], memory: &mut [u16; MEMORY_SIZE]) -> () {
	loop {
		let instruction: u16 = read_memory(registres[Registres::PC as usize], &memory);
		registres[Registres::PC as usize] += 1;

		if !execute(instruction, registres, memory) {break;}
	}
}


fn read_memory(address: u16, memory: &[u16; MEMORY_SIZE]) -> u16 {
	return memory[address as usize];
}


fn execute(instruction: u16, registres: &mut [u16; REGISTERS_COUNT], memory: &mut [u16; MEMORY_SIZE]) -> bool {
	let opcode: u16 = instruction & 0b1111;
	if opcode == Instructions::ADD as u16 {}
	else if opcode == Instructions::AND as u16 {}
	else if opcode == Instructions::BR as u16 {}
	else if opcode == Instructions::JMP as u16 {}
	else if opcode == Instructions::JSR as u16 {}
	else if opcode == Instructions::LD as u16 {}
	else if opcode == Instructions::LDI as u16 {}
	else if opcode == Instructions::LDR as u16 {}
	else if opcode == Instructions::LEA as u16 {}
	else if opcode == Instructions::NOT as u16 {}
	else if opcode == Instructions::RTI as u16 {}
	else if opcode == Instructions::ST as u16 {}
	else if opcode == Instructions::STI as u16 {}
	else if opcode == Instructions::TRAP as u16 {}
	else {false}
}

fn add() -> () {

}
