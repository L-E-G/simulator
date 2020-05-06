use clap::{Arg, App};

use bit_field::BitField;

use std::fs::{File};
use std::io::{Read,Write,BufRead,BufReader,LineWriter};

// mod instructions;
// mod memory;
// mod result;
use crate::instructions::{InstructionT,ALUOp,MemoryOp,ControlOp,ConditionCodes};

pub struct Assembler {}

/// A template for an instructions bit pattern.
struct InstructionTemplate {
    /// Assembly mnemonic
    mnemonic: String,
    
    /// Instruction type bits. Only 2 least significant bits are used.
    itype: u32,

    /// Amount of least significant bits of which are used from the
    /// operation field.
    num_operation_bits: u32,

    /// Operation code for instruction if immediate.
    operationI: u32,

    /// Operation code for instruction if reg direct.
    operationRD: u32,

    /// Indicates the location of immediate value, should there be one
    immediate_idx: u32,
}

#[derive(Copy, Clone)]
struct InstructionDetails {
    condition: u32,
    itype: u32,
    operation: u32,
    imm_idx: u32,
    operand1: u32,
    operand2: u32,
    operand3: u32,
    /// Used only is there is a label associated with the instruction
    label: u32,
}

struct label {
    addr: u32,
    label: u32,
}

/// Number of bits used in the operation field of all instructions.
const NUM_ALU_OP_BITS: u32 = 6;
const NUM_MEMORY_OP_BITS: u32 = 3;
const NUM_CONTROL_OP_BITS: u32 = 3;
const NUM_GRAPHICS_OP_BITS: u32 = 2;
/// Indicates there is no immediate in instruction.
const NO_IMMEDIATE: u32 = 1111;
/// INdicates that there is no condition code.
const NO_CONDITION_CODE: u32 = 11111;
/// Used in the InstructionDetails struct to indicate that the operand field is not set.
const NOT_SET: u32 = 11111111;
/// Size of a reg addr.
const SIZE_OF_REG: u32 = 5;

// Gets the indexes of where the immediate value are
fn has_immediate(tokens: [&str; 4]) -> bool {
    
    for i in 0..tokens.len() {
        if tokens[i] != "" {   // Need to check if there in anything in that spot in the array
            if tokens[i][1..2] == "x".to_string() || tokens[i][1..2] == "b".to_string() || tokens[i][1..2] == "d".to_string() {
                return true;
            }
        }
    }
    return false
}

// gets the immediate value
fn from_immediate(token: &str) -> u32 {
    return match token[2..].parse::<u32>() {
        Err(e) => panic!("Failed to parse immediate value {}", e),
        Ok(l) => l,
    }
}

// gets the reg address
fn from_register(token: &str) -> u32 {
    return match token[1..].parse::<u32>() {
        Err(e) => panic!("Failed to parse reg dir value {}", e),
        Ok(l) => l,
    }
}

// converts a vector into an array, this is for borrow issues with vectors
fn to_array(tokens: Vec<&str>) -> [&str; 4] {
    let mut array = [""; 4];

    let mut i = 0;
    for t in tokens {
        if t == "" {
            continue;
        }
        else {
            array[i] = t;
            i += 1;
        }
    }
    return array;
}

fn tokens_length(tokens: [&str; 4]) -> u32 {
    let mut length = 0;
    for i in 0..4 {
        if tokens[i] != "" {length += 1;}
    }
    return length;
}

