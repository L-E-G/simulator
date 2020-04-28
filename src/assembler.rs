extern crate clap;
use clap::{Arg, App};

use std::fs::{File};
use std::io::{Read,BufRead,BufReader};

mod instructions;
use instructions::{InstructionT,ALUOp};

/// A template for an instructions bit pattern.
struct InstructionTemplate {
    /// Assembly mnemonic
    mnemonic: &str,
    
    /// Instruction type bits. Only 2 least significant bits are used.
    itype: u32,

    /// Amount of least signficant bits of which are used from the
    /// operation field.
    num_operation_bits: u32,

    /// Operation code for instruction.
    operation: u32,

    /// Indicates the number of bits in each operand of the instruction.
    operands_lengths: Vec<u32>,
}

/// Number of bits used in the operation field of ALU instructions.
const NUM_ALU_OP_BITS: u32 = 6;

fn main() {
    // Define instruction mnemonics
    let mnemonics = vec![
        InstructionTemplate{
            mnemonic: "ADD",
            itype: InstructionT::ALU.value(),
            num_operation_bits: NUM_ALU_OP_BITS,
            operation: ALUOp::AddUnsignedI,
            operand_lengths: vec![5, 5, 9],
        },
    ];
    
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
            let tokens = line.split(" ");
            
            // Check for symbol
            let mut symbol: Some<String> = None;
            
            let first_char = line.chars().nth(0)
                .expect(&format!("No 0th character found for non-empty line {}",
                                line_num));

            if first_char != ' ' && first_char != '\t' {
                symbol = Some(tokens[0]);
            }

            // TODO: Store symbol in symbol table
            // TODO:  Find mnemonic in mnemonic table which matches mnemonic on line
            // TODO: Create a structure which represents contents of line
            // TODO: Build this structure for current line
        }

        line_num += 1;
    }
}
