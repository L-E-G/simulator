use std::collections::HashMap;

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
    Wait(u16, SimResult<D, E>),
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
    fn process(&self) -> SimResult<D, E> {
        match self {
            SimResult::Ok(v) => Ok(v),
            SimResult::Err(e) => Err(e),
            SimResult::Wait(i, res) => {
                i -= 1;
                if i <= 1 {
                    res
                }

                SimResult::Wait(i, res)
            },
        }
    }
}

/// Represents unsigned integers which are smaller than 8 bits.
/// To perform computations on the 
struct SmallUInt {
    // Number of bits.
    size: u8,
    
    // Bits in little endian order.
    bits: [bool],
}

impl SmallUInt {
    /// Creates a SmallUInt from data. Only the first size bits of the u8 will
    /// be used.
    fn new(size: u8, data: u8) -> SmallUInt {
        let data = data.to_le_bytes()[0];
        let mut bits = [false; size];
        
        for i in 0..size {
            bits[i] = ((data & (1 << i)) >> i) as bool;
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
            accum |= self.bits[i] << i;
        }

        accum
    }
}


// Memory, A is the address type, D is the data type.
trait Memory<A, D> {
    fn get(self, address: A) -> SimResult<Option<D>, String>;
    fn set(self, address: A, data: D) -> SimResult<(), String>;
}

// Associative memory structure. Contains sets. Each set is selected by an index.
// Each set contains a number of lines, called ways.
struct AssocMem<'a, A, D> {
    // Number of ways in each set.
    nways: u8,

    // Stored data, organized in sets.
    sets: HashMap<A, Vec<AssocLine<A, D>>>,

    // Underlying memory used to service misses.
    base: Option<&'a dyn Memory<A, D>>,
}

// Line in an associative memory structure.
struct AssocLine<A, D> {
    // Line address, split into index and tag by the accessor of this data structure
    // because the number of bits for each field is not know by this data structure.
    address: A,

    // Keeps track of if this line was recently access, smaller number = recent,
    // larger number = old.
    lru: SmallUInt,
}

impl<'a, A, D> AssocMem<'a, A, D> {
    // Initializes with a AssocMem
    fn new(nways: u8, base: &'a dyn Memory<A, D>) -> AssocMem<A, D> {
        AssocMem{
            nways: nways,
            sets: HashMap::new(),
            base: base,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
