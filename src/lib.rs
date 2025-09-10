//! Raw FFI bindings to [Xash3D FWGS engine](https://github.com/FWGS/xash3d-fwgs).

#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::type_complexity)]

#[cfg(feature = "std")]
extern crate std;

#[macro_use]
mod macros;

pub mod common;

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
