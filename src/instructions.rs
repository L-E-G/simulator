use bit_field::BitField;

use std::fmt;
use std::fmt::{Debug,Display};

use crate::result::SimResult;
use crate::memory::{Memory,DRAM,Registers,PC};

/// Defines operations which a single instruction must perform while it is in
/// the pipeline.
pub trait Instruction: Display + Debug {
    /// Extracts parameters from instruction bits and stores them in the
    /// implementing struct for use by future stages. It also retrieves register
    /// values if necessary and does the same.
    fn decode(&mut self, instruction: u32, registers: &Registers) -> SimResult<(), String>;

    /// Executes the instruction.
    fn execute(&mut self) -> SimResult<(), String>;

    /// Accesses memory.
    fn access_memory(&mut self, memory: &mut dyn Memory<u32, u32>) -> SimResult<(), String>;

    /// Write results to registers.
    fn write_back(&mut self, registers: &mut Registers) -> SimResult<(), String>;
}

/// An instruction which performs no operations.
#[derive(Debug)]
pub struct Noop {}

impl Noop {
    pub fn new() -> Noop {
        Noop{}
    }
}

impl Display for Noop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Noop")
    }
}

impl Instruction for Noop {
    fn decode(&mut self, _instruction: u32, _registers: &Registers) -> SimResult<(), String> {
        SimResult::Wait(0, ())
    }

    fn execute(&mut self) -> SimResult<(), String> {
        SimResult::Wait(0, ())
    }

    fn access_memory(&mut self, _memory: &mut dyn Memory<u32, u32>) -> SimResult<(), String> {
        SimResult::Wait(0, ())
    }

    fn write_back(&mut self, _registers: &mut Registers) -> SimResult<(), String> {
        SimResult::Wait(0, ())
    }
}

/// Identifies types of instructions.
#[derive(PartialEq,Debug)]
pub enum InstructionT {
    ALU,
    Memory,
    Control,
    Graphics,
}

impl InstructionT {
    /// Returns the value of the type field for the represented instruction type.
    pub fn value(self) -> u32 {
        match self {
            InstructionT::Control => 0,
            InstructionT::ALU => 1,
            InstructionT::Memory => 2,
            InstructionT::Graphics => 3,
        }
    }

    /// Matches a value to an instruction type.
    pub fn match_val(val: u32) -> Option<InstructionT> {
        match val {
            0 => Some(InstructionT::Control),
            1 => Some(InstructionT::ALU),
            2 => Some(InstructionT::Memory),
            3 => Some(InstructionT::Graphics),
            _ => None,
        }
    }
}

/// Identifies the addressing mode of an instruction operand.
#[derive(PartialEq,Debug)]
pub enum AddrMode {
    /// Value is contained in the specified register.
    RegisterDirect,

    /// Value is the operand.
    Immediate,
}

impl Display for AddrMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddrMode::RegisterDirect => write!(f, "RD"),
            AddrMode::Immediate => write!(f, "I"),
        }
    }
}


/// Identifies control operations.
#[derive(PartialEq,Debug)]
pub ControlOp {
    Halt,
    Jump
}

impl ControlOp {
    /// Returns the value of the operation field for the represented operation.
    pub fn value(self) -> u32 {
        match self {
            ControlOp::Halt => 0,
            ControlOp::Jump => 1,
        }
    }

    /// Matches a value with a ControlOp.
    pub fn match_val(val: u32) -> Option<ControlOp> {
        match val {
            0 => ControlOp::Halt,
            1 => ControlOp::Jump,
            _ => None,
        }
    }
}

/// Identifies arithmetic operations.
#[derive(PartialEq,Debug)]
pub ALUOp {
    AddUnsignedI,
    Move,
    Compare,
}

impl ALUOp {
    /// Returns the value of the operation field for the represented operation.
    pub fn value(self) -> u32 {
        match self {
            ALUOp::AddUnsignedI => 2,
            ALUOp::Move => 12,
            ALUOp::Compare => 13,
        }
    }

    /// Matches a value with a ALUOp.
    pub fn match_val(val: u32) -> Option<ALUOp> {
        match val {
            2 => ALUOp::AddUnsignedI,
            12 => ALUOp::Move,
            13 => ALUOp::Compare,
            _ => None,
        }
    }
}

/// Identifies memory operations.
#[derive(PartialEq,Debug)]
pub enum MemoryOp {
    LoadRD, LoadI,
    StoreRD, StoreI,
    Push,
    Pop,
}

impl MemoryOp {
    /// Returns the value of the operation field for the represented operation.
    pub fn value(self) -> u32 {
        match self {
            MemoryOp::LoadRD => 0,
            MemoryOp::LoadI => 1,
            MemoryOp::StoreRD => 2,
            MemoryOp::StoreI => 3,
            MemoryOp::Push => 4,
            MemoryOp::Pop => 5,
        }
    }

