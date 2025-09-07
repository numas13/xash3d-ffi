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

#[macro_use]
mod macros;

use core::ffi::{c_int, c_ushort};

pub use libc::{off_t, FILE};

pub type uint = core::ffi::c_uint;
pub type va_list = core::ffi::c_void;

include!("ffi.rs");

// model_t->flags
pub const MODEL_CONVEYOR: c_int = 1 << 0;
pub const MODEL_HAS_ORIGIN: c_int = 1 << 1;
pub const MODEL_LIQUID: c_int = 1 << 2;
pub const MODEL_TRANSPARENT: c_int = 1 << 3;
pub const MODEL_COLORED_LIGHTING: c_int = 1 << 4;
pub const MODEL_QBSP2: c_int = 1 << 28;
pub const MODEL_WORLD: c_int = 1 << 29;
pub const MODEL_CLIENT: c_int = 1 << 30;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mnode_s {
    pub contents: c_int,
    pub visframe: c_int,
    pub minmaxs: [f32; 6],
    pub parent: *mut mnode_s,
    pub plane: *mut mplane_s,
    pub children: [*mut mnode_s; 2],
    pub firstsurface_0: c_ushort,
    pub numsurfaces_0: c_ushort,
    #[cfg(target_pointer_width = "64")]
    pub firstsurface_1: c_ushort,
    #[cfg(target_pointer_width = "64")]
    pub numsurfaces_1: c_ushort,
}

impl mnode_s {
    #[cfg(target_pointer_width = "32")]
    pub fn children(&self, model: &model_s, side: usize) -> *mut mnode_s {
        if model.flags & MODEL_QBSP2 != 0 {
            let raw = self.children[side] as usize;
            let offset = (raw & 0xfffffe) >> 1;
            if raw & 1 == 0 {
                return model.leafs.wrapping_add(offset) as *mut mnode_s;
            } else {
                return model.nodes.wrapping_add(offset);
            }
        }
        self.children[side]
    }

    fn firstsurface_0(&self) -> u32 {
        self.firstsurface_0 as u32
    }

    fn numsurfaces_0(&self) -> u32 {
        self.numsurfaces_0 as u32
    }

    #[cfg(target_pointer_width = "32")]
    fn firstsurface_1(&self) -> u32 {
        (self.children[0] as u32) >> 24
    }

    #[cfg(target_pointer_width = "32")]
    fn numsurfaces_1(&self) -> u32 {
        (self.children[1] as u32) >> 24
    }

    #[cfg(not(target_pointer_width = "32"))]
    fn firstsurface_1(&self) -> u32 {
        self.firstsurface_1 as u32
    }

    #[cfg(not(target_pointer_width = "32"))]
    fn numsurfaces_1(&self) -> u32 {
        self.numsurfaces_1 as u32
    }

    pub fn first_surface(&self, model: &model_s) -> u32 {
        if model.flags & MODEL_QBSP2 != 0 {
            self.firstsurface_0() + (self.firstsurface_1() << 16)
        } else {
            self.firstsurface_0()
        }
    }

    pub fn num_surfaces(&self, model: &model_s) -> u32 {
        if model.flags & MODEL_QBSP2 != 0 {
            self.numsurfaces_0() + (self.numsurfaces_1() << 16)
        } else {
            self.numsurfaces_0()
        }
    }
}

// engine/sprite.h
const_assert_size!(dsprite_t, 8);
const_assert_size!(dsprite_q1_t, 36);
const_assert_size!(dsprite_hl_t, 40);
const_assert_size!(dspriteframe_t, 16);
const_assert_size!(dspritegroup_t, 4);
const_assert_size!(dspriteinterval_t, 4);
const_assert_size!(dframetype_t, 4);

// engine/custom.h
const_assert_size!(customization_t, 164, 192);
const_assert_size!(resource_t, 136, 144);

// common/cvardef.h
const_assert_size!(cvar_s, 20, 32);

// common/netadr.h
const_assert_size!(netadr_t, 20);

// common/kbutton.h
const_assert_size!(kbutton_t, 12);

// common/com_model.h
const_assert_size!(mnode_t, 52, 72);
const_assert_size!(mextrasurf_t, 324, 496);
const_assert_size!(decal_t, 60, 88);
const_assert_size!(mfaceinfo_t, 176, 304);
