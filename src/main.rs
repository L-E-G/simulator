extern crate ux;
extern crate text_io;

use std::io;
use std::process::exit;
use std::cell::RefCell;
use std::rc::Rc;
use std::path::Path;
use std::fs::{File, remove_file};
use std::io::{Write, BufReader, BufRead, LineWriter};
use std::borrow::BorrowMut;

use ux::{u22};
use text_io::scan;

/// Indicates the status of a simulator operation with either a value, error, or
/// result which will be available after a delay. D is the data type, E is the
/// error type.
enum SimResult<D, E> {
    /// Value if operation was successful.
    Ok(D),

    /// Error if operation failed.
    Err(E),

    /// Indicates result is not available yet. First field is the number of
    /// simulator cycles before the value will be ready. The result will be
    /// available during the simulator cycle in which this field reaches 0. The
    /// second field is the result. This result can be OK, Err, or even Wait.
    Wait(u16, D),
}

impl<D, E> SimResult<D, E> {
    /// Decrements the cycle count in a Wait. If this field equals 1 after
    /// decrementing then the contained result is returned. The contained result is
    /// returned when the count equals 1, not 0, because this method is expected to
    /// be called once every cycle. The value which is returned should be processed
    /// in the next cycle, when the count would equal 0.
    ///
    /// Otherwise a Wait with a decremented cycle count is returned. If the
    /// SimResult is Ok or Err just returns, method should not be used on these.
    fn process(self) -> SimResult<D, E> {
        match self {
            SimResult::Ok(v) => SimResult::Ok(v),
            SimResult::Err(e) => SimResult::Err(e),
            SimResult::Wait(i, res) => {
                if i <= 2 {
                    return SimResult::Ok(res);
                }

                SimResult::Wait(i-1, res)
            },
        }
    }
}


//----------------------------------------------Cache-------------------------------------------------



//-----------------------------DRAM---------------------------------

// Memory, A is the address type, D is the data type.
trait Memory<A, D> {
    fn get(&mut self, address: A) -> SimResult<D, String>;
    fn set(&mut self, address: A, data: D) -> SimResult<(), String>;
}

// InspectableMemory allows a memory unit to be insepcted for user
// interface purposes. A is the address type.
trait InspectableMemory<A> {
    // Returns a text description of an address.
    fn inspect_address_txt(&self, address: A) -> Result<String, String>;
    fn get(&self, address: A) -> u32;
}

struct DRAM {
    delay: u16,
    data: File,
}

