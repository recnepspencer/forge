use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyConstructionAuthority {
    boundary_name: &'static str,
    authority_scope: &'static str,
    authority_digest: String,
}

impl TopologyConstructionAuthority {
    pub(crate) fn new() -> Self {
        let boundary_name = "worth-topo.construction-authority";
        let authority_scope = "topology_legality_and_execution_authority";
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

pub fn topology_construction_authority() -> TopologyConstructionAuthority {
    TopologyConstructionAuthority::new()
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
    use super::topology_construction_authority;

    #[test]
    fn topology_construction_authority_exposes_named_boundary() {
        let authority = topology_construction_authority();
        assert_eq!(
            authority.boundary_name(),
            "worth-topo.construction-authority"
        );
        assert_eq!(
            authority.authority_scope(),
            "topology_legality_and_execution_authority"
        );
        assert!(!authority.authority_digest().is_empty());
    }
}