    /// Matches a value with a MemoryOp.
    pub fn match_val(val: u32) -> Option<MemoryOp> {
        match val {
            0 => Some(MemoryOp::LoadRD),
            1 => Some(MemoryOp::LoadI),
            2 => Some(MemoryOp::StoreRD),
            3 => Some(MemoryOp::StoreI),
            4 => Some(MemoryOp::Push),
            5 => Some(MemoryOp::Pop),
            _ => None,
        }
    }
}

/// Read a value from an address in memory and place it in a register.
#[derive(Debug)]
pub struct Load {
    /// Indicates the addressing mode of the memory address operand.
    mem_addr_mode: AddrMode,
    
    /// Register to place value from memory.
    dest_reg: usize,

    /// Memory address to load into register.
    mem_addr: u32,

    /// Value loaded from mememory during access_memory.
    value: u32,
}

impl Display for Load {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Load ({})", self.mem_addr_mode)
    }
}

impl Load {
    /// Creates an empty load instruction.
    pub fn new(mem_addr_mode: AddrMode) -> Load {
        Load{
            mem_addr_mode: mem_addr_mode,
            dest_reg: 0,
            mem_addr: 0,
            value: 0,
        }
    }
}

impl Instruction for Load {
    /// Extract dest_reg and mem_addr operands.
    fn decode(&mut self, instruction: u32, registers: &Registers) -> SimResult<(), String> {
        self.dest_reg = instruction.get_bits(10..=14) as usize;
        
        if self.mem_addr_mode == AddrMode::RegisterDirect {
            self.mem_addr = registers[instruction.get_bits(15..=19) as usize];
        } else if self.mem_addr_mode == AddrMode::Immediate {
            self.mem_addr = (((registers[PC] + 1) as i32) + (instruction.get_bits(15..=31) as i32)) as u32;
        }

        return SimResult::Wait(0, ());
    }

    /// No execute step.
    fn execute(&mut self) -> SimResult<(), String> {
        return SimResult::Wait(0, ());
    }

    /// Load value at mem_addr from memory into value.
    fn access_memory(&mut self, memory: &mut dyn Memory<u32, u32>) -> SimResult<(), String> {
        match memory.get(self.mem_addr) {
            SimResult::Err(e) => SimResult::Err(
                format!("failed to retrieve memory address {}: {}",
                        self.mem_addr, e)),
            SimResult::Wait(wait, val) => {
                self.value = val;
                SimResult::Wait(wait, ())
            },
        }
    }

    /// Write value from memory into register.
    fn write_back(&mut self, registers: &mut Registers) -> SimResult<(), String> {
        registers[self.dest_reg] = self.value;
        SimResult::Wait(0, ())
    }
}

/// Writes a value in memory from a register.
#[derive(Debug)]
pub struct Store {
    /// Address in memory to save value.
    dest_addr: u32,

    /// Value in register to save in memory.
    value: u32,
}

impl Store {
    /// Create an empty store instruction.
    pub fn new() -> Store {
        Store{
            dest_addr: 0,
            value: 0,
        }
    }
}

impl Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Store")
    }
}

impl Instruction for Store {
    /// Extract operands and retrieve value to save in memory from registers.
    fn decode(&mut self, instruction: u32, registers: &Registers) -> SimResult<(), String> {
        self.value = registers[((instruction << 17) >> 26) as usize];
        self.dest_addr = registers[((instruction << 12) >> 26) as usize];

        SimResult::Wait(0, ())
    }

    /// No execution stage.
    fn execute(&mut self) -> SimResult<(), String> {
        return SimResult::Wait(0, ());
    }

    /// Set address in memory to value.
    fn access_memory(&mut self, memory: &mut dyn Memory<u32, u32>) -> SimResult<(), String> {
        println!("set({}, {}", self.dest_addr, self.value);
        match memory.set(self.dest_addr, self.value) {
            SimResult::Err(e) => SimResult::Err(
                format!("Failed to store value in {}: {}", self.dest_addr, e)),
            SimResult::Wait(wait, _res) => SimResult::Wait(wait, ()),
        }
    }

    /// No write back stage.
    fn write_back(&mut self, _registers: &mut Registers) -> SimResult<(), String> {
        SimResult::Wait(0, ())
    }
}

