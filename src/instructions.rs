use crate::result::SimResult;

/// Defines operations which a single instruction must perform while it is in
/// the pipeline.
pub trait Instruction {
    /// Extracts parameters from instruction bits and stores them in the
    /// implementing struct for use by future stages. It also retrieves register
    /// values if necessary and does the same.
    fn decode_and_fetch(
}
