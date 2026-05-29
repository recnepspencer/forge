mod aspect_value_canonical_codec;
mod aspect_value_locator_canonical_codec;
mod field_patch_canonical_codec;

pub(crate) use aspect_value_canonical_codec::{
    decode_aspect_value, encode_aspect_value, encode_length_prefixed_aspect_value, encode_string,
    encode_u32, serde_canonical_aspect_value, AspectValueCanonicalCodecError,
};
pub(crate) use aspect_value_locator_canonical_codec::serde_canonical_aspect_value_locator;
#[cfg(test)]
pub(crate) use aspect_value_locator_canonical_codec::{
    decode_aspect_value_locator, encode_aspect_value_locator,
};
pub use field_patch_canonical_codec::AspectFieldPatchCodecError;
pub(crate) use field_patch_canonical_codec::{
    decode_aspect_field_patch_canonical_bytes, decode_aspect_field_patch_target_canonical_bytes,
    encode_aspect_field_patch_canonical_bytes, encode_aspect_field_patch_target_canonical_bytes,
};

#[cfg(test)]
mod aspect_value_canonical_codec_tests;
#[cfg(test)]
mod aspect_value_locator_canonical_codec_tests;
