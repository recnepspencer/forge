use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanSplitPersistentNameRow, PlanarBooleanSplitSubshapeSignatureRow,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanAdmittedReconstructedLoop, PlanarBooleanBornLoop,
    PlanarBooleanDegenerateLoopOutcome, PlanarBooleanDegenerateLoopOutcomeKind,
    PlanarBooleanLoopClassifiedProductKind, PlanarBooleanLoopRoleOutcome,
};

use super::counters::PlanarBooleanLoopIdentityMintingCounters;
use super::denial::{
    PlanarBooleanLoopIdentityMintingDenial, PlanarBooleanLoopIdentityMintingDenialKind,
};
use super::input::PlanarBooleanLoopIdentityMintingInput;

pub(crate) struct IndexedLoopIdentityInputRow<'a> {
    tracked_loop_identity: &'a str,
    loop_kind: PlanarBooleanLoopClassifiedProductKind,
    source_loop_identities: Vec<String>,
    fragment_identities: Vec<String>,
    split_vertex_identities: Vec<String>,
    chain_identities: Vec<String>,
    role_outcome: &'a PlanarBooleanLoopRoleOutcome,
    degenerate_outcome: &'a PlanarBooleanDegenerateLoopOutcome,
}

pub(crate) struct LoopIdentitySupportIndex<'a> {
    admitted_rows: Vec<IndexedLoopIdentityInputRow<'a>>,
    name_rows_by_artifact: BTreeMap<String, Vec<&'a PlanarBooleanSplitPersistentNameRow>>,
    signature_rows_by_artifact: BTreeMap<String, &'a PlanarBooleanSplitSubshapeSignatureRow>,
}

impl<'a> LoopIdentitySupportIndex<'a> {
    pub(crate) fn admit(
        input: PlanarBooleanLoopIdentityMintingInput<'a>,
        counters: &mut PlanarBooleanLoopIdentityMintingCounters,
    ) -> Result<Self, PlanarBooleanLoopIdentityMintingDenial> {
        let request_identity = input.reconstructed_loops().request_identity();
        if input.born_loops().request_identity() != request_identity
            || input.role_outcomes().request_identity() != request_identity
            || input.degenerate_outcomes().request_identity() != request_identity
            || input.denied_loop_candidates().request_identity() != request_identity
            || input.split_attribution().request_identity() != request_identity
            || input.naming_support().request_identity() != request_identity
        {
            return Err(PlanarBooleanLoopIdentityMintingDenial::new(
                PlanarBooleanLoopIdentityMintingDenialKind::RequestIdentityMismatch,
                request_identity.to_string(),
                *counters,
                "loop identity minting requires every phase-twelve input artifact and naming support artifact to share the same request identity",
            ));
        }

        counters.indexed_denied_candidates(input.denied_loop_candidates().rows().len());
        counters.indexed_split_name_rows(input.naming_support().persistent_name_rows().len());

        let role_outcomes_by_loop = input
            .role_outcomes()
            .rows()
            .iter()
            .map(|row| (row.loop_identity().to_string(), row))
            .collect::<BTreeMap<_, _>>();
        let degenerate_outcomes_by_loop = input
            .degenerate_outcomes()
            .rows()
            .iter()
            .map(|row| (row.loop_identity().to_string(), row))
            .collect::<BTreeMap<_, _>>();
        let mut admitted_rows = Vec::new();

        for reconstructed_loop in input.reconstructed_loops().rows() {
            if let Some(admitted) = admitted_loop_row_for_reconstructed(
                reconstructed_loop,
                &role_outcomes_by_loop,
                &degenerate_outcomes_by_loop,
                counters,
            )? {
                admitted_rows.push(admitted);
            }
        }
        for born_loop in input.born_loops().rows() {
            if let Some(admitted) = admitted_loop_row_for_born(
                born_loop,
                &role_outcomes_by_loop,
                &degenerate_outcomes_by_loop,
                counters,
            )? {
                admitted_rows.push(admitted);
            }
        }
        admitted_rows.sort_by(|left, right| {
            left.loop_kind().cmp(&right.loop_kind()).then_with(|| {
                left.tracked_loop_identity()
                    .cmp(right.tracked_loop_identity())
            })
        });

        let name_rows_by_artifact = input.naming_support().persistent_name_rows().iter().fold(
            BTreeMap::<String, Vec<&PlanarBooleanSplitPersistentNameRow>>::new(),
            |mut acc, row| {
                acc.entry(row.artifact_identity().to_string())
                    .or_default()
                    .push(row);
                acc
            },
        );
        let signature_rows_by_artifact = input
            .naming_support()
            .subshape_signature_rows()
            .iter()
            .map(|row| (row.artifact_identity().to_string(), row))
            .collect();

        Ok(Self {
            admitted_rows,
            name_rows_by_artifact,
            signature_rows_by_artifact,
        })
    }

    pub(crate) fn admitted_rows(&self) -> &[IndexedLoopIdentityInputRow<'a>] {
        &self.admitted_rows
    }

    pub(crate) fn name_rows_for_artifact(
        &self,
        artifact_identity: &str,
    ) -> Option<&[&'a PlanarBooleanSplitPersistentNameRow]> {
        self.name_rows_by_artifact
            .get(artifact_identity)
            .map(Vec::as_slice)
    }

    pub(crate) fn signature_row_for_artifact(
        &self,
        artifact_identity: &str,
    ) -> Option<&'a PlanarBooleanSplitSubshapeSignatureRow> {
        self.signature_rows_by_artifact
            .get(artifact_identity)
            .copied()
    }
}