impl InspectableMemory<u32> for DRAM {
    fn inspect_address_txt(&self, address: u32) -> Result<String, String> {
        let d = InspectableMemory::get(self, address);
        if d == 0{
            return Ok(format!("Does not exist"));
        }
        else{
            return Ok(format!("\
Address: {}
Value  : {}", address, d));
        }
    }

    fn get(&self, address: u32) -> u32 {
        let file: File = File::open("src/dram").unwrap();
        let reader = BufReader::new(file);
        let mut tag: u32 = (address >> 10) << 10;
        for (index, line) in reader.lines().enumerate(){
            let line = line.unwrap();
            let items_in_line: Vec<&str> = line.split(" ").collect();
            let data0 = items_in_line[0].parse::<u32>().unwrap();
            let data1 = items_in_line[1].parse::<u32>().unwrap();
            let data2 = items_in_line[2].parse::<u32>().unwrap();
            if tag == data0 && data1 == 1{
                return data2;
            }
        }

        return 0;
    }
}

impl DRAM {
    fn new(delay: u16) -> DRAM {
        DRAM{
            delay: delay,
            data: DRAM::create(),
        }
    }

    fn create() -> File {
        let disk_file_path = Path::new("src/dram");
        let mut file: File;
        if !disk_file_path.exists() {
            file = File::create(disk_file_path).expect("Failed to create");
            file.write_all("0 1 0\n".as_bytes()).expect("Bad write");
        }else{
            file = File::open(disk_file_path).expect("Failed to open");
        }
        return file;
    }
}

impl Memory<u32, u32> for DRAM {
    fn get(&mut self, address: u32) -> SimResult<u32, String> {
        let file: File = File::open("src/dram").unwrap();
        let reader = BufReader::new(file);
        let mut tag: u32 = (address >> 10) << 10;
        for (index, line) in reader.lines().enumerate(){
            let line = line.unwrap();
            let items_in_line: Vec<&str> = line.split(" ").collect();
            let data0 = items_in_line[0].parse::<u32>().unwrap();
            let data1 = items_in_line[1].parse::<u32>().unwrap();
            let data2 = items_in_line[2].parse::<u32>().unwrap();
            if tag == data0 && data1 == 1{
                return SimResult::Wait(self.delay, data2);
            }
        }

        return SimResult::Wait(self.delay, 0);

    }
    
    fn set(&mut self, address: u32, data: u32) -> SimResult<(), String> {

        let file = File::open("src/dram").unwrap();
        let reader = BufReader::new(file);
        remove_file("src/dram");
        let file = File::create("src/dram").unwrap();
        let mut writer = LineWriter::new(file);
        let mut tag: u32 = (address >> 10) << 10;

        let mut i=0;
        for (index, line) in reader.lines().enumerate(){
            let line = line.unwrap();
            let items_in_line: Vec<&str> = line.split(" ").collect();
            let tagFromLine = items_in_line[0].parse::<u32>().unwrap();
            let valid = items_in_line[1].parse::<u32>().unwrap();
            let value = items_in_line[2].parse::<u32>().unwrap();
            if tag < tagFromLine && valid == 1 && i==0{
                writer.write_all(format!("{} 1 {}\n",tag.to_string(), data.to_string()).as_bytes());
                i=1;
            }
            if tag == tagFromLine && valid == 1 && i==0{
                writer.write_all(format!("{} 1 {}\n",tagFromLine.to_string(), data.to_string()).as_bytes());
                i=1;
                continue;
            }
            writer.write_all(format!("{} {} {}\n",tagFromLine.to_string(), valid.to_string(), value.to_string()).as_bytes());
        }
        if i == 0{
            writer.write_all(format!("{} 1 {}\n",tag.to_string(), data.to_string()).as_bytes());
        }
        return SimResult::Wait(self.delay, ());
    }
}


//----------------------L1, L2, L3 Cache----------------------

const DM_CACHE_LINES: usize = 1024;

// Direct mapped cache.
// 10 least significant bits of an address are the index.
// 22 most significant bits of an address are the tag.
struct DMCache {
    delay: u16,
    lines: [DMCacheLine; DM_CACHE_LINES],
    base: Rc<RefCell<dyn Memory<u32, u32>>>,
//    base: &'a mut dyn Memory<u32, u32>,
}

#[derive(Copy,Clone)]
struct DMCacheLine {
    tag: u22,
    data: u32,
    valid: bool,
    dirty: bool,
}

impl DMCacheLine {
    fn new() -> DMCacheLine {
        DMCacheLine{
            tag: u22::new(0),
            data: 0,
            valid: false,
            dirty: false,
        }
    }
}

impl DMCache {
    fn new(delay: u16, base: Rc<RefCell<dyn Memory<u32, u32>>>) -> DMCache {
        let lines = [DMCacheLine::new(); DM_CACHE_LINES];

        DMCache{
            delay: delay,
            lines: lines,
            base: base,
        }
    }

    fn get_address_index(&self, address: u32) -> usize {
        ((address << 22) >> 22) as usize
    }

    fn get_address_tag(&self, address: u32) -> u22 {
        u22::new(address >> 10)
    }
}

impl InspectableMemory<u32> for DMCache {
    fn inspect_address_txt(&self, address: u32) -> Result<String, String> {
        let idx = self.get_address_index(address);

        let line = self.lines[idx];

        Ok(format!("\
Index: {}
Tag  : {}
Data : {}
Valid: {}
Dirty: {}", idx,
                   line.tag, line.data, line.valid, line.dirty))
    }

    fn get(&self, address: u32) -> u32 {return 0;}
}

// impl Memory<u32, u32> for DMCache {
//     fn get(&mut self, address: u32) -> SimResult<u32, String> {
//         // Get line
//         let idx = self.get_address_index(address);
//         let tag: u22 = self.get_address_tag(address);

//         let line = self.lines[idx];

//         // Check if address in cache
//         if line.valid && line.tag == tag {
//             SimResult::Wait(self.delay, line.data)
//         } else {
//             let mut total_wait: u16 = self.delay;
            
//             // Evict current line if dirty and there is a conflict
//             if line.valid && line.tag != tag && line.dirty {
//                 // Write to cache layer below
//                 let evict_res = self.base.borrow_mut().set(address, line.data);

//                 if let SimResult::Err(e) = evict_res {
//                     return SimResult::Err(format!("failed to write out old line value when evicting: {}", e));
//                 }

//                 if let SimResult::Wait(c, _r) = evict_res {
//                     total_wait += c;
//                 }
//             }

//             // Get value from cache layer below
//             let get_res = self.base.borrow_mut().get(address);

//             let data = match get_res {
//                 SimResult::Ok(d) => d,
//                 SimResult::Err(e) => {
//                     return SimResult::Err(format!("failed to get line value from base cache: {}", e));
//                 },
//                 SimResult::Wait(w, d) => {
//                     total_wait += w;
                    
//                     d
//                 }
//             };

//             // Save in cache
//             self.lines[idx].valid = true;
//             self.lines[idx].dirty = false;
//             self.lines[idx].tag = tag;
//             self.lines[idx].data = data;

//             SimResult::Wait(total_wait, data)
//         }
//     }
    
//     fn set(&mut self, address: u32, data: u32) -> SimResult<(), String> {
//         // Get line
//         let idx = self.get_address_index(address);
//         let tag: u22 = self.get_address_tag(address);

//         let line = self.lines[idx];

//         // If line matches address
//         if line.valid && line.tag == tag {
//             self.lines[idx].dirty = true;
//             self.lines[idx].data = data;

//             SimResult::Wait(self.delay, ())
//         } else {
//             let mut total_wait: u16 = self.delay;
            
//             // Evict current line if dirty and there is a conflict
//             if line.valid && line.tag != tag && line.dirty {
//                 // Write to cache layer below
//                 let old_addr = (u32::from(line.tag) << 10) | (idx as u32);
//                 let evict_res = self.base.borrow_mut().set(old_addr, line.data);

//                 if let SimResult::Err(e) = evict_res {
//                     return SimResult::Err(format!("failed to write out old line value when evicting: {}", e));
//                 }

//                 if let SimResult::Wait(c, _r) = evict_res {
//                     total_wait += c;
//                 }
//             }

//             // Save in cache
//             self.lines[idx].valid = true;
//             self.lines[idx].dirty = true;
//             self.lines[idx].tag = tag;
//             self.lines[idx].data = data;

//             SimResult::Wait(total_wait, ())
//         }
//     }
// }

//----------------------------------------Instructions----------------------------------------

struct instruct {
    inst: u32,
    dest: isize,
    op1: isize,
    op2: isize,
}

impl instruct {
    fn new(instruction: u32) -> instruct {
        instruct{
            inst: instruction,
            dest: instruct::getdest(instruction),
            op1: instruct::getop1(instruction),
            op2: instruct::getop2(instruction),
        }
    }

    fn getop1(inst: u32) -> isize {
        let mut instString: String = inst.to_string();
        let mut inststr: &str = &instString[..];
        let mut instbin = isize::from_str_radix(inststr, 2).unwrap();

        instbin = (instbin << 18) >> 31;
        return instbin;
    }

    fn getop2(inst: u32) -> isize {
        let mut instString: String = inst.to_string();
        let mut inststr: &str = &instString[..];
        let mut instbin = isize::from_str_radix(inststr, 2).unwrap();

        instbin = (instbin << 23) >> 31;
        return instbin;
    }

    fn getdest(inst: u32) -> isize {
        let mut instString: String = inst.to_string();
        let mut inststr: &str = &instString[..];
        let mut instbin = isize::from_str_radix(inststr, 2).unwrap();

        instbin = (instbin << 13) >> 31;
        return instbin;
    }
}

trait Instruction {
    fn execute()
}

// trait Instruction<I> {
    
//     fn addUI(&mut self, instruction: I) -> SimResult<String, String>;
//     fn addSI(&mut self, instruction: I) -> SimResult<(), String>;
//     fn addF(&mut self, instruction: I) -> SimResult<(), String>;
//     fn addUIImm(&mut self, instruction: I) -> SimResult<(), String>;
//     fn addSIImm(&mut self, instruction: I) -> SimResult<(), String>;
//     fn addFImm(&mut self, instruction: I) -> SimResult<(), String>;

//     fn subUI(&mut self, instruction: I) -> SimResult<(), String>;
//     fn subSI(&mut self, instruction: I) -> SimResult<(), String>;
//     fn subF(&mut self, instruction: I) -> SimResult<(), String>;
//     fn subUIImm(&mut self, instruction: I) -> SimResult<(), String>;
//     fn subSIImm(&mut self, instruction: I) -> SimResult<(), String>;
//     fn subFImm(&mut self, instruction: I) -> SimResult<(), String>;

//     fn divUI(&mut self, instruction: I) -> SimResult<(), String>;
//     fn divSI(&mut self, instruction: I) -> SimResult<(), String>;
//     fn divF(&mut self, instruction: I) -> SimResult<(), String>;
//     fn divUIImm(&mut self, instruction: I) -> SimResult<(), String>;
//     fn divSIImm(&mut self, instruction: I) -> SimResult<(), String>;
//     fn divFImm(&mut self, instruction: I) -> SimResult<(), String>;

//     fn mltUI(&mut self, instruction: I) -> SimResult<(), String>;
//     fn mltSI(&mut self, instruction: I) -> SimResult<(), String>;
//     fn mltF(&mut self, instruction: I) -> SimResult<(), String>;
//     fn mltUIImm(&mut self, instruction: I) -> SimResult<(), String>;
//     fn mltSIImm(&mut self, instruction: I) -> SimResult<(), String>;
//     fn mltFImm(&mut self, instruction: I) -> SimResult<(), String>;

//     fn mv(&mut self, instruction: I) -> SimResult<(), String>;

//     fn cmpUI(&mut self, instruction: I) -> SimResult<(), String>;
//     fn cmpSI(&mut self, instruction: I) -> SimResult<(), String>;
//     fn cmpF(&mut self, instruction: I) -> SimResult<(), String>;

//     fn asl(&mut self, instruction: I) -> SimResult<(), String>;
//     fn asr(&mut self, instruction: I) -> SimResult<(), String>;
//     fn aslImm(&mut self, instruction: I) -> SimResult<(), String>;
//     fn asrImm(&mut self, instruction: I) -> SimResult<(), String>;
//     fn lsl(&mut self, instruction: I) -> SimResult<(), String>;
//     fn lsr(&mut self, instruction: I) -> SimResult<(), String>;
//     fn lslImm(&mut self, instruction: I) -> SimResult<(), String>;
//     fn lsrImm(&mut self, instruction: I) -> SimResult<(), String>;

//     fn and(&mut self, instruction: I) -> SimResult<(), String>;
//     fn or(&mut self, instruction: I) -> SimResult<(), String>;
//     fn xor(&mut self, instruction: I) -> SimResult<(), String>;
//     fn andImm(&mut self, instruction: I) -> SimResult<(), String>;
//     fn orImm(&mut self, instruction: I) -> SimResult<(), String>;
//     fn xorImm(&mut self, instruction: I) -> SimResult<(), String>;

//     fn not(&mut self, instruction: I) -> SimResult<(), String>;

//     fn ldr(&mut self, instruction: I) -> SimResult<(), String>;
//     fn str(&mut self, instruction: I) -> SimResult<(), String>;
//     fn push(&mut self, instruction: I) -> SimResult<(), String>;
//     fn pop(&mut self, instruction: I) -> SimResult<(), String>;

//     fn jmp(&mut self, instruction: I) -> SimResult<(), String>;
//     fn jmpImm(&mut self, instruction: I) -> SimResult<(), String>;
//     fn jmpS(&mut self, instruction: I) -> SimResult<(), String>;
//     fn jmpSImm(&mut self, instruction: I) -> SimResult<(), String>;

//     fn sih(&mut self, instruction: I) -> SimResult<(), String>;
//     fn int(&mut self, instruction: I) -> SimResult<(), String>;
//     fn ijmp(&mut self, instruction: I) -> SimResult<(), String>;
// }

impl Instruction<u32> for instruct{

    fn addUI(&mut self, instruction: u32) -> SimResult<String, String> {

        let result: isize = self.op1.checked_add(self.op2).unwrap();
        return SimResult::Ok(result.to_string());
    }
}


//--------------------------------------------Main Function----------------------------------------------


fn help() {
    println!("Commands:

- exit(): Exit program
- help(): Show this help text
- get(address): Load address from memory
- set(address, data): Write data to address in memory
- show(level, address): Show an address's cache line, level can be: L1, L2, L3, DRAM");
}

fn main() {
    // TODO: line length 4
    let dram = DRAM::new(100);
    // let l3_cache = Rc::new(RefCell::new(DMCache::new(40, dram.clone())));
    // let l2_cache = Rc::new(RefCell::new(DMCache::new(10, l3_cache.clone())));
    // let l1_cache = Rc::new(RefCell::new(DMCache::new(1, l2_cache.clone())));

    let memory = &dram;
    
    help();

    let mut inst: instruct = instruct::new(1756010927);

    // loop {
    //     print!("> ");
    //     io::stdout().flush().expect("failed to flush stdout");
        
    //     let cmd: String;
    //     let operands: String;
    //     scan!("{}({})\n", cmd, operands);



    //     match cmd.as_str() {
    //         "get" => {
    //             // Parse operands
    //             let address: u32;
    //             scan!(operands.bytes() => "{}", address);

    //             // Perform operation
    //             let s = memory.get(address);
    //             match s {
    //                 SimResult::Ok(v) => {
    //                     println!("Completed in 0 cycles");
    //                     println!("{}: {}", address, v);
    //                 },
    //                 SimResult::Err(e) => eprintln!("Failed to get {}: {}", address, e),
    //                 SimResult::Wait(c, v) => {
    //                     println!("Completed in {} cycles", c);
    //                     println!("{}: {}", address, v);
    //                 }
    //             };
    //         },

    //         "set" => {
    //             // Parse operands
    //             let address: u32;
    //             let data: u32;
    //             scan!(operands.bytes() => "{}, {}", address, data);

    //             // Perform operation
    //             match memory.set(address, data) {
    //                 SimResult::Ok(_v) => {
    //                     println!("Completed in 0 cycles");
    //                 },
    //                 SimResult::Err(e) => eprintln!("Failed to set {}: {}",
    //                                                address, e),
    //                 SimResult::Wait(c, _v) => {
    //                     println!("Completed in {} cycles", c);
    //                 }
    //             };
    //         },

    //         "show" => {
    //             // Parse operands
    //             let level: String;
    //             let address: u32;
    //             scan!(operands.bytes() => "{}, {}", level, address);

    //             let inspect_res = match level.as_str() {
    //                 "L1" => l1_cache.borrow().inspect_address_txt(address),
    //                 "L2" => l2_cache.borrow().inspect_address_txt(address),
    //                 "L3" => l3_cache.borrow().inspect_address_txt(address),
    //                 "DRAM" => dram.borrow().inspect_address_txt(address),
    //                 _ => Err(format!("Cache level name \"{}\" not recognized",
    //                                  level)),
    //             };

    //             match inspect_res {
    //                 Ok(txt) => {
    //                     println!("{} at {}", level, address);
    //                     println!("{}", txt);
    //                 },
    //                 Err(e) => {
    //                     eprintln!("Failed to inspect {} at {}: {}", level, address,
    //                               e);
    //                 }
    //             };
    //         },
    //         "help" => help(),
    //         "exit" => {
    //             exit(0);
    //         },
    //         _ => {
    //             eprintln!("Invalid command: {}", cmd);
    //             eprintln!("Use help() command to see valid commands");
    //         }
    //     }
    // }
}
