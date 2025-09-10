//! Raw FFI bindings to [Xash3D FWGS engine](https://github.com/FWGS/xash3d-fwgs).

#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::type_complexity)]
// unnecessary_transmutes lint is not supported by rustc-1.64
#![allow(unknown_lints)]
#![allow(unnecessary_transmutes)]
#![warn(unknown_lints)]

#[cfg(feature = "std")]
extern crate std;

#[macro_use]
mod macros;

pub mod api;
pub mod common;

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "menu")]
pub mod menu;
#[cfg(feature = "player-move")]
pub mod player_move;
#[cfg(feature = "render")]
pub mod render;
#[cfg(feature = "server")]
pub mod server;
