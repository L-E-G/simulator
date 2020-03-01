extern crate num;
extern crate ux;

use std::collections::HashMap;
use std::hash::Hash;
use std::default::Default;

use num::Integer;
use num::bounds::Bounded;

use ux::{u1,u2};

/// Represents unsigned integers which are smaller than 8 bits.
/// To perform computations on the 
struct SmallUInt {
    // Number of bits.
    size: usize,
    
    // Bits in little endian order.
    bits: Vec<bool>,
}

impl SmallUInt {
    /// Creates a SmallUInt from data. Only the first size bits of the u8 will
    /// be used.
    fn new(size: usize, data: u8) -> SmallUInt {
        let data = data.to_le_bytes()[0];
        let mut bits = Vec::<bool>::new();
        
        for i in 0..size {
            bits.push(((data & (1 << i)) >> i) == 1);
        }

        SmallUInt{
            size: size,
            bits: bits,
        }
    }
    
    /// Converts to a u8 so operations can take place.
    fn as_u8(&self) -> u8 {
        let mut accum: u8 = 0;
        
        for i in 0..self.size {
            accum |= (self.bits[i] as u8) << i;
        }

        accum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn smalluint_new() {
        let s0 = SmallUInt::new(8, 0);
        assert_eq!(s0.size, 8);
        assert_eq!(s0.bits, [false; 8]);

        // 128 64 32 16 8 4 2 1
        // 1   1  0  0  1 1 0 1   = 205
        // ^ ignore ^  | ^ pack ^
        let s5 = SmallUInt::new(4, 205);
        assert_eq!(s5.size, 4);
        assert_eq!(s5.bits, [true, false, true, true]);
    }

    #[test]
    fn smalluint_as_u8() {
        let s5 = SmallUInt::new(4, 205);
        assert_eq!(s5.as_u8(), 13);
    }
}


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
    Wait(u16, Box<SimResult<D, E>>),
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
                    return *res;
                }

                SimResult::Wait(i-1, res)
            },
        }
    }
}

trait Address: Integer + Hash {}

// Memory, A is the address type, D is the data type.
trait Memory<A: Address, D> {
    fn get(self, address: A) -> SimResult<D, String>;
    fn set(self, address: A, data: D) -> SimResult<(), String>;
}

// The 4-way associative cache will be ~64 KB large.
// Each line will be 71 bits large:
//     tag 2b, offset 1b, data1 32b, data2 32b, dirty 1b, LRU 2b, valid 1b
// Each set will have 4 lines => 284 bits per set.
// 64 KB / 284 bits ~= 1,802 sets
// This means that the index will take 11 bits since log_2(1,802) = 10.5 so we
// need 11 bits.
const NUM_FOUR_WAY_ASSOC_CACHE_SETS: usize = 1802;

// Mask which extracts 11 bits for index.
// 0000 0000 0000 0000 0011 1111 1111 1000
const FOUR_WAY_ASSOC_CACHE_ADDR_MASK: u32 = 0x3FF8;

// Mask which extracts 2 bits for the tag.
// 0000 0000 0000 0000 0000 0000 0000 0110
const FOUR_WAY_ASSOC_CACHE_TAG_MASK: u32 = 0x6;

// 4-ways associative cache. Contains sets. Each set is selected by an index.
// Each set contains a number of lines, called ways.
struct FourWayAssocCache<'a> {
    sets: [FourWayAssocSet; NUM_FOUR_WAY_ASSOC_CACHE_SETS],

    // Underlying memory used to service misses.
    base: &'a dyn Memory<u32, u32>
}

struct FourWayAssocSet {
    ways: [FourWayAssocLine; 4],
}

impl FourWayAssocSet {
    fn new() -> FourWayAssocSet {
        FourWayAssocSet{
            ways: [FourWayAssocLine::new(), FourWayAssocLine::new(),
                   FourWayAssocLine::new(), FourWayAssocLine::new()],
        }
    }
}

struct FourWayAssocLine {
    tag: u2,
    offset: u1,
    data: [u32; 2],
    dirty: bool,
    lru: u2,
    valid: bool,
}

impl FourWayAssocLine {
    fn new() -> FourWayAssocLine {
        FourWayAssocLine{
            tag: 0,
            offset: 0,
            data: [0; 2],
            dirty: false,
            lru: 0,
            valid: false,
        }
    }
}

impl<'a> FourWayAssocCache<'a> {
    // Creates an AssocMem, errors if the address type cannot represent the
    fn new(nsets: u8, nways: u8, base: &'a dyn Memory<u32, u32>)
           -> FourWayAssocCache<'a> {
        let mut sets: [FourWayAssocSet; NUM_FOUR_WAY_ASSOC_CACHE_SETS];
        for i in 0..NUM_FOUR_WAY_ASSOC_CACHE_SETS {
            sets[i] = FourWayAssocSet::new();
        }
        
        FourWayAssocCache{
            sets: sets,
            base: base,
        }
    }
}

impl<'a> Memory for FourWayAssocCache {
    fn get(&self, address: u32) -> SimResult<u32, String> {
        // Get set for index
        let idx = (address & FOUR_WAY_ASSOC_CACHE_ADDR_MASK) >> 3;
        let set = self.sets[idx];

        // Get tag bits
        let tag: u2 = (address & FOUR_WAY_ASSOC_CACHE_TAG_MASK) >> 1;

        // Try to find way with tag
        for i in 0..4 {
            if set.ways[i].valid && set.ways[i].tag == tag {
                
            }
        }
    }
    
    fn set(&self, address: u32, data: u32) -> SimResult<(), String> {
    }
}

fn main() {
    println!("Hello, world!");
}
