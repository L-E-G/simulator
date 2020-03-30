extern crate text_io;

use std::io;
use std::io::Write;
use std::process::exit;
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::PartialEq;
use std::fmt::{Debug,Display};
use std::collections::HashMap;

use text_io::scan;

/// Indicates the status of a simulator operation with either a value, error, or
/// result which will be available after a delay. D is the data type, E is the
/// error type.
#[derive(Debug,PartialEq)]
enum SimResult<D, E: Display> {
    /// Error if operation failed.
    Err(E),

    /// Indicates result is not yet available but was successful. First field is
    /// the number of simulator cycles before the value will be ready. A value of
    /// 0 indicates the result is ready. The second field is the value.
    Wait(u16, D),
}

impl<D, E: Display> SimResult<D, E> {
    /// Panics if Err, otherwise returns Wait fields.
    fn unwrap(self, panic_msg: &str) -> (u16, D) {
        match self {
            SimResult::Err(e) => panic!(format!("{}: {}", panic_msg, e)),
            SimResult::Wait(c, d) => (c, d),
        }
    }
}

/// Memory provides an interface to access a memory struct, A is the address type,
/// D is the data type.
trait Memory<A, D> {
    /// Retrieve data at a memory address.
    fn get(&mut self, address: A) -> SimResult<D, String>;

    /// Place data at a memory address.
    fn set(&mut self, address: A, data: D) -> SimResult<(), String>;
}

/// InspectableMemory allows a memory unit to be insepcted for user
/// interface purposes. A is the address type.
trait InspectableMemory<A> {
    /// Returns a text description of the entire data structure.
    fn inspect_txt(&self) -> Result<String, String>;
    
    /// Returns a text description of an address.
    fn inspect_address_txt(&self, address: A) -> Result<String, String>;
}

/// Simulates the slow DRAM memory.
struct DRAM {
    delay: u16,
    data: HashMap<u32, u32>,
}

impl DRAM {
    fn new(delay: u16) -> DRAM {
        DRAM{
            delay: delay,
            data: HashMap::new(),
        }
    }
}

impl InspectableMemory<u32> for DRAM {
    fn inspect_txt(&self) -> Result<String, String> {
        let mut out = String::new();

        for (k, v) in &self.data {
            out.push_str(format!("{}: {}\n", k, v).as_str());
        }

        Ok(out)
    }
    