/*
struct Move {
    dest: usize,
    src_reg: usize,
    value: u32,
}

impl Move {
    pub fn new() -> Move {
        Move{
            dest: 0,
            src_reg: 0,
            value: 0,
        }
    }
}

impl Instruction for Move {
    /// Convert instruction to String, then to &str so we can convert it to a usize
    /// so that we can perform binary operations on it.
    /// Extract destination register from the instruction.
    /// Extract source register that holds the value to move.
    /// Get the value to move and add it to the value field.
    fn decode(&mut self, instruction: u32, registers: &mut Registers) -> SimResult<(), String> {
        let mut instString: String = instruction.to_string();
        let mut inststr: &str = &instString[..];
        let mut instbin: usize = 0;
        match usize::from_str_radix(inststr, 2) {
            Result::Err(e) => return SimResult::Err(e.to_string()),
            Result::Ok(f) => instbin = f,
        }
        // let mut instbin = usize::from_str_radix(inststr, 2).unwrap();

        self.dest = (instbin << 13) >> 31;

        self.src_reg = (instbin << 18) >> 31;

        self.value = registers[self.src_reg];
        return SimResult::Wait(0, ());
    }

    /// Skip, no execution.
    fn execute(&mut self) -> SimResult<(), String> {
        return SimResult::Wait(0, ());
    }

    /// Skip, no memory accessing.
    fn access_memory(&mut self, memory: &mut dyn Memory<u32, u32>) -> SimResult<(), String> {
        return SimResult::Wait(0, ());
    }

    /// Set the value of the destination register to the value from the source register.
    fn write_back(&mut self, registers: &mut Registers) -> SimResult<(), String> {
        registers[self.dest] = self.value;
        return SimResult::Wait(0, ());
    }
}

struct AddUIImm {
    dest: usize,
    src_reg: usize,
    op1: usize,
    op2: usize,
    result: u32,
}

impl AddUIImm {
    pub fn new() -> AddUIImm {
        AddUIImm{
            dest: 0,
            src_reg: 0,
            op1: 0,
            op2: 0,
            result: 0,
        }
    }
}

impl Instruction for AddUIImm {
    /// Convert instruction to String, then to &str so we can convert it to a usize
    /// so that we can perform binary operations on it.
    /// Extract the value of the destination register from instruction.
    /// Extract the value of the immediate value.
    /// Extract the value of the source register that stored one of the operands.
    /// Convert the operand from the register to a String, then to a &str so that we 
    /// can convert it to a usize so we can perform binary operations to it.
    fn decode(&mut self, instruction: u32, registers: &mut Registers) -> SimResult<(), String> {
        let mut instString: String = instruction.to_string();
        let mut inststr: &str = &instString[..];
        let mut instbin: usize = 0;
        match usize::from_str_radix(inststr, 2) {
            Result::Err(e) => return SimResult::Err(e.to_string()),
            Result::Ok(f) => instbin = f,
        }
        // let mut instbin = usize::from_str_radix(inststr, 2).unwrap();

        self.dest = (instbin << 13) >> 31;

        self.op2 = (instbin << 23) >> 31;

        self.src_reg = (instbin << 18) >> 31;

        let mut op1String = registers[self.src_reg].to_string();
        let mut op1str: &str = &op1String[..];

        match usize::from_str_radix(op1str, 2) {
            Result::Err(e) => return SimResult::Err(e.to_string()),
            Result::Ok(f) => self.op1 = f,
        }
        // self.op1 = usize::from_str_radix(op1str, 2).unwrap();
        
        return SimResult::Wait(0, ());
    }

    /// Execute the binary operation using usize's function checked_add().
    /// Store value in result field.
    fn execute(&mut self) -> SimResult<(), String> {
        match self.op1.checked_add(self.op2) {
            None => return SimResult::Err("Error".to_string()),
            Some(f) => self.result = f as u32,
        }
        // self.result = self.op1.checked_add(self.op2).unwrap() as u32;
        return SimResult::Wait(0, ());
    }

    /// Skipped, no memory accessing.
    fn access_memory(&mut self, memory: &mut dyn Memory<u32, u32>) -> SimResult<(), String> {
        return SimResult::Wait(0, ());
    }

    /// Store the value of the result in the destination register.
    fn write_back(&mut self, registers: &mut Registers) -> SimResult<(), String> {
        registers[self.dest] = self.result;
        return SimResult::Wait(0, ());
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use mockers::Scenario;
    
    /// Ensures that the load instruction functions correctly.
    #[test]
    fn test_load_instruction() {
        let scenario = Scenario::new();
        let (mut memory, memory_handle) = scenario.create_mock_for::<dyn Memory<u32, u32>>();
        let mut regs = Registers::new();
        
        // Setup registers
        const DEST_VAL: usize = 444;
        const ADDR_VAL: u32 = 777;
        regs[ADDR_REG_IDX] = ADDR_VAL;

        // Pack instruction bits
        // dest = 10100 = R20
        // addr = 00110 = R6
        const DEST_REG_IDX: usize = 20;
        const ADDR_REG_IDX: usize = 6;

        // Setup memory
        const MEM_DELAY: u16 = 101;
        const MEM_VALUE: u32 = 567;
        
        // Test decode with register direct
        let mut load_instruction = Load::new(AddrMode::RegisterDirect);
        let mut INSTRUCTION_RD: u32 = 0;
        INSTRUCTION_RD.set_bits(10..=14, (DEST_REG_IDX as u32).get_bits(0..=4));
        INSTRUCTION_RD.set_bits(15..=19, (ADDR_REG_IDX as u32).get_bits(0..=4));
        
        assert_eq!(load_instruction.decode(INSTRUCTION_RD, &regs),
                   SimResult::Wait(0, ()),
                   "register direct, decode() == expected");
        assert_eq!(load_instruction.dest_reg, DEST_REG_IDX,
                   "register direct, .dest_reg == expected");
        assert_eq!(load_instruction.mem_addr, ADDR_VAL,
                   "register direct, .mem_addr == expected");

        // Test decode with immediate
        let mut INSTRUCTION_I: u32 = 0;
        INSTRUCTION_I.set_bits(10..=14, (DEST_REG_IDX as u32).get_bits(0..=4));
        INSTRUCTION_I.set_bits(15..=31, ADDR_VAL.get_bits(0..=16)
                               - (regs[PC] + 1));
        
        load_instruction = Load::new(AddrMode::Immediate);

        assert_eq!(load_instruction.decode(INSTRUCTION_I, &regs),
                   SimResult::Wait(0, ()),
                   "immediate, decode() == expected");
        assert_eq!(load_instruction.dest_reg, DEST_REG_IDX,
                   "immediate, .dest_reg == expected");
        assert_eq!(load_instruction.mem_addr, ADDR_VAL,
                   "immediate, .mem_addr == expected");

        // Test execute
        assert_eq!(load_instruction.execute(), SimResult::Wait(0, ()),
                   "execute() == expected");

        // Test access memory
        scenario.expect(memory_handle.get(ADDR_VAL)
                        .and_return(SimResult::Wait(MEM_DELAY, MEM_VALUE)));
        assert_eq!(load_instruction.access_memory(&mut memory),
                   SimResult::Wait(MEM_DELAY, ()), "access_memory() == expected");
        assert_eq!(load_instruction.value, MEM_VALUE,
                   ".value == expected");

        // Test write back
        let mut expected_wb_regs = regs.clone();
        expected_wb_regs[DEST_REG_IDX] = MEM_VALUE;
        
        assert_eq!(load_instruction.write_back(&mut regs),
                   SimResult::Wait(0, ()), "write_back() == expected");
        assert_eq!(regs, expected_wb_regs,
                   "regs == expected");
    }

    /// Ensures the store instruction functions properly
    #[test]
    fn test_store_instruction() {
        let scenario = Scenario::new();

        let (mut memory, memory_handle) = scenario.create_mock_for::<dyn Memory<u32, u32>>();
        
        let mut regs = Registers::new();
        let mut store_instruction = Store::new();

        // Pack instruction operands
        // src = 00101 = R5
        // addr = 01000 = R8
        const SRC_REG_IDX: usize = 5;
        const ADDR_REG_IDX: usize = 8;
        const instruction: u32 = ((SRC_REG_IDX << 9) | (ADDR_REG_IDX << 14)) as u32;

        // Setup registers
        const DEST_ADDR: u32 = 34567;
        const SRC_VAL: u32 = 346;

        regs[SRC_REG_IDX] = SRC_VAL;
        regs[ADDR_REG_IDX] = DEST_ADDR;

        // Test decode
        assert_eq!(store_instruction.decode(instruction, &regs),
                   SimResult::Wait(0, ()), "decode() == expected");
        assert_eq!(store_instruction.value, SRC_VAL, ".value == expected");
        assert_eq!(store_instruction.dest_addr, DEST_ADDR,
                   ".dest_addr == expected");

        // Test execute
        assert_eq!(store_instruction.execute(), SimResult::Wait(0, ()),
                   "execute() == expected");

        // Test access memory
        const MEM_DELAY: u16 = 45;
        scenario.expect(memory_handle.set(DEST_ADDR, SRC_VAL)
                        .and_return(SimResult::Wait(MEM_DELAY, ())));
        assert_eq!(store_instruction.access_memory(&mut memory),
                   SimResult::Wait(MEM_DELAY, ()));

        // Test write back
        let expected_wb_regs = regs.clone();
        
        assert_eq!(store_instruction.write_back(&mut regs),
                   SimResult::Wait(0,()), "write_back == expected");
        assert_eq!(regs, expected_wb_regs, "regs == expected");
    }
}
