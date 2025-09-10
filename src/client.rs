//! Bindings to a client DLL.

use crate::common::*;
use crate::player_move::*;

use crate::api::efx::*;
use crate::api::event::*;
use crate::api::net::*;
use crate::api::render::*;
use crate::api::studio::*;
use crate::api::tri::*;

include!("generated/client.rs");
