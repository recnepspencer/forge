use crate::planar_contracts::signed_area_2d::AreaDegeneracyClass;

use super::face_pair::{CanonicalFacePairIdentity, CertifiedCoplanarOverlapFace2D};
use super::overlap_rows::{
    AmbiguousContactRow, ContainmentRelationRow, OverlapIslandRow, PolicyRequiredExitRow,
    SharedIntervalRow,
};
use super::validation::validate_coplanar_overlap_basis;
use super::{CoplanarOverlapDenial, CoplanarOverlapPerformanceCounters, CoplanarOverlapPolicy};

#[derive(Clone, Debug, PartialEq)]
pub struct CoplanarOverlapContractBasis {
    first_face: CertifiedCoplanarOverlapFace2D,
    second_face: CertifiedCoplanarOverlapFace2D,
    planar_neighborhood_identity: String,
    policy: CoplanarOverlapPolicy,
    pair_identity: String,
    shared_intervals: Vec<SharedIntervalRow>,
    overlap_islands: Vec<OverlapIslandRow>,
    containment_relations: Vec<ContainmentRelationRow>,
    ambiguous_contacts: Vec<AmbiguousContactRow>,
    policy_required_exits: Vec<PolicyRequiredExitRow>,
    counters: Option<CoplanarOverlapPerformanceCounters>,
}

impl CoplanarOverlapContractBasis {
    pub(crate) fn new(
        first_face: CertifiedCoplanarOverlapFace2D,
        second_face: CertifiedCoplanarOverlapFace2D,
        planar_neighborhood_identity: String,
        policy: CoplanarOverlapPolicy,
    ) -> Result<Self, CoplanarOverlapDenial> {
        let pair_identity = CanonicalFacePairIdentity::from_faces(&first_face, &second_face);
        let basis = Self {
            first_face,
            second_face,
            planar_neighborhood_identity,
            policy,
            pair_identity: pair_identity.as_str().to_string(),
            shared_intervals: Vec::new(),
            overlap_islands: Vec::new(),
            containment_relations: Vec::new(),
            ambiguous_contacts: Vec::new(),
            policy_required_exits: Vec::new(),
            counters: None,
        };
        validate_coplanar_overlap_basis(&basis)?;
        Ok(basis)
    }

    pub(crate) fn with_rows(
        mut self,
        shared_intervals: Vec<SharedIntervalRow>,
        overlap_islands: Vec<OverlapIslandRow>,
        containment_relations: Vec<ContainmentRelationRow>,
        ambiguous_contacts: Vec<AmbiguousContactRow>,
        policy_required_exits: Vec<PolicyRequiredExitRow>,
        counters: CoplanarOverlapPerformanceCounters,
    ) -> Self {
        self.shared_intervals = shared_intervals;
        self.overlap_islands = overlap_islands;
        self.containment_relations = containment_relations;
        self.ambiguous_contacts = ambiguous_contacts;
        self.policy_required_exits = policy_required_exits;
        self.counters = Some(counters);
        self
    }

    pub fn first_face(&self) -> &CertifiedCoplanarOverlapFace2D {
        &self.first_face
    }

    pub fn second_face(&self) -> &CertifiedCoplanarOverlapFace2D {
        &self.second_face
    }

    pub fn pair_identity(&self) -> &str {
        &self.pair_identity
    }

    pub fn planar_neighborhood_identity(&self) -> &str {
        &self.planar_neighborhood_identity
    }

    pub fn policy(&self) -> CoplanarOverlapPolicy {
        self.policy
    }

    pub fn shared_intervals(&self) -> &[SharedIntervalRow] {
        &self.shared_intervals
    }

    pub fn overlap_islands(&self) -> &[OverlapIslandRow] {
        &self.overlap_islands
    }

    pub fn containment_relations(&self) -> &[ContainmentRelationRow] {
        &self.containment_relations
    }

    pub fn ambiguous_contacts(&self) -> &[AmbiguousContactRow] {
        &self.ambiguous_contacts
    }

    pub fn policy_required_exits(&self) -> &[PolicyRequiredExitRow] {
        &self.policy_required_exits
    }

    pub fn counters(&self) -> CoplanarOverlapPerformanceCounters {
        self.counters.expect("certified overlap basis has counters")
    }

    pub(crate) fn area_policy_required(&self) -> bool {
        self.first_face.signed_area_receipt().degeneracy() == AreaDegeneracyClass::PolicyRequired
            || self.second_face.signed_area_receipt().degeneracy()
                == AreaDegeneracyClass::PolicyRequired
    }
}
