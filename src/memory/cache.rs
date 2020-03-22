extern crate ux;
extern crate text_io;

use std::path::Path;
use std::fs::{File, remove_file};
use std::io::{Write, BufReader, BufRead, LineWriter};
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::marker::Sync;

use ux::{u22};

lazy_static! {
    static ref dummydram: DRAM = DRAM::new(0);
    static ref memory: &'static mut dyn Memory<u32, u32> = &mut dummydram;
}
unsafe impl !Sync for memory {}
// const dummydram: DRAM = DRAM::new(0);

// static mut memory: &mut dyn Memory<u32, u32> = &mut *a_mod::dummydram;

/// Indicates the status of a simulator operation with either a value, error, or
/// result which will be available after a delay. D is the data type, E is the
/// error type.
pub enum SimResult<D, E> {
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

pub fn newMemory(dramDelay: u16, cacheDelay: u16) {
    let mut dram: DRAM = DRAM::new(dramDelay);
    let mut cache: DMCache = DMCache::new(cacheDelay, &mut dram);
    unsafe{
        memory = &mut dram;
    }
}

pub fn get(address: u32) -> SimResult<u32, String> {
    unsafe{
        return memory.get(address);
    }
}

pub fn set(address: u32, data: u32) -> SimResult<(), String> {
    unsafe{
        return memory.set(address, data);
    }
}

// Memory, A is the address type, D is the data type.
trait Memory<A, D> {
    fn get(&mut self, address: A) -> SimResult<D, String>;
    fn set(&mut self, address: A, data: D) -> SimResult<(), String>;
}

struct DRAM {
    delay: u16,
    data: File,
}

impl DRAM {
    fn new(delay: u16) -> DRAM {
        DRAM{
            delay: delay,
            data: DRAM::create(),
        }
    }

    fn create() -> File {
        let disk_file_path = Path::new("src/memory/dram");
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
        let file: File = File::open("src/memory/dram").unwrap();
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

        let file = File::open("src/memory/dram").unwrap();
        let reader = BufReader::new(file);
        remove_file("src/memory/dram");
        let file = File::create("src/memory/dram").unwrap();
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

const DM_CACHE_LINES: usize = 1024;

// Direct mapped cache.
// 10 least significant bits of an address are the index.
// 22 most significant bits of an address are the tag.
struct DMCache<'a> {
    delay: u16,
    lines: [DMCacheLine; DM_CACHE_LINES],
    base: &'a mut dyn Memory<u32, u32>,
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

impl<'a> DMCache<'a> {
    fn new(delay: u16, base: &'a mut dyn Memory<u32, u32>) -> DMCache {
        let lines = [DMCacheLine::new(); DM_CACHE_LINES];

        DMCache{
            delay: delay,
            lines: lines,
            base: base,
        }
    }
}

impl<'a> Memory<u32, u32> for DMCache<'a> {
    fn get(&mut self, address: u32) -> SimResult<u32, String> {
        // Get line
        let idx = ((address << 22) >> 22) as usize;
        let tag: u22 = u22::new(address >> 10);

        let line = self.lines[idx];

        // Check if address in cache
        if line.valid && line.tag == tag {
            SimResult::Wait(self.delay, line.data)
        } else {
            let mut total_wait: u16 = self.delay;
            
            // Evict current line if dirty and there is a conflict
            if line.valid && line.tag != tag && line.dirty {
                // Write to cache layer below
                let evict_res = self.base.set(address, line.data);

                if let SimResult::Err(e) = evict_res {
                    return SimResult::Err(format!("failed to write out old line value when evicting: {}", e));
                }

                if let SimResult::Wait(c, _r) = evict_res {
                    total_wait += c;
                }
            }

            // Get value from cache layer below
            let get_res = self.base.get(address);

            let data = match get_res {
                SimResult::Ok(d) => d,
                SimResult::Err(e) => {
                    return SimResult::Err(format!("failed to get line value from base cache: {}", e));
                },
                SimResult::Wait(w, d) => {
                    total_wait += w;
                    
                    d
                }
            };

            // Save in cache
            self.lines[idx].dirty = false;
            self.lines[idx].tag = tag;
            self.lines[idx].data = data;

            SimResult::Wait(total_wait, data)
        }
    }
    
    fn set(&mut self, address: u32, data: u32) -> SimResult<(), String> {
        // Get line
        let idx = ((address << 22) >> 22) as usize;
        let tag: u22 = u22::new(address >> 10);

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
                let evict_res = self.base.set(address, line.data);

                if let SimResult::Err(e) = evict_res {
                    return SimResult::Err(format!("failed to write out old line value when evicting: {}", e));
                }

                if let SimResult::Wait(c, _r) = evict_res {
                    total_wait += c;
                }
            }

            // Save in cache
            self.lines[idx].dirty = false;
            self.lines[idx].tag = tag;
            self.lines[idx].data = data;

            SimResult::Wait(total_wait, ())
        }
    }
}