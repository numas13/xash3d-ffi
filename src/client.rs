//! Bindings to a client DLL.

use core::{ffi::c_int, mem};

use crate::common::*;
use crate::player_move::*;

use crate::api::efx::*;
use crate::api::event::*;
use crate::api::net::*;
use crate::api::render::*;
use crate::api::studio::*;
use crate::api::tri::*;

include!("generated/client.rs");

impl Default for SCREENINFO {
    fn default() -> SCREENINFO {
        Self {
            iSize: mem::size_of::<Self>() as c_int,
            ..unsafe { mem::zeroed() }
        }
    }
}
