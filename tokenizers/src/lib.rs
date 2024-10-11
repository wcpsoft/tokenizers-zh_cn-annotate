#![warn(clippy::all)]
#![allow(clippy::upper_case_acronyms)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate derive_builder;

#[macro_use]
pub mod utils;
pub mod decoders;
pub mod models;
pub mod normalizers;
pub mod pre_tokenizers;
pub mod processors;
pub mod tokenizer;

// Re-export from tokenizer
pub use tokenizer::*;

// Re-export also parallelism utils
pub use utils::parallelism;

// Re-export for from_pretrained
#[cfg(feature = "http")]
pub use utils::from_pretrained::FromPretrainedParameters;
