#![recursion_limit = "256"]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::declare_interior_mutable_const)]
#![cfg_attr(not(feature = "std"), no_std)]
//! `util` contains all objects and relationships for all sunshine-bounty modules
//! - see `traits.rs` for behavioral definitions and other files for object impls
#[macro_use]
extern crate derive_new;

pub mod bounty;
pub mod organization;
pub mod share;
pub mod traits;
