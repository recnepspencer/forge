use super::CoplanarOverlapContractBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CoplanarOverlapIdentityEntry {
    locus: &'static str,
    value: String,
}

impl CoplanarOverlapIdentityEntry {
    fn text(locus: &'static str, value: impl Into<String>) -> Self {
        Self {
            locus,
            value: value.into(),
        }
    }

    pub(crate) fn locus(&self) -> &'static str {
        self.locus
    }

    pub(crate) fn value(&self) -> &str {
        &self.value
    }
}

pub(crate) fn coplanar_overlap_contract_identity_entries(
    basis: &CoplanarOverlapContractBasis,
) -> Vec<CoplanarOverlapIdentityEntry> {
    let mut area_facts = [
        basis
            .first_face()
            .signed_area_receipt()
            .fact_digest()
            .to_string(),
        basis
            .second_face()
            .signed_area_receipt()
            .fact_digest()
            .to_string(),
    ];
    area_facts.sort();
    let mut entries = vec![
        CoplanarOverlapIdentityEntry::text("geometry.coplanar_overlap.pair", basis.pair_identity()),
        CoplanarOverlapIdentityEntry::text(
            "geometry.coplanar_overlap.planar_neighborhood",
            basis.planar_neighborhood_identity(),
        ),
        CoplanarOverlapIdentityEntry::text(
            "geometry.coplanar_overlap.policy",
            basis.policy().as_str(),
        ),
        CoplanarOverlapIdentityEntry::text(
            "geometry.coplanar_overlap.first_area_fact",
            area_facts[0].clone(),
        ),
        CoplanarOverlapIdentityEntry::text(
            "geometry.coplanar_overlap.second_area_fact",
            area_facts[1].clone(),
        ),
    ];
    entries.extend(
        basis
            .shared_intervals()
            .iter()
            .enumerate()
            .map(|(index, row)| {
                CoplanarOverlapIdentityEntry::text(
                    "geometry.coplanar_overlap.shared_interval",
                    format!(
                        "{index}:{}:{}:{}:{}",
                        row.island_identity(),
                        row.first_segment_identity(),
                        row.second_segment_identity(),
                        row.segment_fact_digest()
                    ),
                )
            }),
    );
    entries.extend(
        basis
            .containment_relations()
            .iter()
            .enumerate()
            .map(|(index, row)| {
                CoplanarOverlapIdentityEntry::text(
                    "geometry.coplanar_overlap.containment",
                    format!(
                        "{index}:{}:{}:{}:{}",
                        row.face_identity(),
                        row.loop_identity(),
                        row.containment(),
                        row.winding_fact_digest()
                    ),
                )
            }),
    );
    entries.extend(
        basis
            .policy_required_exits()
            .iter()
            .enumerate()
            .map(|(index, row)| {
                CoplanarOverlapIdentityEntry::text(
                    "geometry.coplanar_overlap.policy_exit",
                    format!(
                        "{index}:{}:{}:{}",
                        row.region_identity(),
                        row.reason(),
                        row.consumed_fact_digest()
                    ),
                )
            }),
    );
    entries.extend(
        basis
            .ambiguous_contacts()
            .iter()
            .enumerate()
            .map(|(index, row)| {
                CoplanarOverlapIdentityEntry::text(
                    "geometry.coplanar_overlap.ambiguous_contact",
                    format!(
                        "{index}:{}:{}:{}:{}",
                        row.region_identity(),
                        row.first_segment_identity(),
                        row.second_segment_identity(),
                        row.segment_fact_digest()
                    ),
                )
            }),
    );
    entries
}
