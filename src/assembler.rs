extern crate clap;
use clap::{Arg, App};

use std::fs::{File};
use std::io::{Read,BufRead,BufReader};

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

    /// Operation code for instruction.
    operationI: u32,

    operationRD: u32,

    /// Indicates the location of immediate value, should there be one
    immediate_idx: u32,
}

#[derive(Copy, Clone)]
struct InstructionDetails {
    condition: usize,
    itype: u32,
    operation: u32,
    operand1: u32,
    operand2: u32,
    operand3: u32
}

/// Number of bits used in the operation field of ALU instructions.
const NUM_ALU_OP_BITS: u32 = 6;
const NO_IMMEDIATE: u32 = 111;

fn get_immediate_index(tokens: [&str; 4]) -> u32 {
    for i in 0..tokens.len() {
        if tokens[i][1..2] == "x".to_string() || tokens[i][1..2] == "b".to_string() || tokens[i][1..2] == "d".to_string() {
            return i as u32;
        }
    }
    return NO_IMMEDIATE
}

fn from_immediate(token: &str) -> u32 {
    return token[2..].parse::<u32>().unwrap();
}

fn from_register(token: &str) -> u32 {
    return token.parse::<u32>().unwrap();
}

fn homemade_split(line: &str) -> Vec<String> {
    let mut vec = Vec::new();
    let line_split: Vec<&str> = line.split(" ").collect();
    for tok in line_split {
        vec.push(tok.to_string());
    }
    return vec
}

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
            // let tokens: Vec<String> = homemade_split(&line);
            let tokens_vec: Vec<&str> = line.split(" ").collect();

            let tokens = to_array(tokens_vec);
            
            let first_char = line.chars().nth(0)
                .expect(&format!("No 0th character found for non-empty line {}",line_num));

            if first_char != ' ' && first_char != '\t' {
                panic!("Invalid instruction");
            }

            // let condition, mnemonic = fn_which_extract_condition_codes_from_end_of_mnemonics(token[1]);
            let immediate_idxs = get_immediate_index(tokens);

            let mut template_opt: Option<InstructionTemplate> = None;

            let mut inst = InstructionDetails{
                condition: 0,
                itype: 0,
                operation: 0,
                operand1: 0,
                operand2: 0,
                operand3: 0,
            };

            for t in &mnemonics {
                if tokens[0] == t.mnemonic && t.immediate_idx == immediate_idxs {
                    // template_opt = Some(t);

                    inst.itype = t.itype;
                    inst.operation = t.operationRD;

                    let mut operand_index = 1;
                    for i in 0..tokens.len() {
                        if i as u32 == immediate_idxs {
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

            first_pass.push(inst)

            // TODO: Store symbol in symbol table
            // TODO:  Find mnemonic in mnemonic table which matches mnemonic on line
            // TODO: Create a structure which represents contents of line
            // TODO: Build this structure for current line
        }
        line_num += 1;
    }
    
    for inst in first_pass {
        
    } 

}
