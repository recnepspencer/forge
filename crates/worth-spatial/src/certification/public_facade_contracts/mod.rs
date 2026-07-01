mod compile_fail_closeout;
#[cfg(test)]
mod compile_fail_contracts;
mod phase_fifteen_fixture_inventory;

pub use compile_fail_closeout::{
    current_spatial_public_facade_compile_fail_closeout,
    spatial_public_facade_compile_fail_closeout_excluding_fence_class_for_tests,
    SpatialPublicFacadeCompileFailCloseout, SpatialPublicFacadeCompileFailCloseoutError,
    SpatialPublicFacadeCompileFailCloseoutErrorKind,
};
