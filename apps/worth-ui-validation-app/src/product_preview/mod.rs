mod catalog;
#[cfg(test)]
mod preview_telemetry;

pub(crate) use catalog::{
    register_preview_component_capabilities, register_preview_icon_capabilities,
    register_preview_image_asset_capabilities, register_preview_surface_capabilities,
    PREVIEW_DEFAULT_PAGE,
};
