#![doc = include_str!("../README.md")]
#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::type_complexity)]

#[cfg(feature = "std")]
extern crate std;

#[macro_use]
mod macros;

pub mod common;

pub mod keys {
    //! Key numbers definitions.
    include!("generated/keys.rs");
}

pub mod api;

#[cfg(feature = "player-move")]
pub mod player_move;

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "menu")]
pub mod menu;

#[cfg(feature = "render")]
pub mod render;
