#![allow(unused_imports)]
use crate::machine::Operation;

mod add;
mod equals;
mod input;
mod jump_if_false;
mod jump_if_true;
mod less_than;
mod mul;
mod output;
mod terminate;
mod relative_base;

pub use add::Add;
pub use equals::Equals;
pub use input::Input;
pub use jump_if_false::JumpIfFalse;
pub use jump_if_true::JumpIfTrue;
pub use less_than::LessThan;
pub use mul::Mul;
pub use output::Output;
pub use relative_base::RelativeBase; 

pub use terminate::Terminate;
