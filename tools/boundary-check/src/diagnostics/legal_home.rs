#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct LegalHome(String);

impl LegalHome {
    pub(super) fn new(pointer: impl Into<String>) -> Result<Self, String> {
        let pointer = pointer.into();
        if pointer.trim().is_empty() {
            return Err("diagnostic legal_home must not be empty".to_owned());
        }
        let first = pointer.split_whitespace().next().unwrap_or_default();
        if !first.contains('/') && first != "Cargo.toml" {
            return Err("diagnostic legal_home must name a machine artifact first".to_owned());
        }
        Ok(Self(pointer))
    }

    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legal_home_requires_an_artifact_first_pointer() {
        assert!(LegalHome::new("   ").is_err());
        assert!(LegalHome::new("put this in a facade").is_err());
        assert!(LegalHome::new("Cargo.toml [workspace]").is_ok());
        assert!(LegalHome::new("tools/boundary-check/config/road1.toml [naming]").is_ok());
    }
}
