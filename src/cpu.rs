use memory::Memory;
use std::mem;
use std::fmt;

#[derive(Show)]
enum InstructionMode {
    IMP,
    NUL,
    ZPG,
    ABS,
    IMM,
    ABY,
    ABX,
    ZPY,
    ZPX,
    INY,
    INX,
    XIN,
    REL,
    ACC,
    IND
}


const InstructionOpcodes: [&'static str; 256] = [
    "BRK", "ORA",  ""  ,  ""  ,  ""  , "ORA", "ASL",  ""  ,
    "PHP", "ORA", "ASL",  ""  ,  ""  , "ORA", "ASL",  ""  ,
    "BPL", "ORA",  ""  ,  ""  ,  ""  , "ORA", "ASL",  ""  ,
    "CLC", "ORA",  ""  ,  ""  ,  ""  , "ORA", "ASL",  ""  ,
    "JSR", "AND",  ""  ,  ""  , "BIT", "AND", "ROL",  ""  ,
    "PLP", "AND", "ROL",  ""  , "BIT", "AND", "ROL",  ""  ,
    "BMI", "AND",  ""  ,  ""  ,  ""  , "AND", "ROL",  ""  ,
    "SEC", "AND",  ""  ,  ""  ,  ""  , "AND", "ROL",  ""  ,
    "RTI", "EOR",  ""  ,  ""  ,  ""  , "EOR", "LSR",  ""  ,
    "PHA", "EOR", "LSR",  ""  , "JMP", "EOR", "LSR",  ""  ,
    "BVC", "EOR",  ""  ,  ""  ,  ""  , "EOR", "LSR",  ""  ,
    "CLI", "EOR",  ""  ,  ""  ,  ""  , "EOR", "LSR",  ""  ,
    "RTS", "ADC",  ""  ,  ""  ,  ""  , "ADC", "ROR",  ""  ,
    "PLA", "ADC", "ROR",  ""  , "JMP", "ADC", "ROR",  ""  ,
    "BVS", "ADC",  ""  ,  ""  ,  ""  , "ADC", "ROR",  ""  ,
    "SEI", "ADC",  ""  ,  ""  ,  ""  , "ADC", "ROR",  ""  ,
    "BCS", "STA",  ""  ,  ""  , "STY", "STA", "STX",  ""  ,
    "DEY",  ""  , "TXA",  ""  , "STY", "STA", "STX",  ""  ,
    "BCC", "STA",  ""  ,  ""  , "STY", "STA", "STX",  ""  ,
    "TYA", "STA", "TXS",  ""  ,  ""  , "STA",  ""  ,  ""  ,
    "LDY", "LDA", "LDX",  ""  , "LDY", "LDA", "LDX",  ""  ,
    "TAY", "LDA", "TAX",  ""  , "LDY", "LDA", "LDX",  ""  ,
    "BCS", "LDA",  ""  ,  ""  , "LDY", "LDA", "LDX",  ""  ,
    "CLV", "LDA", "TSX",  ""  , "LDY", "LDA", "LDX",  ""  ,
    "CPY", "CMP",  ""  ,  ""  , "CPY", "CMP", "DEC",  ""  ,
    "INY", "CMP", "DEX",  ""  , "CPY", "CMP", "DEC",  ""  ,
    "BNE", "CMP",  ""  ,  ""  ,  ""  , "CMP", "DEC",  ""  ,
    "CLD", "CMP",  ""  ,  ""  ,  ""  , "CMP", "DEC",  ""  ,
    "CPX", "SBC",  ""  ,  ""  , "CPX", "SBC", "INC",  ""  ,
    "INX", "SBC", "NOP",  ""  , "CPX", "SBC", "INC",  ""  ,
    "BEQ", "SBC",  ""  ,  ""  ,  ""  , "SBC", "INC",  ""  ,
    "SED", "SBC",  ""  ,  ""  ,  ""  , "SBC", "INC",  ""  
];

