mod aes;
mod hash;
mod hmac;
mod dllmain;
mod utils {
    pub mod cstring;
    pub mod aes;
    pub mod base64;
    pub mod hash;
    pub mod hmac;
}

pub use crate::{
    utils::*,
};

pub const ERR: &str = "ERR|";