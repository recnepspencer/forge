use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::fragment_membership::{
    PlanarBooleanFragmentMembershipMap, PlanarBooleanFragmentMembershipRow,
};
use super::overlap_chain_lineage::{
    PlanarBooleanLoopOverlapChainLineageMap, PlanarBooleanLoopOverlapChainLineageRow,
};
use super::source_loop_carriers::{
    PlanarBooleanLoopSourceCarrierRow, PlanarBooleanLoopSourceCarrierSet,
};

pub(crate) fn source_loop_carrier_identity(
    request_identity: &str,
    split_ledger_receipt_identity: &str,
    recovered_carrier_identity: &str,
    source_loop_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-loop-source-carrier".to_string(),
            format!("request:{request_identity}"),
            format!("split-ledger-receipt:{split_ledger_receipt_identity}"),
            format!("recovered-carrier:{recovered_carrier_identity}"),
            format!("source-loop:{source_loop_identity}"),
        ],
    )
}

pub(crate) fn source_loop_carrier_set_identity(
    request_identity: &str,
    split_ledger_receipt_identity: &str,
    rows: &[PlanarBooleanLoopSourceCarrierRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-loop-source-carrier-set".to_string(),
        format!("request:{request_identity}"),
        format!("split-ledger-receipt:{split_ledger_receipt_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("carrier:{}", row.source_loop_carrier_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn fragment_membership_identity(
    request_identity: &str,
    fragment_identity: &str,
    source_loop_carrier_identity: &str,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-fragment-membership".to_string(),
            format!("request:{request_identity}"),
            format!("fragment:{fragment_identity}"),
            format!("source-loop-carrier:{source_loop_carrier_identity}"),
        ],
    )
}

pub(crate) fn fragment_membership_map_identity(
    request_identity: &str,
    rows: &[PlanarBooleanFragmentMembershipRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-fragment-membership-map".to_string(),
        format!("request:{request_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("membership:{}", row.membership_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn overlap_chain_lineage_identity(
    request_identity: &str,
    chain_identity: &str,
    member_identities: &[String],
    fragment_identities: &[String],
    source_loop_identities: &[String],
    source_edge_identities: &[String],
    boundary_roles: &[crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole],
) -> String {
    let mut parts = vec![
        "planar-boolean-loop-overlap-chain-lineage".to_string(),
        format!("request:{request_identity}"),
        format!("chain:{chain_identity}"),
    ];
    parts.extend(
        member_identities
            .iter()
            .map(|member_identity| format!("member:{member_identity}")),
    );
    parts.extend(
        fragment_identities
            .iter()
            .map(|fragment_identity| format!("fragment:{fragment_identity}")),
    );
    parts.extend(
        source_loop_identities
            .iter()
            .map(|source_loop_identity| format!("source-loop:{source_loop_identity}")),
    );
    parts.extend(
        source_edge_identities
            .iter()
            .map(|source_edge_identity| format!("source-edge:{source_edge_identity}")),
    );
    parts.extend(
        boundary_roles
            .iter()
            .map(|boundary_role| format!("boundary-role:{boundary_role:?}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn overlap_chain_lineage_map_identity(
    request_identity: &str,
    rows: &[PlanarBooleanLoopOverlapChainLineageRow],
) -> String {
    let mut parts = vec![
        "planar-boolean-loop-overlap-chain-lineage-map".to_string(),
        format!("request:{request_identity}"),
    ];
    parts.extend(
        rows.iter()
            .map(|row| format!("lineage:{}", row.lineage_identity())),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

pub(crate) fn provenance_bundle_identity(
    request_identity: &str,
    source_loop_carriers: &PlanarBooleanLoopSourceCarrierSet,
    fragment_membership: &PlanarBooleanFragmentMembershipMap,
    overlap_chain_lineage: &PlanarBooleanLoopOverlapChainLineageMap,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "planar-boolean-loop-source-provenance-bundle".to_string(),
            format!("request:{request_identity}"),
            format!(
                "source-loop-carriers:{}",
                source_loop_carriers.carrier_set_identity()
            ),
            format!(
                "fragment-membership:{}",
                fragment_membership.membership_map_identity()
            ),
            format!(
                "overlap-chain-lineage:{}",
                overlap_chain_lineage.lineage_map_identity()
            ),
        ],
    )
}
