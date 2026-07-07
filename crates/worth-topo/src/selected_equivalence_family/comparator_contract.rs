use serde::{Deserialize, Serialize};

use crate::compiled_product_family::DeterministicDigest;

use super::family_identity::TopologySelectedEquivalenceFamilyIdentity;
use super::posture::{
    TopologyCompatibilityPosture, TopologyFreshnessRequirementPosture,
    TopologyOrderingNoisePosture, TopologyRenderedOutputComparisonPosture,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologySelectedEquivalenceDimension {
    SelectedEquivalenceBasisIdentity,
    SelectedReuseBasisIdentity,
    MaterializedTopologyDigest,
    InterpretedTopologyDigest,
    DerivedValidationDigest,
}

impl TopologySelectedEquivalenceDimension {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::SelectedEquivalenceBasisIdentity => "selected-equivalence-basis-identity",
            Self::SelectedReuseBasisIdentity => "selected-reuse-basis-identity",
            Self::MaterializedTopologyDigest => "materialized-topology-digest",
            Self::InterpretedTopologyDigest => "interpreted-topology-digest",
            Self::DerivedValidationDigest => "derived-validation-digest",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologySelectedEquivalenceComparatorContract {
    family_identity: TopologySelectedEquivalenceFamilyIdentity,
    equivalence_policy_identity_digest: String,
    equivalence_dimensions: Vec<TopologySelectedEquivalenceDimension>,
    compatibility_posture: TopologyCompatibilityPosture,
    freshness_requirement_posture: TopologyFreshnessRequirementPosture,
    ordering_noise_posture: TopologyOrderingNoisePosture,
    rendered_output_comparison_posture: TopologyRenderedOutputComparisonPosture,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySelectedEquivalenceComparable<'a> {
    family_identity: Option<TopologySelectedEquivalenceFamilyIdentity>,
    selected_equivalence_basis_identity_digest: Option<&'a str>,
    selected_reuse_basis_identity_digest: Option<&'a str>,
    ordering_noise_posture: Option<TopologyOrderingNoisePosture>,
    rendered_output_comparison_posture: Option<TopologyRenderedOutputComparisonPosture>,
    materialized_topology_digest: Option<&'a DeterministicDigest>,
    interpreted_topology_digest: Option<&'a DeterministicDigest>,
    derived_validation_digest: Option<&'a DeterministicDigest>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologySelectedEquivalenceComparisonReport {
    pub comparison_supported: bool,
    pub unsupported_comparison_reason: Option<String>,
    pub selected_equivalence_basis_identity_match: bool,
    pub selected_reuse_basis_identity_match: bool,
    pub materialized_topology_digest_match: bool,
    pub interpreted_topology_digest_match: bool,
    pub derived_validation_digest_match: bool,
    pub equivalent_derived_meaning: bool,
}

impl TopologySelectedEquivalenceComparatorContract {
    pub(crate) fn new(
        family_identity: TopologySelectedEquivalenceFamilyIdentity,
        equivalence_policy_identity_digest: String,
        equivalence_dimensions: Vec<TopologySelectedEquivalenceDimension>,
        compatibility_posture: TopologyCompatibilityPosture,
        freshness_requirement_posture: TopologyFreshnessRequirementPosture,
        ordering_noise_posture: TopologyOrderingNoisePosture,
        rendered_output_comparison_posture: TopologyRenderedOutputComparisonPosture,
    ) -> Self {
        Self {
            family_identity,
            equivalence_policy_identity_digest,
            equivalence_dimensions,
            compatibility_posture,
            freshness_requirement_posture,
            ordering_noise_posture,
            rendered_output_comparison_posture,
        }
    }

    #[cfg(test)]
    pub fn family_identity(&self) -> TopologySelectedEquivalenceFamilyIdentity {
        self.family_identity
    }

    #[cfg(test)]
    pub fn equivalence_policy_identity_digest(&self) -> &str {
        &self.equivalence_policy_identity_digest
    }

    #[cfg(test)]
    pub fn equivalence_dimensions(&self) -> &[TopologySelectedEquivalenceDimension] {
        &self.equivalence_dimensions
    }

    #[cfg(test)]
    pub const fn compatibility_posture(&self) -> TopologyCompatibilityPosture {
        self.compatibility_posture
    }

    #[cfg(test)]
    pub const fn freshness_requirement_posture(&self) -> TopologyFreshnessRequirementPosture {
        self.freshness_requirement_posture
    }

    #[cfg(test)]
    pub const fn ordering_noise_posture(&self) -> TopologyOrderingNoisePosture {
        self.ordering_noise_posture
    }

    #[cfg(test)]
    pub const fn rendered_output_comparison_posture(
        &self,
    ) -> TopologyRenderedOutputComparisonPosture {
        self.rendered_output_comparison_posture
    }

    #[cfg(test)]
    pub(crate) fn with_test_equivalence_dimensions(
        mut self,
        dimensions: Vec<TopologySelectedEquivalenceDimension>,
    ) -> Self {
        self.equivalence_dimensions = dimensions;
        self
    }

    pub fn compare(
        &self,
        lhs: &TopologySelectedEquivalenceComparable<'_>,
        rhs: &TopologySelectedEquivalenceComparable<'_>,
    ) -> TopologySelectedEquivalenceComparisonReport {
        let unsupported_reason = self.unsupported_comparison_reason(lhs, rhs);
        let selected_equivalence_basis_identity_match = lhs
            .selected_equivalence_basis_identity_digest
            == rhs.selected_equivalence_basis_identity_digest;
        let selected_reuse_basis_identity_match =
            lhs.selected_reuse_basis_identity_digest == rhs.selected_reuse_basis_identity_digest;
        let materialized_topology_digest_match =
            lhs.materialized_topology_digest == rhs.materialized_topology_digest;
        let interpreted_topology_digest_match =
            lhs.interpreted_topology_digest == rhs.interpreted_topology_digest;
        let derived_validation_digest_match =
            lhs.derived_validation_digest == rhs.derived_validation_digest;
        let equivalent_derived_meaning = unsupported_reason.is_none()
            && self
                .equivalence_dimensions
                .iter()
                .all(|dimension| match dimension {
                    TopologySelectedEquivalenceDimension::SelectedEquivalenceBasisIdentity => {
                        selected_equivalence_basis_identity_match
                    }
                    TopologySelectedEquivalenceDimension::SelectedReuseBasisIdentity => {
                        selected_reuse_basis_identity_match
                    }
                    TopologySelectedEquivalenceDimension::MaterializedTopologyDigest => {
                        materialized_topology_digest_match
                    }
                    TopologySelectedEquivalenceDimension::InterpretedTopologyDigest => {
                        interpreted_topology_digest_match
                    }
                    TopologySelectedEquivalenceDimension::DerivedValidationDigest => {
                        derived_validation_digest_match
                    }
                });
        TopologySelectedEquivalenceComparisonReport {
            comparison_supported: unsupported_reason.is_none(),
            unsupported_comparison_reason: unsupported_reason,
            selected_equivalence_basis_identity_match,
            selected_reuse_basis_identity_match,
            materialized_topology_digest_match,
            interpreted_topology_digest_match,
            derived_validation_digest_match,
            equivalent_derived_meaning,
        }
    }

    fn unsupported_comparison_reason(
        &self,
        lhs: &TopologySelectedEquivalenceComparable<'_>,
        rhs: &TopologySelectedEquivalenceComparable<'_>,
    ) -> Option<String> {
        if lhs.family_identity != Some(self.family_identity)
            || rhs.family_identity != Some(self.family_identity)
        {
            return Some(
                "topology reports did not carry the selected family comparator contract".into(),
            );
        }
        if lhs.ordering_noise_posture != Some(self.ordering_noise_posture)
            || rhs.ordering_noise_posture != Some(self.ordering_noise_posture)
        {
            return Some("topology reports declared different ordering postures".into());
        }
        if lhs.rendered_output_comparison_posture != Some(self.rendered_output_comparison_posture)
            || rhs.rendered_output_comparison_posture
                != Some(self.rendered_output_comparison_posture)
        {
            return Some("topology reports declared different rendered-output postures".into());
        }
        self.equivalence_dimensions
            .iter()
            .find_map(|dimension| missing_dimension_reason(*dimension, lhs, rhs))
    }
}

impl<'a> TopologySelectedEquivalenceComparable<'a> {
    pub(crate) fn new(
        family_identity: Option<TopologySelectedEquivalenceFamilyIdentity>,
        selected_equivalence_basis_identity_digest: Option<&'a str>,
        selected_reuse_basis_identity_digest: Option<&'a str>,
        ordering_noise_posture: Option<TopologyOrderingNoisePosture>,
        rendered_output_comparison_posture: Option<TopologyRenderedOutputComparisonPosture>,
        materialized_topology_digest: Option<&'a DeterministicDigest>,
        interpreted_topology_digest: Option<&'a DeterministicDigest>,
        derived_validation_digest: Option<&'a DeterministicDigest>,
    ) -> Self {
        Self {
            family_identity,
            selected_equivalence_basis_identity_digest,
            selected_reuse_basis_identity_digest,
            ordering_noise_posture,
            rendered_output_comparison_posture,
            materialized_topology_digest,
            interpreted_topology_digest,
            derived_validation_digest,
        }
    }
}

impl TopologySelectedEquivalenceComparisonReport {
    pub(crate) fn unsupported(reason: impl Into<String>) -> Self {
        Self {
            comparison_supported: false,
            unsupported_comparison_reason: Some(reason.into()),
            selected_equivalence_basis_identity_match: false,
            selected_reuse_basis_identity_match: false,
            materialized_topology_digest_match: false,
            interpreted_topology_digest_match: false,
            derived_validation_digest_match: false,
            equivalent_derived_meaning: false,
        }
    }
}

fn missing_dimension_reason(
    dimension: TopologySelectedEquivalenceDimension,
    lhs: &TopologySelectedEquivalenceComparable<'_>,
    rhs: &TopologySelectedEquivalenceComparable<'_>,
) -> Option<String> {
    let dimension_missing = match dimension {
        TopologySelectedEquivalenceDimension::SelectedEquivalenceBasisIdentity => {
            lhs.selected_equivalence_basis_identity_digest.is_none()
                || rhs.selected_equivalence_basis_identity_digest.is_none()
        }
        TopologySelectedEquivalenceDimension::SelectedReuseBasisIdentity => {
            lhs.selected_reuse_basis_identity_digest.is_none()
                || rhs.selected_reuse_basis_identity_digest.is_none()
        }
        TopologySelectedEquivalenceDimension::MaterializedTopologyDigest => {
            lhs.materialized_topology_digest.is_none() || rhs.materialized_topology_digest.is_none()
        }
        TopologySelectedEquivalenceDimension::InterpretedTopologyDigest => {
            lhs.interpreted_topology_digest.is_none() || rhs.interpreted_topology_digest.is_none()
        }
        TopologySelectedEquivalenceDimension::DerivedValidationDigest => {
            lhs.derived_validation_digest.is_none() || rhs.derived_validation_digest.is_none()
        }
    };
    dimension_missing.then(|| {
        format!(
            "topology selected family requires declared comparison dimension {}",
            dimension.as_str()
        )
    })
}
