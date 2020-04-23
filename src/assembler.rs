use crate::result::SimResult;
use crate::memory::{Memory,DRAM,Registers,PC};

use std::io::{Read,BufReader};
use std::fs::File;

trait assembler {
    pub fn read_from_file(file: File);
}

impl assembler {
    pub fn read_from_file(file_p: File) {
        let file = match File::open(file_p) {
            Ok(f) => f,
            Err(e) => {
                return Err(format!("Failed to open DRAM file \"{}\": {}",
                                   file_p, e));
            },
        };

        let mut reader = BufReader::new(file);
        let mut buf: [u8; 4] = [0; 4];

        loop {
            match reader.read(&mut buf) {
                Ok(bytes_read) => {
                    if bytes_read == 0 { // End of file
                        return Ok(());
                    } else if bytes_read != 4 { // Incorrect number of bytes read
                        return Err(format!("Read {} bytes from buffer but \
                                            expected 4 bytes",
                                           bytes_read));
                    }

                    let value: u32 = (buf[3] as u32) |
                        (buf[2] as u32) << 8 |
                        (buf[1] as u32) << 16 |
                        (buf[0] as u32) << 24;
                    
                    self.data.insert(addr, value);
                    addr += 1;
                },
                Err(e) => {
                    return Err(format!("Failed to read buffer: {}", e));
                },
            }
        }
    }
}