use std::env::args;
use std::fmt::Formatter;
use std::fs::read;

const REGISTER_SIZE: usize = 8;

enum Instruction {
    MovR32Imm32(usize),
    ShortJump,
    NotImplemented,
}

impl From<u8> for Instruction {
    fn from(b: u8) -> Self {
        if b >= 0xb8 && b <= 0xbf {
            return Instruction::MovR32Imm32((b - 0xb8) as usize);
        }
        match b {
            0xeb => Instruction::ShortJump,
            _ => Instruction::NotImplemented,
        }
    }
}

#[derive(Debug)]
struct Emulator {
    // TODO
    registers: [u32; REGISTER_SIZE],
    eflags: u32,
    memory: Vec<u8>,
    memory_capacity: usize,
    eip: u32,
}

impl Emulator {
    pub fn new() -> Emulator {
        Emulator {
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            eflags: 0,
            memory: Vec::new(),
            memory_capacity: 1024,
            eip: 0,
        }
    }
    pub fn load(self: &mut Self, codes: Vec<u8>) -> Result<(), String> {
        if codes.len() > self.memory_capacity {
            Err(format!(
                "code size {} is larger than memory capacity {}",
                codes.len(),
                self.memory_capacity
            ))
        } else {
            self.memory = codes;
            Ok(())
        }
    }

    pub fn run(self: &mut Self) -> Result<(), String> {
        while (self.eip as usize) < self.memory.len() {
            if let Some(b) = self.memory.get(self.eip as usize) {
                match Instruction::from(*b) {
                    Instruction::MovR32Imm32(idx) => match self.fetch_32(1) {
                        Ok(v) => {
                            self.registers[idx] = v;
                            self.eip += 5
                        }
                        Err(e) => return Err(e),
                    },
                    Instruction::ShortJump => {
                        let diff = self.fetch_signed_8(1).expect("failed to fetch signed 8");
                        self.eip += 2;
                        let abs = diff.abs() as u32;
                        if diff >= 0 {
                            self.eip += abs;
                        } else {
                            self.eip -= abs;
                        }
                        println!("short jump to {}", self.eip);
                    }
                    Instruction::NotImplemented => return Err(format!("NotImplemented: {}", b)),
                }
            } else {
                println!("end of program");
                break;
            }
        }
        Ok(())
    }

    fn fetch_8(self: &Self, offset: usize) -> Result<u8, String> {
        let idx = self.eip as usize + offset;
        if let Some(val) = self.memory.get(idx) {
            Ok(*val)
        } else {
            Err(format!("failed to load 32 bit value from {}", idx))
        }
    }

    fn fetch_signed_8(self: &Self, offset: usize) -> Result<i8, String> {
        let idx = self.eip as usize + offset;
        if let Some(val) = self.memory.get(idx) {
            Ok(*val as i8)
        } else {
            Err(format!("failed to load 32 bit value from {}", idx))
        }
    }

    fn fetch_32(self: &Self, offset: usize) -> Result<u32, String> {
        let mut val = 0_u32;
        for i in 0..4 {
            match self.fetch_8(offset + i) {
                Ok(v) => {
                    let mut v32 = v as u32;
                    v32 <<= i * 8;
                    val |= v32;
                }
                Err(e) => return Err(e),
            }
        }
        Ok(val)
    }

    pub fn show_registers(self: &Self) {
        for v in self.registers.iter() {
            println!("{}", v);
        }
    }
}

impl std::fmt::Display for Emulator {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        f.write_str(format!("<Emulator code_size={}>", self.memory.len()).as_str())
    }
}

fn main() {
    if let Some(filename) = args().nth(1) {
        match read(filename.clone()) {
            Ok(codes) => {
                let mut emulator = Emulator::new();
                emulator.load(codes).unwrap();
                println!("{}", emulator);
                println!("start emulation...");
                if let Err(e) = emulator.run() {
                    eprintln!("ERROR: {}", e)
                }
                emulator.show_registers();
            }
            Err(e) => eprintln!("cannot read {}: {}", filename, e),
        }
    } else {
        eprintln!("Usage: {} FILENAME", args().nth(0).unwrap());
    }
}
