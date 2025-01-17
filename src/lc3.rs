

const R0: usize = 0;
const R1: usize = 1;
const R2: usize = 2;
const R3: usize = 3;
const R4: usize = 4;
const R5: usize = 5;
const R6: usize = 6;
const R7: usize = 7;
const PC: usize = 8;
const CC: usize = 9;


const N: u16 = 0b100;
const Z: u16 = 0b010;
const P: u16 = 0b001;


const ADD: u16 = 0b0001;
const AND: u16 = 0b0101;
const BR: u16 = 0b0000;
const JMP: u16= 0b1100;
const JSR: u16 = 0b0100;
/* JSSR 0b0100 */
const LD: u16 = 0b0010;
const LDI: u16 = 0b1010;
const LDR: u16 = 0b0110;
const LEA: u16 = 0b1110;
const NOT: u16 = 0b1001;
/* RET 0b1100 */
const RTI: u16 = 0b1000;
const ST: u16 = 0b0011;
const STI: u16 = 0b1011;
const STR: u16 = 0b0111;
const TRAP: u16 = 0b1111;
const RESERVED: u16 = 0b1101;


const REG_COUNT: usize = 10;
const MEMORY_SIZE: usize = u16::MAX as usize;
const PC_START: u16 = 0x3000;


struct LC3 {
	reg: [u16; REG_COUNT],
	mem: [u16; MEMORY_SIZE]
}


impl LC3 {
	const PC_START: u16 = 0x3000;


	pub fn new() -> Self {
		let mut lc3: Self = Self {reg: [0; REG_COUNT], mem: [0; MEMORY_SIZE]};
		lc3.reg[PC] = Self::PC_START;
		lc3.reg[CC] = Z;
		return lc3;
	}


	pub fn update(&mut self) -> Result<(), ()> {
		let instruction: u16 = self.read_mem(self.reg[PC]);
		self.reg[PC] += 1;
		return self.execute(instruction);
	}


	fn read_mem(&self, address: u16) -> u16 {
		return self.mem[address as usize];
	}


	fn execute(&mut self, instruction: u16) -> Result<(), ()> {
		let opcode: u16 = instruction >> 12 & 0b1111;

		match opcode {
			ADD => self.add(instruction),
			AND => self.and(instruction),

			BR => self.add(instruction),
			JMP => self.add(instruction),
			JSR => self.add(instruction),
			LD => self.add(instruction),
			LDI => self.add(instruction),
			LDR => self.add(instruction),
			LEA => self.add(instruction),
			NOT => self.add(instruction),
			RTI => self.add(instruction),
			ST => self.add(instruction),
			STI => self.add(instruction),
			STR => self.add(instruction),
			TRAP => self.add(instruction),

			_ => return Err(())
		}

		return Ok(());
	}


	fn add(&mut self, instruction: u16) -> () {
		let dr: usize = Self::get_r0(instruction);
		let a: u16 = self.reg[Self::get_r1(instruction)];
		let b: u16 = if Self::get_imm5_flag(instruction) {Self::sign_extend(instruction & 0b11111, 5)} else {self.reg[Self::get_r2(instruction)]};
		let sum: u16 = a + b;

		self.reg[dr] = sum;
		self.set_cc(sum);
	}


	fn and(&mut self, instruction: u16) -> () {
		let dr: usize = Self::get_r0(instruction);
		let a: u16 = self.reg[Self::get_r1(instruction)];
		let b: u16 = if Self::get_imm5_flag(instruction) {Self::sign_extend(instruction & 0b11111, 5)} else {self.reg[Self::get_r2(instruction)]};
		let result: u16 = a & b;

		self.reg[dr] = result;
		self.set_cc(result);
	}


	fn br(&mut self, instruction: u16) -> () {
		let sign_flag: u16 = instruction >> 9 & 0b111;
		if sign_flag & self.reg[CC] != 0 {
			let pc_offset_9: u16 = Self::sign_extend(instruction & 0x1FF, 9);
			self.reg[PC] += pc_offset_9;
		}
	}


	fn jmp(&mut self, instruction: u16) -> () {
		let br: usize = Self::get_r1(instruction);
		self.reg[PC] = self.reg[br]
	}


	fn jsr(&mut self, instruction: u16) -> () {
		self.reg[R7] = self.reg[PC];
		let jsr_flag: bool = instruction >> 11 & 1 == 1;
		if jsr_flag {
			let pc_offset_11: u16 = Self::sign_extend(instruction & 0x7FF, 11);
			self.reg[PC] += pc_offset_11;
		}
		else {
			let br: usize = Self::get_r1(instruction);
			self.reg[PC] = self.reg[br]
		}
	}


	fn ld(&mut self, instruction: u16) -> () {
		let dr: usize = Self::get_r0(instruction);
		let pc_offset_9: u16 = Self::sign_extend(instruction & 0x1FF, 9);
		let result: u16 = self.read_mem(self.reg[PC] + pc_offset_9);

		self.reg[dr] = result;
		self.set_cc(result);
	}


	fn ldi(&mut self, instruction: u16) -> () {
		let dr: usize = Self::get_r0(instruction);
		let pc_offset_9: u16 = Self::sign_extend(instruction & 0x1FF, 9);
		let result: u16 = self.read_mem(self.read_mem(self.reg[PC] + pc_offset_9));

		self.reg[dr] = result;
		self.set_cc(result);
	}


	fn ldr(&mut self, instruction: u16) -> () {
		let dr: usize = Self::get_r0(instruction);
		let br: usize = Self::get_r1(instruction);
		let pc_offset_6: u16 = Self::sign_extend(instruction & 0x3F, 6);
		let result: u16 = self.read_mem(self.reg[br] + pc_offset_6);

		self.reg[dr] = result;
		self.set_cc(result);
	}


	fn lea(&mut self, instruction: u16) -> () {
		let dr: usize = Self::get_r0(instruction);
		let pc_offset_9: u16 = Self::sign_extend(instruction & 0x1FF, 9);
		let result: u16 = self.reg[PC] + pc_offset_9;

		self.reg[dr] = result;
		self.set_cc(result);
	}


	fn not(&mut self, instruction: u16) -> () {
		let dr: usize = Self::get_r0(instruction);
		let sr: usize = Self::get_r1(instruction);
		let result: u16 = !self.reg[sr];

		self.reg[dr] = result;
		self.set_cc(result);
	}


	fn rti(&mut self, _instruction: u16) -> () {}


	fn set_cc(&mut self, value: u16) -> () {
		if value == 0 {
			self.reg[CC] = Z;
		}
		else if value >> 15 & 1 == 1 {
			self.reg[CC] = N;
		}
		else {
			self.reg[CC] = P;
		}
	}


	fn get_r0(instruction: u16) -> usize {
		return (instruction >> 9 & 0b111) as usize;
	}


	fn get_r1(instruction: u16) -> usize {
		return (instruction >> 6 & 0b111) as usize;
	}


	fn get_r2(instruction: u16) -> usize {
		return (instruction & 0b111) as usize;
	}


	fn get_imm5_flag(instruction: u16) -> bool {
		return instruction >> 5 & 1 == 1;
	}


	fn sign_extend(a: u16, bit_count: u16) -> u16 {
		if a >> (bit_count - 1) & 1 == 1 {
			return a | (0xFFFF << bit_count);
		}
		return a & ((1 << bit_count) - 1);
	}
}