const InstructionSizes: [u8; 256] = [
    1, 2, 0, 0, 0, 2, 2, 0, 1, 2, 1, 0, 0, 3, 3, 0,
    2, 2, 0, 0, 0, 2, 2, 0, 1, 3, 0, 0, 0, 3, 3, 0,
    3, 2, 0, 0, 2, 2, 2, 0, 1, 2, 1, 0, 3, 3, 3, 0,
    2, 2, 0, 0, 0, 2, 2, 0, 1, 3, 0, 0, 0, 3, 3, 0,
    1, 2, 0, 0, 0, 2, 2, 0, 1, 2, 1, 0, 3, 3, 3, 0,
    2, 2, 0, 0, 0, 2, 2, 0, 1, 3, 0, 0, 0, 3, 3, 0,
    1, 2, 0, 0, 0, 2, 2, 0, 1, 2, 1, 0, 3, 3, 3, 0,
    2, 2, 0, 0, 0, 2, 2, 0, 1, 3, 0, 0, 0, 3, 3, 0,
    2, 2, 0, 0, 2, 2, 2, 0, 1, 0, 1, 0, 3, 3, 3, 0,
    2, 2, 0, 0, 2, 2, 2, 0, 1, 3, 1, 0, 0, 3, 0, 0,
    2, 2, 2, 0, 2, 2, 2, 0, 1, 2, 1, 0, 3, 3, 3, 0,
    2, 2, 0, 0, 2, 2, 2, 0, 1, 3, 1, 0, 3, 3, 3, 0,
    2, 2, 0, 0, 2, 2, 2, 0, 1, 2, 1, 0, 3, 3, 3, 0,
    2, 2, 0, 0, 0, 2, 2, 0, 1, 3, 0, 0, 0, 3, 3, 0,
    2, 2, 0, 0, 2, 2, 2, 0, 1, 2, 1, 0, 3, 3, 3, 0,
    2, 2, 0, 0, 0, 2, 2, 0, 1, 3, 0, 0, 0, 3, 3, 0
];

const InstructionTicks: [u8; 256] = [
    7, 6, 0, 0, 0, 3, 5, 0, 3, 2, 2, 0, 0, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
    6, 6, 0, 0, 3, 3, 5, 0, 4, 2, 2, 0, 4, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
    6, 6, 0, 0, 0, 3, 5, 0, 3, 2, 2, 0, 3, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
    6, 6, 0, 0, 0, 3, 5, 0, 4, 2, 2, 0, 5, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
    2, 6, 0, 0, 3, 3, 3, 0, 2, 0, 2, 0, 4, 4, 4, 0,
    2, 6, 0, 0, 4, 4, 4, 0, 2, 5, 2, 0, 0, 5, 0, 0,
    2, 6, 2, 0, 3, 3, 3, 0, 2, 2, 2, 0, 4, 4, 4, 0,
    2, 5, 0, 0, 4, 4, 4, 0, 2, 4, 2, 0, 4, 4, 4, 0,
    2, 6, 0, 0, 3, 3, 5, 0, 2, 2, 2, 0, 4, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0,
    2, 6, 0, 0, 3, 3, 5, 0, 2, 2, 2, 0, 4, 4, 6, 0,
    2, 5, 0, 0, 0, 4, 6, 0, 2, 4, 0, 0, 0, 4, 7, 0
];

