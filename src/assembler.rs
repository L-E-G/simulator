extern crate clap;
use clap::{Arg, App};

use bit_field::BitField;

use std::fs::{File};
use std::io::{Read,Write,BufRead,BufReader,LineWriter};

mod instructions;
mod memory;
mod result;
use instructions::{InstructionT,ALUOp};

/// A template for an instructions bit pattern.
struct InstructionTemplate {
    /// Assembly mnemonic
    mnemonic: String,
    
    /// Instruction type bits. Only 2 least significant bits are used.
    itype: u32,

    /// Amount of least signficant bits of which are used from the
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
    condition: usize,
    itype: u32,
    operation: u32,
    imm_idx: u32,
    operand1: u32,
    operand2: u32,
    operand3: u32
}

/// Number of bits used in the operation field of ALU instructions.
const NUM_ALU_OP_BITS: u32 = 6;
/// Indicates there is no immediate in instruction.
const NO_IMMEDIATE: u32 = 111;
/// Used in the InstructionDetails struct to indicate that the operand field is not set.
/// Used to tell how many operands there are in the instruction
const NOT_SET: u32 = 11111111;
/// Size of a reg addr.
const SIZE_OF_REG: u32 = 5;

// Gets the indexes of where the immediate value are
fn get_immediate_index(tokens: [&str; 4]) -> u32 {
    for i in 0..tokens.len() {
        if tokens[i][1..2] == "x".to_string() || tokens[i][1..2] == "b".to_string() || tokens[i][1..2] == "d".to_string() {
            return i as u32;
        }
    }
    return NO_IMMEDIATE
}

// gets the immediate value
fn from_immediate(token: &str) -> u32 {
    return token[2..].parse::<u32>().unwrap();
}

// gets the reg address
fn from_register(token: &str) -> u32 {
    return token.parse::<u32>().unwrap();
}

// converts a vector into an array, this is for borrow issues with vectors
fn to_array(tokens: Vec<&str>) -> [&str; 4] {
    let mut array = [""; 4];

    let mut index = 0;
    for tok in tokens {
        array[index] = tok;
        index += 1;
    }
    return array;
}

