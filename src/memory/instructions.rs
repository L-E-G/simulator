

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

trait Instruction<I> {
    fn addUI(&mut self, instruction: I) -> SimResult<(), String>;
    fn addSI(&mut self, instruction: I) -> SimResult<(), String>;
    fn addF(&mut self, instruction: I) -> SimResult<(), String>;
    fn addUIImm(&mut self, instruction: I) -> SimResult<(), String>;
    fn addSIImm(&mut self, instruction: I) -> SimResult<(), String>;
    fn addFImm(&mut self, instruction: I) -> SimResult<(), String>;

    fn subUI(&mut self, instruction: I) -> SimResult<(), String>;
    fn subSI(&mut self, instruction: I) -> SimResult<(), String>;
    fn subF(&mut self, instruction: I) -> SimResult<(), String>;
    fn subUIImm(&mut self, instruction: I) -> SimResult<(), String>;
    fn subSIImm(&mut self, instruction: I) -> SimResult<(), String>;
    fn subFImm(&mut self, instruction: I) -> SimResult<(), String>;

    fn divUI(&mut self, instruction: I) -> SimResult<(), String>;
    fn divSI(&mut self, instruction: I) -> SimResult<(), String>;
    fn divF(&mut self, instruction: I) -> SimResult<(), String>;
    fn divUIImm(&mut self, instruction: I) -> SimResult<(), String>;
    fn divSIImm(&mut self, instruction: I) -> SimResult<(), String>;
    fn divFImm(&mut self, instruction: I) -> SimResult<(), String>;

    fn mltUI(&mut self, instruction: I) -> SimResult<(), String>;
    fn mltSI(&mut self, instruction: I) -> SimResult<(), String>;
    fn mltF(&mut self, instruction: I) -> SimResult<(), String>;
    fn mltUIImm(&mut self, instruction: I) -> SimResult<(), String>;
    fn mltSIImm(&mut self, instruction: I) -> SimResult<(), String>;
    fn mltFImm(&mut self, instruction: I) -> SimResult<(), String>;

    fn mv(&mut self, instruction: I) -> SimResult<(), String>;

    fn cmpUI(&mut self, instruction: I) -> SimResult<(), String>;
    fn cmpSI(&mut self, instruction: I) -> SimResult<(), String>;
    fn cmpF(&mut self, instruction: I) -> SimResult<(), String>;

    fn asl(&mut self, instruction: I) -> SimResult<(), String>;
    fn asr(&mut self, instruction: I) -> SimResult<(), String>;
    fn aslImm(&mut self, instruction: I) -> SimResult<(), String>;
    fn asrImm(&mut self, instruction: I) -> SimResult<(), String>;
    fn lsl(&mut self, instruction: I) -> SimResult<(), String>;
    fn lsr(&mut self, instruction: I) -> SimResult<(), String>;
    fn lslImm(&mut self, instruction: I) -> SimResult<(), String>;
    fn lsrImm(&mut self, instruction: I) -> SimResult<(), String>;

    fn and(&mut self, instruction: I) -> SimResult<(), String>;
    fn or(&mut self, instruction: I) -> SimResult<(), String>;
    fn xor(&mut self, instruction: I) -> SimResult<(), String>;
    fn andImm(&mut self, instruction: I) -> SimResult<(), String>;
    fn orImm(&mut self, instruction: I) -> SimResult<(), String>;
    fn xorImm(&mut self, instruction: I) -> SimResult<(), String>;

    fn not(&mut self, instruction: I) -> SimResult<(), String>;

    fn ldr(&mut self, instruction: I) -> SimResult<(), String>;
    fn str(&mut self, instruction: I) -> SimResult<(), String>;
    fn push(&mut self, instruction: I) -> SimResult<(), String>;
    fn pop(&mut self, instruction: I) -> SimResult<(), String>;

    fn jmp(&mut self, instruction: I) -> SimResult<(), String>;
    fn jmpImm(&mut self, instruction: I) -> SimResult<(), String>;
    fn jmpS(&mut self, instruction: I) -> SimResult<(), String>;
    fn jmpSImm(&mut self, instruction: I) -> SimResult<(), String>;

    fn sih(&mut self, instruction: I) -> SimResult<(), String>;
    fn int(&mut self, instruction: I) -> SimResult<(), String>;
    fn ijmp(&mut self, instruction: I) -> SimResult<(), String>;
}