const InstructionModes:[InstructionMode; 256] = [
    InstructionMode::IMP, InstructionMode::XIN, InstructionMode::NUL, InstructionMode::NUL,   // 0x00-0x03
    InstructionMode::NUL, InstructionMode::ZPG, InstructionMode::ZPG, InstructionMode::NUL,   // 0x04-0x07
    InstructionMode::IMP, InstructionMode::IMM, InstructionMode::ACC, InstructionMode::NUL,   // 0x08-0x0b
    InstructionMode::NUL, InstructionMode::ABS, InstructionMode::ABS, InstructionMode::NUL,   // 0x0c-0x0f
    InstructionMode::REL, InstructionMode::INY, InstructionMode::NUL, InstructionMode::NUL,   // 0x10-0x13
    InstructionMode::NUL, InstructionMode::ZPX, InstructionMode::ZPX, InstructionMode::NUL,   // 0x14-0x17
    InstructionMode::IMP, InstructionMode::ABY, InstructionMode::NUL, InstructionMode::NUL,   // 0x18-0x1b
    InstructionMode::NUL, InstructionMode::ABX, InstructionMode::ABX, InstructionMode::NUL,   // 0x1c-0x1f
    InstructionMode::ABS, InstructionMode::XIN, InstructionMode::NUL, InstructionMode::NUL,   // 0x20-0x23
    InstructionMode::ZPG, InstructionMode::ZPG, InstructionMode::ZPG, InstructionMode::NUL,   // 0x24-0x27
    InstructionMode::IMP, InstructionMode::IMM, InstructionMode::ACC, InstructionMode::NUL,   // 0x28-0x2b
    InstructionMode::ABS, InstructionMode::ABS, InstructionMode::ABS, InstructionMode::NUL,   // 0x2c-0x2f
    InstructionMode::REL, InstructionMode::INY, InstructionMode::NUL, InstructionMode::NUL,   // 0x30-0x33
    InstructionMode::NUL, InstructionMode::ZPX, InstructionMode::ZPX, InstructionMode::NUL,   // 0x34-0x37
    InstructionMode::IMP, InstructionMode::ABY, InstructionMode::NUL, InstructionMode::NUL,   // 0x38-0x3b
    InstructionMode::NUL, InstructionMode::ABX, InstructionMode::ABX, InstructionMode::NUL,   // 0x3c-0x3f
    InstructionMode::IMP, InstructionMode::XIN, InstructionMode::NUL, InstructionMode::NUL,   // 0x40-0x43
    InstructionMode::NUL, InstructionMode::ZPG, InstructionMode::ZPG, InstructionMode::NUL,   // 0x44-0x47
    InstructionMode::IMP, InstructionMode::IMM, InstructionMode::ACC, InstructionMode::NUL,   // 0x48-0x4b
    InstructionMode::ABS, InstructionMode::ABS, InstructionMode::ABS, InstructionMode::NUL,   // 0x4c-0x4f
    InstructionMode::REL, InstructionMode::INY, InstructionMode::NUL, InstructionMode::NUL,   // 0x50-0x53
    InstructionMode::NUL, InstructionMode::ZPX, InstructionMode::ZPX, InstructionMode::NUL,   // 0x54-0x57
    InstructionMode::IMP, InstructionMode::ABY, InstructionMode::NUL, InstructionMode::NUL,   // 0x58-0x5b
    InstructionMode::NUL, InstructionMode::ABX, InstructionMode::ABX, InstructionMode::NUL,   // 0x5c-0x5f
    InstructionMode::IMP, InstructionMode::XIN, InstructionMode::NUL, InstructionMode::NUL,   // 0x60-0x63
    InstructionMode::NUL, InstructionMode::ZPG, InstructionMode::ZPG, InstructionMode::NUL,   // 0x64-0x67
    InstructionMode::IMP, InstructionMode::IMM, InstructionMode::ACC, InstructionMode::NUL,   // 0x68-0x6b
    InstructionMode::IND, InstructionMode::ABS, InstructionMode::ABS, InstructionMode::NUL,   // 0x6c-0x6f
    InstructionMode::REL, InstructionMode::INY, InstructionMode::NUL, InstructionMode::NUL,   // 0x70-0x73
    InstructionMode::NUL, InstructionMode::ZPX, InstructionMode::ZPX, InstructionMode::NUL,   // 0x74-0x77
    InstructionMode::IMP, InstructionMode::ABY, InstructionMode::NUL, InstructionMode::NUL,   // 0x78-0x7b
    InstructionMode::NUL, InstructionMode::ABX, InstructionMode::ABX, InstructionMode::NUL,   // 0x7c-0x7f
    InstructionMode::REL, InstructionMode::XIN, InstructionMode::NUL, InstructionMode::NUL,   // 0x80-0x83
    InstructionMode::ZPG, InstructionMode::ZPG, InstructionMode::ZPG, InstructionMode::NUL,   // 0x84-0x87
    InstructionMode::IMP, InstructionMode::NUL, InstructionMode::IMP, InstructionMode::NUL,   // 0x88-0x8b
    InstructionMode::ABS, InstructionMode::ABS, InstructionMode::ABS, InstructionMode::NUL,   // 0x8c-0x8f
    InstructionMode::REL, InstructionMode::INY, InstructionMode::NUL, InstructionMode::NUL,   // 0x90-0x93
    InstructionMode::ZPX, InstructionMode::ZPX, InstructionMode::ZPY, InstructionMode::NUL,   // 0x94-0x97
    InstructionMode::IMP, InstructionMode::ABY, InstructionMode::IMP, InstructionMode::NUL,   // 0x98-0x9b
    InstructionMode::NUL, InstructionMode::ABX, InstructionMode::NUL, InstructionMode::NUL,   // 0x9c-0x9f
    InstructionMode::IMM, InstructionMode::XIN, InstructionMode::IMM, InstructionMode::NUL,   // 0xa0-0xa3
    InstructionMode::ZPG, InstructionMode::ZPG, InstructionMode::ZPG, InstructionMode::NUL,   // 0xa4-0xa7
    InstructionMode::IMP, InstructionMode::IMM, InstructionMode::IMP, InstructionMode::NUL,   // 0xa8-0xab
    InstructionMode::ABS, InstructionMode::ABS, InstructionMode::ABS, InstructionMode::NUL,   // 0xac-0xaf
    InstructionMode::REL, InstructionMode::INY, InstructionMode::NUL, InstructionMode::NUL,   // 0xb0-0xb3
    InstructionMode::ZPX, InstructionMode::ZPX, InstructionMode::ZPY, InstructionMode::NUL,   // 0xb4-0xb7
    InstructionMode::IMP, InstructionMode::ABY, InstructionMode::IMP, InstructionMode::NUL,   // 0xb8-0xbb
    InstructionMode::ABX, InstructionMode::ABX, InstructionMode::ABY, InstructionMode::NUL,   // 0xbc-0xbf
    InstructionMode::IMM, InstructionMode::XIN, InstructionMode::NUL, InstructionMode::NUL,   // 0xc0-0xc3
    InstructionMode::ZPG, InstructionMode::ZPG, InstructionMode::ZPG, InstructionMode::NUL,   // 0xc4-0xc7
    InstructionMode::IMP, InstructionMode::IMM, InstructionMode::IMP, InstructionMode::NUL,   // 0xc8-0xcb
    InstructionMode::ABS, InstructionMode::ABS, InstructionMode::ABS, InstructionMode::NUL,   // 0xcc-0xcf
    InstructionMode::REL, InstructionMode::INY, InstructionMode::NUL, InstructionMode::NUL,   // 0xd0-0xd3
    InstructionMode::NUL, InstructionMode::ZPX, InstructionMode::ZPX, InstructionMode::NUL,   // 0xd4-0xd7
    InstructionMode::IMP, InstructionMode::ABY, InstructionMode::NUL, InstructionMode::NUL,   // 0xd8-0xdb
    InstructionMode::NUL, InstructionMode::ABX, InstructionMode::ABX, InstructionMode::NUL,   // 0xdc-0xdf
    InstructionMode::IMM, InstructionMode::XIN, InstructionMode::NUL, InstructionMode::NUL,   // 0xe0-0xe3
    InstructionMode::ZPG, InstructionMode::ZPG, InstructionMode::ZPG, InstructionMode::NUL,   // 0xe4-0xe7
    InstructionMode::IMP, InstructionMode::IMM, InstructionMode::IMP, InstructionMode::NUL,   // 0xe8-0xeb
    InstructionMode::ABS, InstructionMode::ABS, InstructionMode::ABS, InstructionMode::NUL,   // 0xec-0xef
    InstructionMode::REL, InstructionMode::INY, InstructionMode::NUL, InstructionMode::NUL,   // 0xf0-0xf3
    InstructionMode::NUL, InstructionMode::ZPX, InstructionMode::ZPX, InstructionMode::NUL,   // 0xf4-0xf7
    InstructionMode::IMP, InstructionMode::ABY, InstructionMode::NUL, InstructionMode::NUL,   // 0xf8-0xfb
    InstructionMode::NUL, InstructionMode::ABX, InstructionMode::ABX, InstructionMode::NUL    // 0xfc-0xff
];

