use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};

pub(crate) fn validate_descriptive_text(value: &str) -> Result<(), Denial> {
    if value.trim().is_empty() || value.trim() != value || value.chars().any(char::is_control) {
        return Err(Denial::new(Kind::InvalidEnvelopeText));
    }
    Ok(())
}