    fn inspect_address_txt(&self, address: u32) -> Result<String, String> {
        match self.data.get(&address) {
            Some(d) => Ok(format!("\
Address: {}
Value  : {}", address, *d)),
            None => Ok(format!("Does not exist")),
        }
    }
}

impl Memory<u32, u32> for DRAM {
    fn get(&mut self, address: u32) -> SimResult<u32, String> {
        match self.data.get(&address) {
            Some(d) => SimResult::Wait(self.delay, *d),
            None => {
                self.data.insert(address, 0);
                SimResult::Wait(self.delay, 0)
            }
        }
    }
    
    fn set(&mut self, address: u32, data: u32) -> SimResult<(), String> {
        self.data.insert(address, data);
        SimResult::Wait(self.delay, ())
    }
}


const DM_CACHE_LINES: usize = 1024;

// Direct mapped cache.
// 10 least significant bits of an address are the index.
// 22 most significant bits of an address are the tag.
struct DMCache {
    delay: u16,
    lines: [DMCacheLine; DM_CACHE_LINES],
    base: Rc<RefCell<dyn Memory<u32, u32>>>,
}

#[derive(Copy,Clone)]
struct DMCacheLine {
    tag: u32,
    data: u32,
    valid: bool,
    dirty: bool,
}

impl DMCacheLine {
    fn new() -> DMCacheLine {
        DMCacheLine{
            tag: 0,
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

    fn get_address_tag(&self, address: u32) -> u32 {
        address >> 10
    }
}

impl InspectableMemory<u32> for DMCache {
    fn inspect_txt(&self) -> Result<String, String> {
        let mut out = String::new();

        for line in self.lines.iter() {
            if !line.valid {
                continue;
            }
            
            out.push_str(format!("{} = {} [valid={}, dirty={}]\n",
                line.tag, line.data, line.valid, line.dirty).as_str());
        }

        Ok(out)
    }
        
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
}

impl Memory<u32, u32> for DMCache {
    fn get(&mut self, address: u32) -> SimResult<u32, String> {
        // Get line
        let idx = self.get_address_index(address);
        let tag = self.get_address_tag(address);

        let line = self.lines[idx];

        // Check if address in cache
        if line.valid && line.tag == tag {
            SimResult::Wait(self.delay, line.data)
        } else {
            let mut total_wait: u16 = self.delay;
            
            // Evict current line if dirty and there is a conflict
            if line.valid && line.tag != tag && line.dirty {
                // Write to cache layer below
                let evict_res = self.base.borrow_mut().set(address, line.data);

                if let SimResult::Err(e) = evict_res {
                    return SimResult::Err(format!("failed to write out old line value when evicting: {}", e));
                }

                if let SimResult::Wait(c, _r) = evict_res {
                    total_wait += c;
                }
            }

            // Get value from cache layer below
            let get_res = self.base.borrow_mut().get(address);

            let data = match get_res {
                SimResult::Wait(w, d) => {
                    total_wait += w;
                    
                    d
                },
                SimResult::Err(e) => {
                    return SimResult::Err(format!("failed to get line value from base cache: {}", e));
                },
            };

            // Save in cache
            self.lines[idx].valid = true;
            self.lines[idx].dirty = false;
            self.lines[idx].tag = tag;
            self.lines[idx].data = data;

            SimResult::Wait(total_wait, data)
        }
    }
    
    fn set(&mut self, address: u32, data: u32) -> SimResult<(), String> {
        // Get line
        let idx = self.get_address_index(address);
        let tag = self.get_address_tag(address);

        let line = self.lines[idx];

        // If line matches address
        if line.valid && line.tag == tag {
            self.lines[idx].dirty = true;
            self.lines[idx].data = data;

            SimResult::Wait(self.delay, ())
        } else {
            let mut total_wait: u16 = self.delay;
            
            // Evict current line if dirty and there is a conflict
            if line.valid && line.tag != tag && line.dirty {
                // Write to cache layer below
                let old_addr = (u32::from(line.tag) << 10) | (idx as u32);
                let evict_res = self.base.borrow_mut().set(old_addr, line.data);

                if let SimResult::Err(e) = evict_res {
                    return SimResult::Err(format!("failed to write out old line value when evicting: {}", e));
                }

                if let SimResult::Wait(c, _r) = evict_res {
                    total_wait += c;
                }
            }

            // Save in cache
            self.lines[idx].valid = true;
            self.lines[idx].dirty = true;
            self.lines[idx].tag = tag;
            self.lines[idx].data = data;

            SimResult::Wait(total_wait, ())
        }
    }
}

fn help() {
    println!("Commands:

- exit(): Exit program
- help(): Show this help text
- get(address): Load address from memory
- set(address, data): Write data to address in memory
- show(level, address): Show an address's cache line, level can be: L1, L2, L3, DRAM. If address is \"_\" the entire level will be shown");
}

fn main() {
    let dram = Rc::new(RefCell::new(DRAM::new(100)));
    let l3_cache = Rc::new(RefCell::new(DMCache::new(40, dram.clone())));
    let l2_cache = Rc::new(RefCell::new(DMCache::new(10, l3_cache.clone())));
    let l1_cache = Rc::new(RefCell::new(DMCache::new(1, l2_cache.clone())));

    let memory = &l1_cache;
    
    help();

    loop {
        print!("> ");
        io::stdout().flush().expect("failed to flush stdout");
        
        let cmd: String;
        let operands: String;
        scan!("{}({})\n", cmd, operands);

        match cmd.as_str() {
            "get" => {
                // Parse operands
                let address: u32;
                scan!(operands.bytes() => "{}", address);

                // Perform operation
                match memory.borrow_mut().get(address) {
                    SimResult::Err(e) => eprintln!("Failed to get {}: {}",
                                                   address, e),
                    SimResult::Wait(c, v) => {
                        println!("Completed in {} cycles", c);
                        println!("{}: {:?}", address, v);
                    }
                };
            },
            "set" => {
                // Parse operands
                let address: u32;
                let data: u32;
                scan!(operands.bytes() => "{}, {}", address, data);

                // Perform operation
                match memory.borrow_mut().set(address, data) {
                    SimResult::Err(e) => eprintln!("Failed to set {}: {}",
                                                   address, e),
                    SimResult::Wait(c, _v) => {
                        println!("Completed in {} cycles", c);
                    }
                };
            },
            "show" => {
                // Parse operands
                let level: String;
                let address_str: String;
                scan!(operands.bytes() => "{}, {}", level, address_str);

                let inspect_res = match address_str.as_str() {
                    "_" => {
                        match level.as_str() {
                            "L1" => l1_cache.borrow().inspect_txt(),
                            "L2" => l2_cache.borrow().inspect_txt(),
                            "L3" => l3_cache.borrow().inspect_txt(),
                            "DRAM" => dram.borrow().inspect_txt(),
                            _ => Err(format!("Cache level name \"{}\" not \
                                recognized", level)),
                        }
                    },
                    _ => {
                        match address_str.parse() {
                            Err(e) => Err(format!("Failed to parse address argument {} as u32: {}", address_str, e)),
                            Ok(address) => {
                                match level.as_str() {
                                    "L1" => l1_cache.borrow()
                                        .inspect_address_txt(address),
                                    "L2" => l2_cache.borrow()
                                        .inspect_address_txt(address),
                                    "L3" => l3_cache.borrow()
                                        .inspect_address_txt(address),
                                    "DRAM" => dram.borrow()
                                        .inspect_address_txt(address),
                                    _ => Err(format!("Cache level name \"{}\" not \
                                                      recognized", level)),
                                }
                            }
                        }
                    },
                };

                match inspect_res {
                    Ok(txt) => {
                        println!("{} at {}", level, address_str);
                        println!("{}", txt);
                    },
                    Err(e) => {
                        eprintln!("Failed to inspect {} at {}: {}", level,
                                  address_str, e);
                    }
                };
            },
            "help" => help(),
            "exit" => {
                exit(0);
            },
            _ => {
                eprintln!("Invalid command: {}", cmd);
                eprintln!("Use help() command to see valid commands");
            }
        }
    }
}