#[derive(Show)]
struct Op {
    opcode: &'static str,
    size: u8,
    ticks: u8,
    raw: u8,
    param: u16,
    mode: InstructionMode
}

impl Op {
    fn new(instruction: u8, param_lo: u8, param_hi: u8) -> Op {
        let index = instruction as usize;
        let param:u16 = (param_hi as u16) * 0x100 + (param_lo as u16);
        Op {
            raw: instruction,
            opcode: InstructionOpcodes[index],
            size: InstructionSizes[index],
            ticks: InstructionTicks[index],
            mode: InstructionModes[index],
            param: param
        }
    }
}

enum Ops {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA
}

pub struct CPU<'a> {
    // Registers
    pc: u16,
    a: u8,
    x: u8,
    y: u8,
    s: u8,

    // Flags
    n: u8,
    v: u8,
    d: u8,
    i: u8,
    z: u8,
    c: u8,

    halted: u8,
    memory: &'a mut Memory,

}

impl<'a> CPU<'a> {
    pub fn new(memory: &mut Memory) -> CPU {
        CPU {
            pc: 0,
            a: 0,
            x: 0,
            y: 0,
            s: 0,
            n: 0,
            v: 0,
            d: 0,
            i: 0,
            z: 0,
            c: 0,
            halted: 0,
            memory: memory
        }
    }

    pub fn tick(&mut self) {
        let operation = self.fetch();
        println!("Operation is {:?} ( {:X} )", operation, operation.raw);
        self.decode(&operation);
        self.execute(&operation);
    }

    fn fetch(&mut self) -> Op {
        let opcode:u8 = self.mem_get(self.pc);
        let param_lo:u8 = self.mem_get(self.pc + 1);
        let param_hi:u8 = self.mem_get(self.pc + 2);
        // println!("Opcode is {:X}, Param is {:X} pc is {}", opcode, param, self.pc);

        let operation = Op::new(opcode, param_lo, param_hi);
        self.pc += operation.size as u16;

        operation
    }

    fn mem_get(&self, offset: u16) -> u8 {
        self.memory.get(offset)
    }

    fn decode(&self, operation: &Op) {
    }

    fn execute(&mut self, operation: &Op) {
    }
}
