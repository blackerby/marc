const FIELD_TERMINATOR: usize = 0x1E;
const SUBFIELD_TERMINATOR: usize = 0x1F;
const RECORD_TERMINATOR: usize = 0x1D;

pub mod control_field;
pub mod data_field;
pub mod errors;
pub mod leader;
pub mod subfield;
