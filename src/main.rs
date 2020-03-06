extern crate ux;
extern crate text_io;

use std::io;
use std::io::Write;
use std::process::exit;
use std::cell::RefCell;
use std::rc::Rc;
use std::cmp::PartialEq;
use std::fmt::{Debug,Display};

use text_io::scan;

/// Indicates the status of a simulator operation with either a value, error, or
/// result which will be available after a delay. D is the data type, E is the
/// error type.
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

/// Length of cache lines
const CACHE_LINE_LEN: usize = 4;

/// Simulates the slow DRAM memory. Since data is stored in groups of 4 the lower
/// 2 bits of each address will be truncated.
struct DRAM {
    /// Number of processor cycles it takes to retrieve data.
    delay: u16,

    /// Data in memory stored in a linear form. Since DRAM can be quite large we
    /// don't want to store all unset addresses. We use a data_table for this.
    data: Vec<[u32; CACHE_LINE_LEN]>,

    /// Keeps track of the start index in the data field for regions of memory.
    data_table: Vec<DRAMDataTableEntry>,
}

/// Entry in the DRAM data_table.
#[derive(Clone,Copy,PartialEq,Debug)]
struct DRAMDataTableEntry {
    /// First address in memory region, inclusive.
    start_address: u32,

    /// Index in the DRAM data field.
    data_index: usize,

    /// Length of memory region, inclusive.
    length: u32,
}

impl DRAM {
    fn new(delay: u16) -> DRAM {
        DRAM{
            delay: delay,
            data: Vec::new(),
            data_table: Vec::new(),
        }
    }

    /// Returns the data table entry for the specified address. Second entry in
    /// tuple is the index in the data_table which the entry was found. The address
    /// argument should already have its lower 2 bits truncated. For internal use.
    fn data_table_entry(&self, address: u32) -> Option<(DRAMDataTableEntry, usize)> {
        let mut idx = 0;
        for entry in self.data_table.iter() {
            if address >= entry.start_address && address <= (entry.start_address + entry.length) {
                return Some((*entry, idx));
            }

            idx += 1;
        }

        None
    }
}

impl InspectableMemory<u32> for DRAM {
    fn inspect_txt(&self) -> Result<String, String> {
        let mut out = String::new();
        out.push_str(format!("Delay: {}", self.delay).as_str());
        out.push_str("\nData Table:");

        for entry in self.data_table.iter() {
            out.push_str(format!("\n    Start Address: {}, Data Index: {}, Length: {}",
                             entry.start_address, entry.data_index, entry.length).as_str());
        }

        out.push_str("\nData:");
        let mut idx: usize = 0;
        while idx < self.data.len() {
            out.push_str(format!("\n    {}: {:?}", idx, self.data[idx]).as_str());
            idx += 1;
        }

        Ok(out)
    }
    
