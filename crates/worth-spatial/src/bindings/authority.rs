use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialConstructionBirthAuthority {
    boundary_name: &'static str,
    authority_scope: &'static str,
    authority_digest: String,
}

impl SpatialConstructionBirthAuthority {
    pub(crate) fn new() -> Self {
        let boundary_name = "worth-spatial.construction-birth-authority";
        let authority_scope = "construction_time_topology_geometry_birth_truth";
        let authority_digest = digest_parts(&[boundary_name, authority_scope]);
        Self {
            boundary_name,
            authority_scope,
            authority_digest,
        }
    }

    pub fn boundary_name(&self) -> &str {
        self.boundary_name
    }

    pub fn authority_scope(&self) -> &str {
        self.authority_scope
    }

    pub fn authority_digest(&self) -> &str {
        &self.authority_digest
    }
}

pub fn construction_birth_authority() -> SpatialConstructionBirthAuthority {
    SpatialConstructionBirthAuthority::new()
}

fn digest_parts(parts: &[&str]) -> String {
    let mut hasher = DefaultHasher::new();
    for part in parts {
        part.hash(&mut hasher);
    }
    format!("{:016x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::construction_birth_authority;

    #[test]
    fn construction_birth_authority_exposes_named_boundary() {
        let authority = construction_birth_authority();
        assert_eq!(
            authority.boundary_name(),
            "worth-spatial.construction-birth-authority"
        );
        assert_eq!(
            authority.authority_scope(),
            "construction_time_topology_geometry_birth_truth"
        );
        assert!(!authority.authority_digest().is_empty());
    }
}