fn get_condition_code(mnemonic: &str) -> u32 {

    for i in 0..mnemonic.len() {
        if mnemonic[i..i+1] == "E".to_string() {
            return ConditionCodes::E.value();
        }
        if mnemonic[i..i+1] == "L".to_string() {
            if i+1 >= mnemonic.len() {
                return NO_CONDITION_CODE;
            }
            if mnemonic[i..i+2] == "T".to_string() {
                return ConditionCodes::LT.value();
            }
        }
        if mnemonic[i..i+1] == "G".to_string() {
            if i+1 >= mnemonic.len() {
                return NO_CONDITION_CODE;
            }
            if mnemonic[i..i+2] == "T".to_string() {
                return ConditionCodes::GT.value();
            }
        }
    }
    return NO_CONDITION_CODE;
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler{}
    }

    pub fn assembler(&self, content: Vec<&str>) -> Vec<u8>{
        // Define instruction mnemonics
        let mnemonics = vec![
            InstructionTemplate{
                mnemonic: "HALT".to_string(),
                itype: InstructionT::Control.value(),
                num_operation_bits: NUM_CONTROL_OP_BITS,
                operationI: NO_IMMEDIATE,
                operationRD: ControlOp::Halt.value(),
                immediate_idx: NO_IMMEDIATE,
            },
            InstructionTemplate{
                mnemonic: "STR".to_string(),
                itype: InstructionT::Memory.value(),
                num_operation_bits: NUM_MEMORY_OP_BITS,
                operationI: MemoryOp::StoreI.value(),
                operationRD: MemoryOp::StoreRD.value(),
                immediate_idx: 2,
            },
            InstructionTemplate{
                mnemonic: "LDR".to_string(),
                itype: InstructionT::Memory.value(),
                num_operation_bits: NUM_MEMORY_OP_BITS,
                operationI: MemoryOp::LoadI.value(),
                operationRD: MemoryOp::LoadRD.value(),
                immediate_idx: 2,
            },
            InstructionTemplate{
                mnemonic: "PUSH".to_string(),
                itype: InstructionT::Memory.value(),
                num_operation_bits: NUM_MEMORY_OP_BITS,
                operationI: NO_IMMEDIATE,
                operationRD: MemoryOp::Push.value(),
                immediate_idx: NO_IMMEDIATE,
            },
            InstructionTemplate{
                mnemonic: "POP".to_string(),
                itype: InstructionT::Memory.value(),
                num_operation_bits: NUM_MEMORY_OP_BITS,
                operationI: NO_IMMEDIATE,
                operationRD: MemoryOp::Pop.value(),
                immediate_idx: NO_IMMEDIATE,
            },
            InstructionTemplate{
                mnemonic: "JMP".to_string(),
                itype: InstructionT::Control.value(),
                num_operation_bits: NUM_CONTROL_OP_BITS,
                operationI: ControlOp::JmpI.value(),
                operationRD: ControlOp::JmpRD.value(),
                immediate_idx: 1,
            },
            InstructionTemplate{
                mnemonic: "JMPS".to_string(),
                itype: InstructionT::Control.value(),
                num_operation_bits: NUM_CONTROL_OP_BITS,
                operationI: ControlOp::JmpSI.value(),
                operationRD: ControlOp::JmpSRD.value(),
                immediate_idx: 1,
            },
            InstructionTemplate{
                mnemonic: "JMPGT".to_string(),
                itype: InstructionT::Control.value(),
                num_operation_bits: NUM_CONTROL_OP_BITS,
                operationI: ControlOp::JmpI.value(),
                operationRD: ControlOp::JmpRD.value(),
                immediate_idx: 1,
            },
            InstructionTemplate{
                mnemonic: "JMPLT".to_string(),
                itype: InstructionT::Control.value(),
                num_operation_bits: NUM_CONTROL_OP_BITS,
                operationI: ControlOp::JmpI.value(),
                operationRD: ControlOp::JmpRD.value(),
                immediate_idx: 1,
            },
            InstructionTemplate{
                mnemonic: "JMPE".to_string(),
                itype: InstructionT::Control.value(),
                num_operation_bits: NUM_CONTROL_OP_BITS,
                operationI: ControlOp::JmpI.value(),
                operationRD: ControlOp::JmpRD.value(),
                immediate_idx: 1,
            },
            InstructionTemplate{
                mnemonic: "JMPI".to_string(),
                itype: InstructionT::Control.value(),
                num_operation_bits: NUM_CONTROL_OP_BITS,
                operationI: NO_IMMEDIATE,
                operationRD: ControlOp::RFI.value(),
                immediate_idx: NO_IMMEDIATE,
            },
            InstructionTemplate{
                mnemonic: "ADDU".to_string(),
                itype: InstructionT::ALU.value(),
                num_operation_bits: NUM_ALU_OP_BITS,
                operationI: ALUOp::AddUII.value(),
                operationRD: ALUOp::AddUIRD.value(),
                immediate_idx: 3,
            },
            InstructionTemplate{
                mnemonic: "ADDS".to_string(),
                itype: InstructionT::ALU.value(),
                num_operation_bits: NUM_ALU_OP_BITS,
                operationI: ALUOp::AddSII.value(),
                operationRD: ALUOp::AddSIRD.value(),
                immediate_idx: 3,
            },
            InstructionTemplate{
                mnemonic: "SUBU".to_string(),
                itype: InstructionT::ALU.value(),
                num_operation_bits: NUM_ALU_OP_BITS,
                operationI: ALUOp::SubUII.value(),
                operationRD: ALUOp::SubUIRD.value(),
                immediate_idx: 3,
            },
            InstructionTemplate{
                mnemonic: "SUBS".to_string(),
                itype: InstructionT::ALU.value(),
                num_operation_bits: NUM_ALU_OP_BITS,
                operationI: ALUOp::SubSII.value(),
                operationRD: ALUOp::SubSIRD.value(),
                immediate_idx: 3,
            },
            InstructionTemplate{
                mnemonic: "MULU".to_string(),
                itype: InstructionT::ALU.value(),
                num_operation_bits: NUM_ALU_OP_BITS,
                operationI: ALUOp::MulUII.value(),
                operationRD: ALUOp::MulUIRD.value(),
                immediate_idx: 3,
            },
            InstructionTemplate{
                mnemonic: "MULS".to_string(),
                itype: InstructionT::ALU.value(),
                num_operation_bits: NUM_ALU_OP_BITS,
                operationI: ALUOp::MulSII.value(),
                operationRD: ALUOp::MulSIRD.value(),
                immediate_idx: 3,
            },
            InstructionTemplate{
                mnemonic: "DIVU".to_string(),
                itype: InstructionT::ALU.value(),
                num_operation_bits: NUM_ALU_OP_BITS,
                operationI: ALUOp::DivUII.value(),
                operationRD: ALUOp::DivUIRD.value(),
                immediate_idx: 3,
            },
            InstructionTemplate{
                mnemonic: "DIVS".to_string(),
                itype: InstructionT::ALU.value(),
                num_operation_bits: NUM_ALU_OP_BITS,
                operationI: ALUOp::DivSII.value(),
                operationRD: ALUOp::DivSIRD.value(),
                immediate_idx: 3,
            },
            InstructionTemplate{
                mnemonic: "MV".to_string(),
                itype: InstructionT::ALU.value(),
                num_operation_bits: NUM_ALU_OP_BITS,
                operationI: NO_IMMEDIATE,
                operationRD: ALUOp::Move.value(),
                immediate_idx: NO_IMMEDIATE,
            },
            InstructionTemplate{
                mnemonic: "CMP".to_string(),
                itype: InstructionT::ALU.value(),
                num_operation_bits: NUM_ALU_OP_BITS,
                operationI: NO_IMMEDIATE,
                operationRD: ALUOp::Comp.value(),
                immediate_idx: NO_IMMEDIATE,
            },
            InstructionTemplate{
                mnemonic: "ASL".to_string(),
                itype: InstructionT::ALU.value(),
                num_operation_bits: NUM_ALU_OP_BITS,
                operationI: ALUOp::ASLI.value(),
                operationRD: ALUOp::ASLRD.value(),
                immediate_idx: 2,
            },
            InstructionTemplate{
                mnemonic: "ASR".to_string(),
                itype: InstructionT::ALU.value(),
                num_operation_bits: NUM_ALU_OP_BITS,
                operationI: ALUOp::ASRI.value(),
                operationRD: ALUOp::ASRRD.value(),
                immediate_idx: 2,
            },
            InstructionTemplate{
                mnemonic: "LSL".to_string(),
                itype: InstructionT::ALU.value(),
                num_operation_bits: NUM_ALU_OP_BITS,
                operationI: ALUOp::LSLI.value(),
                operationRD: ALUOp::LSRRD.value(),
                immediate_idx: 2,
            },
            InstructionTemplate{
                mnemonic: "AND".to_string(),
                itype: InstructionT::ALU.value(),
                num_operation_bits: NUM_ALU_OP_BITS,
                operationI: ALUOp::AndI.value(),
                operationRD: ALUOp::AndRD.value(),
                immediate_idx: 3,
            },
            InstructionTemplate{
                mnemonic: "OR".to_string(),
                itype: InstructionT::ALU.value(),
                num_operation_bits: NUM_ALU_OP_BITS,
                operationI: ALUOp::OrI.value(),
                operationRD: ALUOp::OrRD.value(),
                immediate_idx: 3,
            },
            InstructionTemplate{
                mnemonic: "XOR".to_string(),
                itype: InstructionT::ALU.value(),
                num_operation_bits: NUM_ALU_OP_BITS,
                operationI: ALUOp::XorI.value(),
                operationRD: ALUOp::XorRD.value(),
                immediate_idx: 3,
            },
            InstructionTemplate{
                mnemonic: "NOT".to_string(),
                itype: InstructionT::ALU.value(),
                num_operation_bits: NUM_ALU_OP_BITS,
                operationI: NO_IMMEDIATE,
                operationRD: ALUOp::Not.value(),
                immediate_idx: NO_IMMEDIATE,
            },
        ];
        
        let mut first_pass = Vec::new();
        let mut labels = Vec::new();
        
        // Parse command line arguments
        // let app = App::new("LEG assembler")
        //     .about("Converts LEG assembly into the LEG binary format")
        //     .arg(Arg::with_name("IN_ASSEMBLY")
        //          .short("i")
        //          .long("in")
        //          .help("Input LEG assembly file")
        //          .takes_value(true)
        //          .required(true))
        //     .arg(Arg::with_name("OUT_BINARY")
        //          .short("o")
        //          .long("out")
        //          .help("Output LEG binary file")
        //          .takes_value(true)
        //          .required(true))
        //     .get_matches();

        // let in_assembly_path = app.value_of("IN_ASSEMBLY").unwrap();
        // let out_binary_path = app.value_of("OUT_BINARY").unwrap();




        // // Read assembly file
        // let in_assembly_f = match File::open(format!("test-data/{}",file)) {
        //     Err(e) => panic!("Failed to open input assembly file: {}", e),
        //     Ok(f) => f,
        // };
        // let in_assembly_buf = BufReader::new(in_assembly_f);




        let mut index = 0;
        for line in content {
            // let line = match in_line {
            //     Err(e) => panic!("Failed to read line number {}: {}",index + 1,e).unwrap(),
            //     Ok(l) => l,
            // };

            if line.len() > 0 {
                // Parse the line
                let mut tokens_vec: Vec<&str> = line.split(" ").collect();

                // get first character
                let first_char = line.chars().nth(0)
                    .expect(&format!("No 0th character found for non-empty line {}",index + 1));

                // identify if label
                if first_char != ' ' {
                    // Create unique identifier based on character's 
                    let label_arr = tokens_vec[0].as_bytes();
                    let mut label: u32 = 0;
                    for i in 0..label_arr.len() {
                        label += label_arr[i] as u32;
                    }
                    // save position and name of label
                    labels.push(label{
                        addr: index,
                        label: label,
                    });
                    
                    // Remove label and carry on as if this is a normal instruction
                    tokens_vec.remove(0);
                }
                
                // Convert to array for borrow issues, can't borrow with vectors
                let tokens = to_array(tokens_vec);

                let tokens_len: u32 = tokens_length(tokens);
                
                // Get immediate indexes if there are any
                let has_immediate = has_immediate(tokens);

                // Create instruction to be assigned later
                let mut inst = InstructionDetails{
                    condition: 0,
                    itype: 0,
                    operation: 0,
                    imm_idx: NOT_SET,
                    operand1: NOT_SET,
                    operand2: NOT_SET,
                    operand3: NOT_SET,
                    label: NOT_SET,
                };

                // Jump through template table to find the template that matches the current line
                for t in &mnemonics {
                    // Find correct template
                    if tokens[0] == t.mnemonic {
                        // Set initial values
                        inst.itype = t.itype;
                        inst.operation = t.operationRD;

                        // Check if includes any labels
                        if tokens[0][..3] == "JMP".to_string() && !has_immediate && tokens[1][0..1] != "R".to_string() {
                            // Compute label's unique value
                            let label_arr = tokens[1].as_bytes();
                            let mut label: u32 = 0;
                            for i in 0..label_arr.len() {
                                label += label_arr[i] as u32;
                            }

                            inst.operation = t.operationI;
                            inst.label = label;

                            let cond = get_condition_code(tokens[0]);

                            if cond != NO_CONDITION_CODE {
                                inst.condition = cond;
                            }
                            break;
                        }

                        // Check if there are no operands
                        if tokens_len == 1 {
                            break;
                        }
                        
                        // Loop though operands, set remaining values
                        let mut operand_index: u32 = 1;
                        for i in 1..tokens_len {
                            // Enter if immediate
                            if i == t.immediate_idx && has_immediate {
                                inst.imm_idx = t.immediate_idx;
                                inst.operation = t.operationI;
                                match operand_index {
                                    1 => inst.operand1 = from_immediate(tokens[i as usize]),
                                    2 => inst.operand2 = from_immediate(tokens[i as usize]),
                                    3 => inst.operand3 = from_immediate(tokens[i as usize]),
                                    _ => panic!("Failed to assign operand immediate value"),
                                }
                                operand_index += 1;
                            } else {
                                match operand_index {
                                    1 => inst.operand1 = from_register(tokens[i as usize]),
                                    2 => inst.operand2 = from_register(tokens[i as usize]),
                                    3 => inst.operand3 = from_register(tokens[i as usize]),
                                    _ => panic!("Failed to assign operand reg direct value"),
                                }
                                operand_index += 1;
                            }
                            
                        }
                        break;
                    }
                }
                // Push to first pass vector
                first_pass.push(inst);
            }
            index += 1;
        }
        
        // template:
        // let mut inst = InstructionDetails{
        //     condition: 0,
        //     itype: 0,
        //     operation: 0,
        //     imm_idx: 0,
        //     operand1: 0,
        //     operand2: 0,
        //     operand3: 0,
        //     label: 0,
        // };


        // let file = match File::create(format!("test-data/{}.bin",file)) {
        //     Err(e) => panic!("Failed to open file to write binary instructions: {}", e),
        //     Ok(f) => f,
        // };
        // let mut writer = LineWriter::new(file);

        let mut bin_insts = Vec::new();

        //Second pass
        let mut index = 0;
        for inst in first_pass {
            let mut inst_pos: u32 = 0;
            let mut instruction: u32 = 0;
        
            // Set condition code
            instruction.set_bits(inst_pos as usize..=(inst_pos+4) as usize, 
                inst.condition.get_bits(0..=4) as u32);
            inst_pos += 5;

            // Set instruction type
            instruction.set_bits(inst_pos as usize..=(inst_pos+1) as usize, 
                inst.itype.get_bits(0..=1));
            inst_pos += 2;
            
            // Set operation code
            if InstructionT::ALU.value() == inst.itype {
                instruction.set_bits(inst_pos as usize..=(inst_pos + NUM_ALU_OP_BITS) as usize, 
                    inst.operation.get_bits(0..=NUM_ALU_OP_BITS as usize));
                inst_pos += NUM_ALU_OP_BITS;
            } else if InstructionT::Memory.value() == inst.itype {
                instruction.set_bits(inst_pos as usize..=(inst_pos + NUM_MEMORY_OP_BITS) as usize, 
                    inst.operation.get_bits(0..=NUM_MEMORY_OP_BITS as usize));
                inst_pos += NUM_MEMORY_OP_BITS;
            } else if InstructionT::Control.value() == inst.itype {
                instruction.set_bits(inst_pos as usize..=(inst_pos + NUM_CONTROL_OP_BITS) as usize, 
                    inst.operation.get_bits(0..=NUM_CONTROL_OP_BITS as usize));
                inst_pos += NUM_CONTROL_OP_BITS;
            } else if InstructionT::Graphics.value() == inst.itype {
                instruction.set_bits(inst_pos as usize..=(inst_pos + NUM_GRAPHICS_OP_BITS) as usize, 
                    inst.operation.get_bits(0..=NUM_GRAPHICS_OP_BITS as usize));
                inst_pos += NUM_GRAPHICS_OP_BITS;
            }
            
            if inst.label != NOT_SET {
                for l in &labels {
                    if l.label == inst.label {
                        instruction.set_bits(inst_pos as usize..=(32 - inst_pos) as usize, 
                            l.addr.get_bits(0..=(32 - inst_pos) as usize));
                    }
                }
            }
            
            // Set first operand (if applicable)
            if inst.operand1 != NOT_SET {
                if index == inst.imm_idx {
                    instruction.set_bits(inst_pos as usize..=(32 - inst_pos) as usize, 
                        inst.operand1.get_bits(0..=(32 - inst_pos) as usize));
                }else {
                    instruction.set_bits(inst_pos as usize..=(inst_pos + SIZE_OF_REG) as usize, 
                        inst.operand1.get_bits(0..=SIZE_OF_REG as usize));
                    inst_pos += SIZE_OF_REG;
                }
            }

            //Set second operand (if applicable)
            if inst.operand2 != NOT_SET {
                if index == inst.imm_idx {
                    instruction.set_bits(inst_pos as usize..=(32 - inst_pos) as usize, 
                        inst.operand2.get_bits(0..=(32 - inst_pos) as usize));
                }else {
                    instruction.set_bits(inst_pos as usize..=(inst_pos + SIZE_OF_REG) as usize, 
                        inst.operand2.get_bits(0..=SIZE_OF_REG as usize));
                    inst_pos += SIZE_OF_REG;
                }
            }

            // Set third operand (if applicable)
            if inst.operand3 != NOT_SET {
                if index == inst.imm_idx {
                    instruction.set_bits(inst_pos as usize..=(32 - inst_pos) as usize, 
                        inst.operand3.get_bits(0..=(32 - inst_pos) as usize));
                }
                instruction.set_bits(inst_pos as usize..=(inst_pos + SIZE_OF_REG) as usize, 
                    inst.operand3.get_bits(0..=SIZE_OF_REG as usize));
            }

            // Convert to [u8] for .write()
            let b1: u8 = ((instruction >> 24) & 0xff) as u8;
            bin_insts.push(b1);
            let b2: u8 = ((instruction >> 16) & 0xff) as u8;
            bin_insts.push(b2);
            let b3: u8 = ((instruction >> 8) & 0xff) as u8;
            bin_insts.push(b3);
            let b4: u8 = (instruction & 0xff) as u8;
            bin_insts.push(b4);
            // let write_val: [u8;4] = [b1,b2,b3,b4];

            // bin_insts.push(write_val);


            // // Write to bin file, this takes an &[u8] value
            // writer.write(&write_val).expect("Not able to write binary instruction");

            index += 1;
        } 
        // panic!("Rob");
        return bin_insts;
    }
}

pub fn load_data () {

}

#[cfg(test)]
mod tests {
    use super::*;
    use mockers::Scenario;
    
    #[test]
    fn test_assembler() {
        let mut assem = Assembler::new();
        let data_vec: Vec<&str> = vec!["LDR R6 R4","ADDU R4 R18 R2"];
        assem.assembler(data_vec);
    }
}