    fn inspect_address_txt(&self, address: u32) -> Result<String, String> {
        let trimmed_address = address >> 2;
        
        // Find route_table entry
        let value = match self.data_table_entry(trimmed_address) {
            Some((entry, _entry_idx)) => {
                self.data[entry.data_index + (address as usize)]
            },
            None => {
                [0; 4]
            },
        };

        Ok(format!("\
Address: {}
Value  : {:?}", address, value))
    }
}

impl Memory<u32, [u32; CACHE_LINE_LEN]> for DRAM {
    fn get(&mut self, raw_address: u32) -> SimResult<[u32; CACHE_LINE_LEN], String> {
        // Truncate lower 2 bits of address since data stored in groups of 4
        let address = raw_address >> 2;
        
        // Find address range in data_table
        let (range_entry, _idx) = match self.data_table_entry(address) {
            Some(e) => e,
            None => {
                // No value explicity stored
                return SimResult::Wait(self.delay, [0; 4])
            },
        };

        // Find value in data array
        let data_idx = range_entry.data_index + ((range_entry.start_address - address) as usize);
        SimResult::Wait(self.delay, self.data[data_idx])
    }
    
    fn set(&mut self, raw_address: u32, data: [u32; CACHE_LINE_LEN]) -> SimResult<(), String> {
        // Truncate lower 2 bits of address since data stored in groups of 4
        let address = raw_address >> 2;

        // Check if address range in data_table
        let (range_entry, entry_idx) = match self.data_table_entry(address) {
            Some(d) => d,
            None => {
                // No entry, first find index to insert at
                let mut insert_idx: usize = 0;
                let mut last_calc_data_idx: usize = 0;
                while insert_idx < self.data_table.len() &&
                    self.data_table[insert_idx].start_address < address {
                        
                    last_calc_data_idx = self.data_table[insert_idx].data_index +
                        (self.data_table[insert_idx].length as usize);
                    insert_idx += 1;
                }

                // Insert
                self.data_table.insert(insert_idx, DRAMDataTableEntry{
                    start_address: address,
                    data_index: last_calc_data_idx,
                    length: 0,
                });

                (self.data_table[insert_idx], insert_idx)
            },
        };

        // Insert
        self.data.insert(range_entry.data_index, data);
        self.data_table[entry_idx].length += 1;

        // Modify other data table entries to account for newly inserted data
        let mut modify_idx = entry_idx + 1;

        while modify_idx < self.data_table.len() {
            self.data_table[modify_idx].data_index += 1;
            
            modify_idx += 1;
        }
        
        // Coalesce data table entries
        modify_idx = 0;

        while modify_idx < self.data_table.len() - 1 {
            let entry = self.data_table[modify_idx];
            let next_entry = self.data_table[modify_idx + 1];

            if (entry.start_address + entry.length) == next_entry.start_address {
                self.data_table[modify_idx].length = next_entry.length;
                self.data_table.remove(modify_idx + 1);
            }

            modify_idx += 1;
        }

        // Return
        SimResult::Wait(self.delay, ())
    }
}

/*
const DM_CACHE_LINES: usize = 1024;

// Direct mapped cache.
// 2 least significant bits are offset.
// 10 next least significant bits of an address are the index.
// 20 most significant bits of an address are the tag.
struct DMCache {
    delay: u16,
    lines: [DMCacheLine; DM_CACHE_LINES],
    base: Rc<RefCell<dyn Memory<u32, [u32; CACHE_LINE_LEN]>>>,
}

#[derive(Copy,Clone)]
struct DMCacheLine {
    tag: u32,
    data: [u32; CACHE_LINE_LEN],
    valid: bool,
    dirty: bool,
}

impl DMCacheLine {
    fn new() -> DMCacheLine {
        DMCacheLine{
            tag: 0,
            data: [0; CACHE_LINE_LEN],
            valid: false,
            dirty: false,
        }
    }
}

impl DMCache {
    fn new(delay: u16, base: Rc<RefCell<dyn Memory<u32, [u32; CACHE_LINE_LEN]>>>) -> DMCache {
        let lines = [DMCacheLine::new(); DM_CACHE_LINES];

        DMCache{
            delay: delay,
            lines: lines,
            base: base,
        }
    }

    fn get_address_offset(&self, address: u32) -> usize {
        (address & 3) as usize
    }

    fn get_address_index(&self, address: u32) -> usize {
        ((address & 0xFFC) >> 2) as usize
    }
    
    fn get_address_tag(&self, address: u32) -> u32 {
        address >> 12
    }
}

impl InspectableMemory<u32> for DMCache {
    fn inspect_address_txt(&self, address: u32) -> Result<String, String> {
        let idx = self.get_address_index(address);
        let offset = self.get_address_offset(address);

        let line = self.lines[idx];

        Ok(format!("\
Index : {}
Tag   : {}
Offset: {}
Data  : {:?}
Valid : {}
Dirty : {}", idx, line.tag, offset, line.data, line.valid, line.dirty))
    }
}

impl Memory<u32, u32> for DMCache {
    // TODO: Make DMCache.{get,set} use offset
    // TODO: Make DMCache.{get,set} use base as Memory<u32, [u32; CACHE_LINE_LEN]>
    fn get(&mut self, address: u32) -> SimResult<u32, String> {
        // Extract components from address
        let idx = self.get_address_index(address);
        let tag = self.get_address_tag(address);
        let offset = self.get_address_offset(address);

        let line = self.lines[idx];

        // Cache hit
        if line.valid && line.tag == tag {
            SimResult::Wait(self.delay, line.data[offset])
        } else { // Cache miss
            // Record total cycles used in servicing miss
            let mut total_wait: u16 = self.delay;
            
            // Evict current line if dirty and there is a conflict
            if line.valid && line.tag != tag && line.dirty {
                // Write to cache layer below
                let evict_res = self.base.borrow_mut().set(address, line.data);

                if let SimResult::Err(e) = evict_res {
                    return SimResult::Err(format!("failed to write out conflicting line when evicting: {}", e));
                }

                if let SimResult::Wait(c, _r) = evict_res {
                    total_wait += c;
                }
            }

            // Get value from cache layer below
            let get_res = self.base.borrow_mut().get(address);

            let data = match get_res {
                SimResult::Err(e) => {
                    return SimResult::Err(format!("failed to get line value from base cache: {}", e));
                },
                SimResult::Wait(w, d) => {
                    total_wait += w;
                    
                    d
                }
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
*/

fn help() {
    println!("Commands:

- exit(): Exit program
- help(): Show this help text
- get(address): Load address from memory
- set(address, data): Write data to address in memory
- show(level, address): Show an address's cache line, level can be: L1, L2, L3, DRAM");
}

fn main() {
    // TODO: Figure out why set(5, x), set(6, x), get(5) == 6, get(6) == 6, but 5 and 6 get placed in different DRAM.data indexes, is this bc the mechanism hasn't quite been refactored to store 4 values in each index? I thought truncating first 2 bits of address would do this, investigate...
    // TODO: line length 4
    let dram = Rc::new(RefCell::new(DRAM::new(100)));
    /*
    let l3_cache = Rc::new(RefCell::new(DMCache::new(40, dram.clone())));
    let l2_cache = Rc::new(RefCell::new(DMCache::new(10, l3_cache.clone())));
    let l1_cache = Rc::new(RefCell::new(DMCache::new(1, l2_cache.clone())));
*/

    //let memory = &l1_cache;
    let memory = &dram;
    
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
                match memory.borrow_mut().set(address, [data; 4]) {
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
                            "DRAM" => dram.borrow().inspect_txt(),
                            _ => Err(format!("Cache level name \"{}\" not recognized", level)),
                        }
                    },
                    _ => Err(format!("Specific address show not supported at this time")),
                    /*
                    _ => {
                        let address: u32 = match address_str.parse() {
                            Err(e) => {
                                return Err(format!("Failed to parse address argument {} as u32: {}", address_str, e));
                            },
                            Ok(v) => v,
                        };
                        
                        match level.as_str() {
                            /*
                            "L1" => l1_cache.borrow().inspect_address_txt(address),
                            "L2" => l2_cache.borrow().inspect_address_txt(address),
                            "L3" => l3_cache.borrow().inspect_address_txt(address),
*/
                            "DRAM" => dram.borrow().inspect_address_txt(address),
                            _ => Err(format!("Cache level name \"{}\" not recognized",
                                             level)),
                        }
                    },
*/
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dram_data_table_entries() {
        let mut dram = DRAM::new(100);

        // Ensure data_table entries are created for 2 addresses far apart
        const ADDR_A: u32 = 5;
        const VAL_A: [u32; 4] = [6; 4];
        
        const ADDR_B: u32 = 500;
        const VAL_B: [u32; 4] = [666; 4];
        
        dram.set(ADDR_A, VAL_A).unwrap("Failed to set a");
        dram.set(ADDR_B, VAL_B).unwrap("Failed to set b");

        assert_eq!(dram.data_table[0], DRAMDataTableEntry{
            start_address: ADDR_A >> 2,
            data_index: 0,
            length: 1,
        }, "Data table entry for A");
        assert_eq!(dram.data_table[1], DRAMDataTableEntry{
            start_address: ADDR_B >> 2,
            data_index: 1,
            length: 1,
        }, "Data table entry for B");

        assert_eq!(dram.get(ADDR_A).unwrap("Failed to get a").1, VAL_A,
                   "Value for A");
        assert_eq!(dram.get(ADDR_B).unwrap("Failed to get b").1, VAL_B,
                   "Value for B");
    }

    // TODO: Write data_table coalesce test, write insert in middle of range test
}
