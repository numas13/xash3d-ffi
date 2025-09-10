//! Bindings to shared APIs.

#[cfg(feature = "studio-api")]
pub mod studio {
    use crate::common::*;

    include!("generated/studio_api.rs");
}

#[cfg(feature = "render-api")]
pub mod render {
    use crate::api::studio::*;
    use crate::common::*;

    include!("generated/render_api.rs");
}

#[cfg(feature = "net-api")]
pub mod net {
    use crate::common::*;

    include!("generated/net_api.rs");
}

#[cfg(feature = "fs-api")]
pub mod fs {
    use crate::common::*;

    include!("generated/fs_api.rs");
}

#[cfg(feature = "tri-api")]
pub mod tri {
    use crate::common::*;

    include!("generated/tri_api.rs");
}

#[cfg(feature = "event-api")]
pub mod event {
    use crate::common::*;

    include!("generated/event_api.rs");
}

#[cfg(feature = "efx-api")]
pub mod efx {
    use crate::common::*;

    include!("generated/efx_api.rs");
}

#[cfg(feature = "phys-api")]
pub mod phys {
    use crate::common::*;

    use crate::api::render::*;
    use crate::api::tri::*;

    use crate::server::SAVERESTOREDATA;

    include!("generated/phys_api.rs");

    pub type PHYSICAPI = Option<
        unsafe extern "C" fn(
            version: core::ffi::c_int,
            eng_funcs: *mut server_physics_api_t,
            dll_funcs: *mut physics_interface_t,
        ) -> core::ffi::c_int,
    >;
}
