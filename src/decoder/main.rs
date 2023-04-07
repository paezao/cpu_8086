use std::fmt::{self, Display, Formatter};
use std::io::Read;
use std::fs::File;


#[derive(Debug)]
enum Register {
    AL, AX,
    CL, CX,
    DL, DX,
    BL, BX,
    AH, SP,
    CH, BP,
    DH, SI,
    BH, DI,
}
impl Register {
    fn from_bytes(value: u8, word: bool) -> Register {
        match (value, word) {
            (0b000, false) => Register::AL,
            (0b000, true) => Register::AX,
            (0b001, false) => Register::CL,
            (0b001, true) => Register::CX,
            (0b010, false) => Register::DL,
            (0b010, true) => Register::DX,
            (0b011, false) => Register::BL,
            (0b011, true) => Register::BX,
            (0b100, false) => Register::AH,
            (0b100, true) => Register::SP,
            (0b101, false) => Register::CH,
            (0b101, true) => Register::BP,
            (0b110, false) => Register::DH,
            (0b110, true) => Register::SI,
            (0b111, false) => Register::BH,
            (0b111, true) => Register::DI,
            (2_u8..=u8::MAX, _) => todo!(),
        }
    }
}

trait Instruction: fmt::Display {}

struct MovInstructionRegisterToRegister {
    source: Register,
    destination: Register,
}
impl Instruction for MovInstructionRegisterToRegister{}
impl Display for MovInstructionRegisterToRegister {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "MOV {:?}, {:?}",
            self.destination, self.source
        )
    }
}

#[derive(Debug)]
struct EncodedInstruction {
    opcode: u8,
    d: u8,
    w: u8,
    mode: u8,
    reg: u8,
    rm: u8,
}

impl EncodedInstruction {
    fn from_bytes(value: u16) -> EncodedInstruction {
        let opcode = (value >> 10) as u8;
        let d = ((value >> 9) & 0b1) as u8;
        let w = ((value >> 8) & 0b1) as u8;
        let mode = ((value >> 6) & 0b11) as u8;
        let reg = ((value >> 3) & 0b111) as u8;
        let rm = (value & 0b111) as u8;

        EncodedInstruction { opcode, d, w, mode, reg, rm }   
    }

    fn decode(&self) -> impl Instruction {
        match self.opcode {
            0b100010 => {
                let (source, destination);
                if self.d == 0b0 {
                    source = Register::from_bytes(self.reg, self.w == 0b1);
                    destination = Register::from_bytes(self.rm, self.w == 0b1);
                } else {
                    source = Register::from_bytes(self.rm, self.w == 0b1);
                    destination = Register::from_bytes(self.reg, self.w == 0b1);
                }
                MovInstructionRegisterToRegister {
                   source, destination
                } 
           }
            0_u8..=33_u8 | 35_u8..=u8::MAX => todo!()
        }
    }
}

impl Display for EncodedInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Instruction [\n\topcode: {:06b}\n\td: {:01b}\n\tw: {:01b}\n\tmod: {:02b}\n\treg: {:03b}\n\tr/m: {:03b}\n]",
            self.opcode, self.d, self.w, self.mode, self.reg, self.rm            
        )
    }
}


fn main() {
    //let file_path = "C:\\dev\\cpu_8086\\examples\\listing_0037_single_register_mov";
    let file_path = "C:\\dev\\cpu_8086\\examples\\listing_0038_many_register_mov";
    println!("Reading file {}", file_path);

    let mut file = File::open(file_path).unwrap();
    let mut buffer = [0u8; 2];

    loop {
       match file.read_exact(&mut buffer) {
        Ok(_) => {
            let value = u16::from_be_bytes(buffer);
            let encoded_instruction = EncodedInstruction::from_bytes(value);
            let decoded_instruction = encoded_instruction.decode();
            println!("{}", decoded_instruction);
        },
        Err(_) => break,
       }
    }
}
