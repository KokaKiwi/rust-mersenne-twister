#![cfg_attr(test, feature(test))]
extern crate rand;
#[cfg(test)]
extern crate test;

pub use self::mt32::MTRng32;
pub use self::mt64::MTRng64;

#[cfg(target_pointer_width = "32")]
pub use self::mt32::MTRng32 as MTRng;
#[cfg(target_pointer_width = "64")]
pub use self::mt64::MTRng64 as MTRng;

pub mod mt32;
pub mod mt64;
