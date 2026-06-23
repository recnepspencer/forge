pub const VALIDATION_SAMPLE_MODULE_PATH: &str = "validation/header.wui";
pub const VALIDATION_SAMPLE_THEME_SOURCE: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/theme/header.theme"));
pub const VALIDATION_SAMPLE_COMMAND_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/theme/header.commands"
));
pub const VALIDATION_SAMPLE_COMMAND_PROJECTION_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/theme/header.projections"
));
pub const VALIDATION_SAMPLE_COMPONENT_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/theme/header.components"
));
pub const VALIDATION_SAMPLE_APPEARANCE_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/theme/header.appearance"
));
pub const VALIDATION_SAMPLE_DENSITY_SOURCE: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/theme/header.density"));

pub const VALIDATION_SAMPLE_SOURCE: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/source/header.wui"));
