mod artifact;
mod body;
mod bounded_serialization;
mod canonicalization;
mod contract;
mod typed_value;
mod validation;

pub use artifact::{
    WorthServerProductResultArtifact, WorthServerProductResultArtifactError,
    WorthServerProductResultArtifactErrorCode,
};
pub use body::WorthServerProductResultBody;
pub use contract::{
    WorthServerProductResultCanonicalization, WorthServerProductResultContract,
    WorthServerProductResultEncoding, WorthServerProductResultSchema,
};
pub use typed_value::WorthServerProductResultValue;
pub use validation::{
    WorthServerProductResultContractError, WorthServerProductResultContractErrorCode,
};

pub(crate) use canonicalization::{canonicalize_json, sha256_hex};
pub(crate) use validation::artifact_matches_contract;

fn hex_digest(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}
