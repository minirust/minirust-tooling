use std::collections::{HashSet, HashMap};

// contains the BigInt implemenation.
// accessed using `BigInt`.
mod bigint;
pub use bigint::BigInt;

// contains hidden functions that are called only due to generated code.
// accessed using `baselib::hidden::_`.
pub mod hidden;

// contains items to be exposed to the user without importing.
// like List, Set etc.
// accessed using `_`.
#[macro_use]
mod env;
pub use env::*;

// contains implementation for opaque items from MiniRust, which are not already defined in mirror.
// accessed using `baselib::_`.
#[macro_use]
mod intrinsics;
pub use intrinsics::*;

pub mod prelude {
    pub use crate::baselib::BigInt;
    pub use crate::baselib::env::*;
}