impl<'a> IndexedLoopIdentityInputRow<'a> {
    pub(crate) fn tracked_loop_identity(&self) -> &str {
        self.tracked_loop_identity
    }

    pub(crate) fn loop_kind(&self) -> PlanarBooleanLoopClassifiedProductKind {
        self.loop_kind
    }

    pub(crate) fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }

    pub(crate) fn fragment_identities(&self) -> &[String] {
        &self.fragment_identities
    }

    pub(crate) fn split_vertex_identities(&self) -> &[String] {
        &self.split_vertex_identities
    }

    pub(crate) fn role_outcome(&self) -> &PlanarBooleanLoopRoleOutcome {
        self.role_outcome
    }

    pub(crate) fn degenerate_outcome(&self) -> &PlanarBooleanDegenerateLoopOutcome {
        self.degenerate_outcome
    }

    pub(crate) fn seed_artifact_identities(&self) -> Vec<String> {
        let mut ids = self.fragment_identities.clone();
        ids.extend(self.split_vertex_identities.clone());
        ids.extend(self.chain_identities.clone());
        ids.sort();
        ids.dedup();
        ids
    }
}

fn admitted_loop_row_for_reconstructed<'a>(
    reconstructed_loop: &'a PlanarBooleanAdmittedReconstructedLoop,
    role_outcomes_by_loop: &BTreeMap<String, &'a PlanarBooleanLoopRoleOutcome>,
    degenerate_outcomes_by_loop: &BTreeMap<String, &'a PlanarBooleanDegenerateLoopOutcome>,
    counters: &mut PlanarBooleanLoopIdentityMintingCounters,
) -> Result<Option<IndexedLoopIdentityInputRow<'a>>, PlanarBooleanLoopIdentityMintingDenial> {
    counters.considered_admitted_loop();
    let tracked_loop_identity = reconstructed_loop.reconstructed_loop_identity();
    let Some(role_outcome) = role_outcomes_by_loop.get(tracked_loop_identity).copied() else {
        return Err(PlanarBooleanLoopIdentityMintingDenial::new(
            PlanarBooleanLoopIdentityMintingDenialKind::MissingRoleOutcome,
            tracked_loop_identity.to_string(),
            *counters,
            "loop identity minting requires a role outcome for every admitted reconstructed loop",
        ));
    };
    let Some(degenerate_outcome) = degenerate_outcomes_by_loop
        .get(tracked_loop_identity)
        .copied()
    else {
        return Err(PlanarBooleanLoopIdentityMintingDenial::new(
            PlanarBooleanLoopIdentityMintingDenialKind::MissingDegenerateOutcome,
            tracked_loop_identity.to_string(),
            *counters,
            "loop identity minting requires a degenerate outcome row for every admitted reconstructed loop",
        ));
    };
    if degenerate_outcome.kind()
        != PlanarBooleanDegenerateLoopOutcomeKind::AdmittedForIdentityMinting
    {
        return Ok(None);
    }
    Ok(Some(IndexedLoopIdentityInputRow {
        tracked_loop_identity,
        loop_kind: PlanarBooleanLoopClassifiedProductKind::ReconstructedLoop,
        source_loop_identities: vec![reconstructed_loop.source_loop_identity().to_string()],
        fragment_identities: reconstructed_loop.fragment_identities().to_vec(),
        split_vertex_identities: reconstructed_loop.split_vertex_identities().to_vec(),
        chain_identities: Vec::new(),
        role_outcome,
        degenerate_outcome,
    }))
}

fn admitted_loop_row_for_born<'a>(
    born_loop: &'a PlanarBooleanBornLoop,
    role_outcomes_by_loop: &BTreeMap<String, &'a PlanarBooleanLoopRoleOutcome>,
    degenerate_outcomes_by_loop: &BTreeMap<String, &'a PlanarBooleanDegenerateLoopOutcome>,
    counters: &mut PlanarBooleanLoopIdentityMintingCounters,
) -> Result<Option<IndexedLoopIdentityInputRow<'a>>, PlanarBooleanLoopIdentityMintingDenial> {
    counters.considered_admitted_loop();
    let tracked_loop_identity = born_loop.born_loop_identity();
    let Some(role_outcome) = role_outcomes_by_loop.get(tracked_loop_identity).copied() else {
        return Err(PlanarBooleanLoopIdentityMintingDenial::new(
            PlanarBooleanLoopIdentityMintingDenialKind::MissingRoleOutcome,
            tracked_loop_identity.to_string(),
            *counters,
            "loop identity minting requires a role outcome for every born loop under identity consideration",
        ));
    };
    let Some(degenerate_outcome) = degenerate_outcomes_by_loop
        .get(tracked_loop_identity)
        .copied()
    else {
        return Err(PlanarBooleanLoopIdentityMintingDenial::new(
            PlanarBooleanLoopIdentityMintingDenialKind::MissingDegenerateOutcome,
            tracked_loop_identity.to_string(),
            *counters,
            "loop identity minting requires a degenerate outcome row for every born loop under identity consideration",
        ));
    };
    if degenerate_outcome.kind()
        != PlanarBooleanDegenerateLoopOutcomeKind::AdmittedForIdentityMinting
    {
        return Ok(None);
    }
    Ok(Some(IndexedLoopIdentityInputRow {
        tracked_loop_identity,
        loop_kind: PlanarBooleanLoopClassifiedProductKind::BornLoop,
        source_loop_identities: born_loop.source_loop_identities().to_vec(),
        fragment_identities: born_loop.fragment_identities().to_vec(),
        split_vertex_identities: born_loop.split_vertex_identities().to_vec(),
        chain_identities: born_loop.contributing_chain_identities().to_vec(),
        role_outcome,
        degenerate_outcome,
    }))
}
