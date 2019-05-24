use std::fmt::Formatter;

const REGISTER_SIZE: usize = 8;

type Result<T> = std::result::Result<T, String>;

#[derive(Debug)]
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
pub struct Emulator {
    // TODO: use array?
    // EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI
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

    pub fn load(self: &mut Self, codes: Vec<u8>) -> Result<()> {
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

    pub fn run(self: &mut Self) -> Result<()> {
        while (self.eip as usize) < self.memory.len() {
            let insn = self.current_instruction()?;
            println!("EIP={}: {:?}", self.eip, insn);
            match insn {
                Instruction::MovR32Imm32(idx) => match self.fetch_32(1) {
                    Ok(v) => {
                        self.registers[idx] = v;
                        self.eip += 5
                    }
                    Err(e) => return Err(e),
                },
                Instruction::ShortJump => {
                    let diff = self.fetch_signed_8(1)?;
                    self.eip += 2;
                    let abs = diff.abs() as u32;
                    // TODO: overflow?
                    if diff >= 0 {
                        self.eip += abs;
                    } else {
                        self.eip -= abs;
                    }
                    println!("short jump to {}", self.eip);
                }
                Instruction::NotImplemented => {
                    return Err(format!("NotImplemented at {}", self.eip))
                }
            }
            if self.eip == 0 {
                println!("end of program");
                break;
            }
        }
        Ok(())
    }

    fn current_instruction(self: &Self) -> Result<Instruction> {
        match self.memory.get(self.eip as usize) {
            Some(&b) => Ok(Instruction::from(b)),
            None => Err(format!("no instruction at {}", self.eip)),
        }
    }

    fn fetch_8(self: &Self, offset: usize) -> Result<u8> {
        let idx = self.eip as usize + offset;
        if let Some(val) = self.memory.get(idx) {
            Ok(*val)
        } else {
            Err(format!("failed to load 8 bit value from {}", idx))
        }
    }

    fn fetch_signed_8(self: &Self, offset: usize) -> Result<i8> {
        let idx = self.eip as usize + offset;
        if let Some(val) = self.memory.get(idx) {
            Ok(*val as i8)
        } else {
            Err(format!("failed to load 8 bit value from {}", idx))
        }
    }

    fn fetch_32(self: &Self, offset: usize) -> Result<u32> {
        let mut val = 0_u32;
        for i in 0..4 {
            let mut v = self.fetch_8(offset + i)? as u32;
            v <<= i * 8;
            val |= v;
        }
        Ok(val)
    }

    pub fn show_registers(self: &Self) {
        let names = vec!["EAX", "ECX", "EDX", "EBX", "ESP", "EBP", "ESI", "EDI"];
        for i in 0..REGISTER_SIZE {
            let v = self.registers[i];
            println!("{}: {:>#010x} = {:#10}", names[i], v, v);
        }
    }
}

impl std::fmt::Display for Emulator {
    fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
        f.write_str(format!("<Emulator code_size={}>", self.memory.len()).as_str())
    }
}
#[cfg(test)]
mod tests {
    use crate::Emulator;

    #[test]
    fn test_fetch_32() {
        let mut emulator = Emulator::new();
        emulator
            .load(vec![0x78, 0x56, 0x34, 0x12]) // LE
            .expect("failed to load");
        let res = emulator.fetch_32(0);
        assert_eq!(res, Ok(0x12345678))
    }
}
