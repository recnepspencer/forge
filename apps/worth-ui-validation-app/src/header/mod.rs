mod command_registration;
mod header_renderer;

pub(crate) use command_registration::register_header_command_capabilities;
pub use header_renderer::render_header_only;
