use std::cell::RefCell;
use std::rc::Rc;

use crate::result::SimResult;
use crate::memory::{Memory,Registers};

/// Defines operations which a single instruction must perform while it is in
/// the pipeline.
pub trait Instruction {
    /// Extracts parameters from instruction bits and stores them in the
    /// implementing struct for use by future stages. It also retrieves register
    /// values if necessary and does the same.
    fn decode_and_fetch(&mut self, instruction: u32, registers: &mut Registers) -> SimResult<(), String>;

    /// Executes the instruction.
    fn execute(&mut self) -> SimResult<(), String>;

    /// Accesses memory.
    fn access_memory(&mut self, memory: Rc<RefCell<dyn Memory<u32, u32>>>) -> SimResult<(), String>;

    /// Write results to registers.
    fn write_back(&mut self, registers: &mut Registers) -> SimResult<(), String>;
}

/// Read a value from an address in memory and place it in a register.
pub struct Load {
    /// Register to place value from memory.
    dest_reg: usize,

    /// Memory address to load into register.
    mem_addr: u32,

    /// Value loaded from mememory during access_memory.
    value: u32,
}

impl Load {
    pub fn new() -> Load {
        Load{
            dest_reg: 0,
            mem_addr: 0,
            value: 0,
        }
    }
}

impl Instruction for Load {
    /// Extract dest_reg and mem_addr operands.
    fn decode_and_fetch(&mut self, instruction: u32, registers: &mut Registers) -> SimResult<(), String> {
        self.dest_reg = (((instruction) << 17) >> 26) as usize;
        self.mem_addr = registers[((instruction << 12) >> 26) as usize];
        
        return SimResult::Wait(0, ());
    }

    /// No execute step.
    fn execute(&mut self) -> SimResult<(), String> {
        return SimResult::Wait(0, ());
    }

    /// Load value at mem_addr from memory into value.
    fn access_memory(&mut self, memory: Rc<RefCell<dyn Memory<u32, u32>>>) -> SimResult<(), String> {
        match memory.borrow_mut().get(self.mem_addr) {
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

struct Store {
    src_reg: usize,
    addr: u32,
    value: u32,
}

impl Store {
    pub fn new() -> Store {
        Store{
            src_reg: 0,
            addr: 0,
            value: 0,
        }
    }
}

impl Instruction for Store {
    /// Convert instruction to String, then to &str so we can convert it to a usize
    /// so that we can perform binary operations on it.
    /// Extract DRAM address from instruction.
    /// Extract register from instruction.
    /// Get value to be added to DRAM from register.
    fn decode_and_fetch(&mut self, instruction: u32, registers: &mut Registers) -> SimResult<(), String> {
        let mut instString: String = instruction.to_string();
        let mut inststr: &str = &instString[..];
        let mut instbin: usize = 0;
        match usize::from_str_radix(inststr, 2) {
            Result::Err(e) => return SimResult::Err(e.to_string()),
            Result::Ok(f) => instbin = f,
        }
        // let mut instbin = usize::from_str_radix(inststr, 2).unwrap();

        self.addr = ((instbin << 14) >> 31) as u32;

        self.src_reg = (instbin << 9) >> 31;

        self.value = registers[self.src_reg];
        return SimResult::Wait(0, ());
    }

    /// Skip, will not need this since we are only accessing memory.
    fn execute(&mut self) -> SimResult<(), String> {
        return SimResult::Wait(0, ());
    }

    /// Call set from DRAM witht he value we want to add to DRAM. 
    fn access_memory(&mut self, memory: Rc<RefCell<dyn Memory<u32, u32>>>) -> SimResult<(), String> {
        let mut wait = 0;
 
        match memory.borrow_mut().set(self.addr, self.value) {
            SimResult::Err(e) => return SimResult::Err(e),
            SimResult::Wait(c, _v) => {
                wait += c;

            }
        };
        return SimResult::Wait(wait, ());
    }

    /// Skip, no write back to register.
    fn write_back(&mut self, registers: &mut Registers) -> SimResult<(), String> {
        return SimResult::Wait(0, ());
    }
}


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
    fn decode_and_fetch(&mut self, instruction: u32, registers: &mut Registers) -> SimResult<(), String> {
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
    fn access_memory(&mut self, memory: Rc<RefCell<dyn Memory<u32, u32>>>) -> SimResult<(), String> {
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
    fn decode_and_fetch(&mut self, instruction: u32, registers: &mut Registers) -> SimResult<(), String> {
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
    fn access_memory(&mut self, memory: Rc<RefCell<dyn Memory<u32, u32>>>) -> SimResult<(), String> {
        return SimResult::Wait(0, ());
    }

    /// Store the value of the result in the destination register.
    fn write_back(&mut self, registers: &mut Registers) -> SimResult<(), String> {
        registers[self.dest] = self.result;
        return SimResult::Wait(0, ());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Ensures that the load instruction functions correctly.
    #[test]
    fn test_load_instruction() {
        // Pack instruction bits
        // dest = 10100 = R20
        // addr = 00110 = R6
        const DEST_REG_IDX: usize = 20;
        const ADDR_REG_IDX: usize = 6;
        const instruction: u32 = ((ADDR_REG_IDX as u32) << 14) | ((DEST_REG_IDX as u32) << 9);

        // Setup registers
        let mut regs = Registers::new();

        const DEST_VAL: usize = 444;
        const ADDR_VAL: u32 = 777;
        regs[DEST_REG_IDX] = DEST_VAL as u32;
        regs[ADDR_REG_IDX] = ADDR_VAL;

        // Decode and fetch
        let mut load_instruction = Load::new();
        
        assert_eq!(load_instruction.decode_and_fetch(instruction, &mut regs),
                   SimResult::Wait(0, ()), "decode_and_fetch result");
        assert_eq!(load_instruction.dest_reg, DEST_REG_IDX,
                   "instruction.dest_reg == expected");
        assert_eq!(load_instruction.mem_addr, ADDR_VAL,
                   "instruction.mem_addr == expected");

        // Execute
        assert_eq!(load_instruction.execute(), SimResult::Wait(0, ()),
                   "execute result");

        // TODO: Use mock library to test if correct memory calls are made
        // TODO: Test rest of Load instruction
    }
}
