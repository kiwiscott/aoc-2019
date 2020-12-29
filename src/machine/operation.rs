#![allow(unused_imports)]
use crate::machine::Operation;

mod add;
mod mul;
mod input;
mod output;
mod terminate;
mod jump_if_false;
mod jump_if_true;
mod equals;
mod less_than; 


pub use add::Add;
pub use mul::Mul;
pub use output::Output;
pub use input::Input;
pub use jump_if_false::JumpIfFalse;
pub use jump_if_true::JumpIfTrue;
pub use equals::Equals;
pub use less_than::LessThan; 

pub use terminate::Terminate;
