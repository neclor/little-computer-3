

macro_rules! get_r0 {($instr: expr) => {(($instr >> 9 & 0b111) as usize)};}
macro_rules! get_r1 {($instr: expr) => {(($instr >> 6 & 0b111) as usize)};}
macro_rules! get_r2 {($instr: expr) => {(($instr & 0b111) as usize)};}
macro_rules! get_imm5_flag {($instr: expr) => {($instr >> 5 & 1 == 1)};}
macro_rules! sign_extend {
	($a: expr, $bit_count: expr) => {
		(
			if $a >> ($bit_count - 1) & 1 == 1 {
				$a | (0xFFFF << $bit_count)
			}
			else {
				$a & ((1 << $bit_count) - 1)
			}
		)
	};
}


const P: u16 = 0b001;
const Z: u16 = 0b010;
const N: u16 = 0b100;


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


const TRAP_GETC: usize = 0x20;
const TRAP_OUT: usize = 0x21;
const TRAP_PUTS: usize = 0x22;
const TRAP_IN: usize = 0x23;
const TRAP_PUTSP: usize = 0x24;
const TRAP_HALT: usize = 0x25;


/*
const KBSR: usize = 0xFE00;
const KBDR: usize = 0xFE02;
const DSR: usize = 0xFE04;
const DDR: usize = 0xFE06;
const MCR: usize = 0xFFFE;
*/


const MEMORY_SIZE: usize = 1 << 16;
const REG_COUNT: usize = 8;
const PC_START: u16 = 0x3000;


pub struct LC3 {
	mem: [u16; MEMORY_SIZE],
	reg: [u16; REG_COUNT],
	pc: u16,
	cc: u16
}


impl LC3 {
	pub fn new() -> Self {
		Self {
			mem: [0; MEMORY_SIZE],
			reg: [0; REG_COUNT],
			pc: PC_START,
			cc: Z
		}
	}


	pub fn update(&mut self) -> Result<(), ()> {
		let instr: u16 = self.mem[self.pc as usize];
		self.pc += 1;
		let opcode: u16 = instr >> 12 & 0xF;

		match opcode {
			ADD => self.add(instr),
			AND => self.and(instr),
			BR => self.br(instr),
			JMP => self.jmp(instr),
			JSR => self.jsr(instr),
			LD => self.add(instr),
			LDI => self.ldi(instr),
			LDR => self.ldr(instr),
			LEA => self.lea(instr),
			NOT => self.not(instr),
			RTI => self.rti(instr),
			ST => self.st(instr),
			STI => self.sti(instr),
			STR => self.str(instr),
			TRAP => self.trap(instr),

			_ => return Err(())
		}

		return Ok(());
	}


	fn add(&mut self, instr: u16) -> () {
		let dr: usize = get_r0!(instr);
		let a: u16 = self.reg[get_r1!(instr)];
		let b: u16 = if get_imm5_flag!(instr) {sign_extend!(instr & 0x1F, 5)} else {self.reg[get_r2!(instr)]};
		let value: u16 = a + b;

		self.reg[dr] = value;
		self.set_cc(value);
	}


	fn and(&mut self, instr: u16) -> () {
		let dr: usize = get_r0!(instr);
		let a: u16 = self.reg[get_r1!(instr)];
		let b: u16 = if get_imm5_flag!(instr) {sign_extend!(instr & 0x1F, 5)} else {self.reg[get_r2!(instr)]};
		let value: u16 = a & b;

		self.reg[dr] = value;
		self.set_cc(value);
	}


	fn br(&mut self, instr: u16) -> () {
		let sign_flag: u16 = instr >> 9 & 0b111;
		if sign_flag & self.cc != 0 {
			let pc_offset_9: u16 = sign_extend!(instr & 0x1FF, 9);
			self.pc += pc_offset_9;
		}
	}


	fn jmp(&mut self, instr: u16) -> () {
		let br: usize = get_r1!(instr);
		self.pc = self.reg[br]
	}


	fn jsr(&mut self, instr: u16) -> () {
		self.reg[7] = self.pc;
		let jsr_flag: bool = instr >> 11 & 1 == 1;
		if jsr_flag {
			let pc_offset_11: u16 = sign_extend!(instr & 0x7FF, 11);
			self.pc += pc_offset_11;
		}
		else {
			let br: usize = get_r1!(instr);
			self.pc = self.reg[br]
		}
	}


	fn ld(&mut self, instr: u16) -> () {
		let dr: usize = get_r0!(instr);
		let pc_offset_9: u16 = sign_extend!(instr & 0x1FF, 9);
		let address: usize = (self.pc + pc_offset_9) as usize;
		let value: u16 = self.mem[address];

		self.reg[dr] = value;
		self.set_cc(value);
	}


	fn ldi(&mut self, instr: u16) -> () {
		let dr: usize = get_r0!(instr);
		let pc_offset_9: u16 = sign_extend!(instr & 0x1FF, 9);
		let address: usize = (self.pc + pc_offset_9) as usize;
		let address_2: usize = self.mem[address] as usize;
		let value: u16 = self.mem[address_2];

		self.reg[dr] = value;
		self.set_cc(value);
	}


	fn ldr(&mut self, instr: u16) -> () {
		let dr: usize = get_r0!(instr);
		let br: usize = get_r1!(instr);
		let pc_offset_6: u16 = sign_extend!(instr & 0x3F, 6);
		let address: usize = (self.reg[br] + pc_offset_6) as usize;
		let value: u16 = self.mem[address];

		self.reg[dr] = value;
		self.set_cc(value);
	}


	fn lea(&mut self, instr: u16) -> () {
		let dr: usize = get_r0!(instr);
		let pc_offset_9: u16 = sign_extend!(instr & 0x1FF, 9);
		let address: u16 = self.pc + pc_offset_9;

		self.reg[dr] = address;
		self.set_cc(address);
	}


	fn not(&mut self, instr: u16) -> () {
		let dr: usize = get_r0!(instr);
		let sr: usize = get_r1!(instr);
		let value: u16 = !self.reg[sr];

		self.reg[dr] = value;
		self.set_cc(value);
	}


	fn rti(&mut self, _instr: u16) -> () {}


	fn st(&mut self, instr: u16) -> () {
		let sr: usize = get_r0!(instr);
		let pc_offset_9: u16 = sign_extend!(instr & 0x1FF, 9);
		let address: usize = (self.pc + pc_offset_9) as usize;
		self.mem[address] = self.reg[sr];
	}


	fn sti(&mut self, instr: u16) -> () {
		let sr: usize = get_r0!(instr);
		let pc_offset_9: u16 = sign_extend!(instr & 0x1FF, 9);
		let address: usize = (self.pc + pc_offset_9) as usize;
		let address_2: usize = self.mem[address] as usize;
		self.mem[address_2] = self.reg[sr];
	}


	fn str(&mut self, instr: u16) -> () {
		let sr: usize = get_r0!(instr);
		let br: usize = get_r1!(instr);
		let pc_offset_6: u16 = sign_extend!(instr & 0x3F, 6);
		let address: usize = (self.reg[br] + pc_offset_6) as usize;
		self.mem[address] = self.reg[sr];
	}


	fn trap(&mut self, instr: u16) -> () {

	}


	fn set_cc(&mut self, value: u16) -> () {
		if value == 0 {
			self.cc = Z;
		}
		else if value >> 15 & 1 == 1 {
			self.cc = N;
		}
		else {
			self.cc = P;
		}
	}
}
