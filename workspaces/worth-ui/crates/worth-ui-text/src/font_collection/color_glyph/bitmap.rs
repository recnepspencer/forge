mod cbdt;
mod sbix;

pub(in crate::font_collection) use cbdt::validate as validate_cbdt;
pub(in crate::font_collection) use sbix::validate as validate_sbix;