fn main() {
    // Define instruction mnemonics
    let mnemonics = vec![
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
        // InstructionTemplate{
        //     mnemonic: "SUBU",
        //     itype: InstructionT::ALU.value(),
        //     num_operation_bits: NUM_ALU_OP_BITS,
        //     operation: ALUOp::SubUIRD,
        // },
        // InstructionTemplate{
        //     mnemonic: "SUBS",
        //     itype: InstructionT::ALU.value(),
        //     num_operation_bits: NUM_ALU_OP_BITS,
        //     operation: ALUOp::SubSIRD,
        // },
        // InstructionTemplate{
        //     mnemonic: "MULU",
        //     itype: InstructionT::ALU.value(),
        //     num_operation_bits: NUM_ALU_OP_BITS,
        //     operation: ALUOp::MulUIRD,
        // },
        // InstructionTemplate{
        //     mnemonic: "MULS",
        //     itype: InstructionT::ALU.value(),
        //     num_operation_bits: NUM_ALU_OP_BITS,
        //     operation: ALUOp::MulUII,
        // },
        // InstructionTemplate{
        //     mnemonic: "MV",
        //     itype: InstructionT::ALU.value(),
        //     num_operation_bits: NUM_ALU_OP_BITS,
        //     operation: ALUOp::Move,
        // },
        // InstructionTemplate{
        //     mnemonic: "CMP",
        //     itype: InstructionT::ALU.value(),
        //     num_operation_bits: NUM_ALU_OP_BITS,
        //     operation: ALUOp::CompUI,
        // },
        // InstructionTemplate{
        //     mnemonic: "ASL",
        //     itype: InstructionT::ALU.value(),
        //     num_operation_bits: NUM_ALU_OP_BITS,
        //     operation: ALUOp::CompUI,
        // },
    ];
    
    let mut first_pass = Vec::new();
    
    // Parse command line arguments
    let app = App::new("LEG assembler")
        .about("Converts LEG assembly into the LEG binary format")
        .arg(Arg::with_name("IN_ASSEMBLY")
             .short("i")
             .long("in")
             .help("Input LEG assembly file")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("OUT_BINARY")
             .short("o")
             .long("out")
             .help("Output LEG binary file")
             .takes_value(true)
             .required(true))
        .get_matches();

    let in_assembly_path = app.value_of("IN_ASSEMBLY").unwrap();
    let out_binary_path = app.value_of("OUT_BINARY").unwrap();

    // Read assembly file
    let in_assembly_f = match File::open(in_assembly_path) {
        Err(e) => panic!("Failed to open input assembly file: {}", e),
        Ok(f) => f,
    };
    let in_assembly_buf = BufReader::new(in_assembly_f);

    let mut line_num = 1;
    for in_line in in_assembly_buf.lines() {
        let line = match in_line {
            Err(e) => panic!("Failed to read line number {}", line_num),
            Ok(l) => l,
        };

        if line.len() > 0 {
            // Parse the line
            let tokens_vec: Vec<&str> = line.split(" ").collect();

            // Convert to array for borrow issues, can't borrow with vectors
            let tokens = to_array(tokens_vec);
            
            // get first character
            let first_char = line.chars().nth(0)
                .expect(&format!("No 0th character found for non-empty line {}",line_num));

            // Check id valid line
            if first_char != ' ' && first_char != '\t' {
                panic!("Invalid instruction");
            }

            // let condition, mnemonic = fn_which_extract_condition_codes_from_end_of_mnemonics(token[1]);

            // Get immediate indexes if there are any
            let immediate_idxs = get_immediate_index(tokens);

            // Create instruction to be assigned later
            let mut inst = InstructionDetails{
                condition: 0,
                itype: 0,
                operation: 0,
                imm_idx: NOT_SET,
                operand1: NOT_SET,
                operand2: NOT_SET,
                operand3: NOT_SET,
            };

            // Jump through template table to find the template that matches the current line
            for t in &mnemonics {
                // Find correct template
                if tokens[0] == t.mnemonic {
                    
                    // Set initial values
                    inst.itype = t.itype;
                    inst.operation = t.operationRD;

                    // Loop though tokens, set remaining values
                    let mut operand_index = 1;
                    for i in 0..tokens.len() {
                        // Enter is immediate
                        if i as u32 == immediate_idxs {
                            inst.imm_idx = t.immediate_idx;
                            match operand_index {
                                1 => inst.operand1 = from_immediate(tokens[i]),
                                2 => inst.operand2 = from_immediate(tokens[i]),
                                3 => inst.operand3 = from_immediate(tokens[i]),
                                _ => panic!("Failed to assign operand immediate value"),
                            }
                            operand_index += 1;
                            inst.operation = t.operationI;
                        } else {
                            match operand_index {
                                1 => inst.operand1 = from_register(tokens[i]),
                                2 => inst.operand2 = from_register(tokens[i]),
                                3 => inst.operand3 = from_register(tokens[i]),
                                _ => panic!("Failed to assign operand reg direct value"),
                            }
                            operand_index += 1;
                        }
                    }
                }
            }

            // Push to first vector
            first_pass.push(inst)

            // TODO: Store symbol in symbol table
            // TODO:  Find mnemonic in mnemonic table which matches mnemonic on line
            // TODO: Create a structure which represents contents of line
            // TODO: Build this structure for current line
        }
        line_num += 1;
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
    // };

    let mut index = 0;
    for inst in first_pass {
        // Create file
        let file = match File::create("test-data/instructions.bin") {
            Err(e) => panic!("Failed to open file to write binary instructions: {}", e),
            Ok(f) => f,
        };

        let mut writer = LineWriter::new(file);

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
        // TODO: fix this to that it is not just doing the ALU bits
        instruction.set_bits(inst_pos as usize..=(inst_pos + NUM_ALU_OP_BITS) as usize, 
            inst.operation.get_bits(0..=NUM_ALU_OP_BITS as usize));
        inst_pos += NUM_ALU_OP_BITS+1;

        // Set first operand
        instruction.set_bits(inst_pos as usize..=(inst_pos + SIZE_OF_REG) as usize, 
            inst.operand1.get_bits(0..=SIZE_OF_REG as usize));
        inst_pos += SIZE_OF_REG+1;

        //Set second operand (if applicable)
        if inst.operand2 != NOT_SET {
            if index == inst.imm_idx {
                instruction.set_bits(inst_pos as usize..=(32 - inst_pos) as usize, 
                    inst.operand2.get_bits(0..=(32 - inst_pos) as usize));
            }
            instruction.set_bits(inst_pos as usize..=(inst_pos + SIZE_OF_REG) as usize, 
                inst.operand2.get_bits(0..=SIZE_OF_REG as usize));
            inst_pos += SIZE_OF_REG+1;
        }

        // Set third operand (if applicable)
        if inst.operand3 != NOT_SET {
            if index == inst.imm_idx {
                instruction.set_bits(inst_pos as usize..=(32 - inst_pos) as usize, 
                    inst.operand3.get_bits(0..=(32 - inst_pos) as usize));
            }
            instruction.set_bits(inst_pos as usize..=(inst_pos + SIZE_OF_REG) as usize, 
                inst.operand3.get_bits(0..=SIZE_OF_REG as usize));
            inst_pos += SIZE_OF_REG+1;
        }

        // Convert to [u8] for .write_all()
        let b1: u8 = ((instruction >> 24) & 0xff) as u8;
        let b2: u8 = ((instruction >> 16) & 0xff) as u8;
        let b3: u8 = ((instruction >> 8) & 0xff) as u8;
        let b4: u8 = (instruction & 0xff) as u8;
        let write_val: [u8;4] = [b1,b2,b3,b4];

        // Write to bin file
        writer.write_all(&write_val);

        index += 1;
    } 

}
