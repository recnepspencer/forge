use std::collections::HashSet;
use std::path::Path;

use super::closeout_report::WorthGraphAuthorityCloseoutViolation;
use super::closeout_types::{
    WorthGraphAuthorityCloseoutMatrixRow, WorthGraphAuthorityPublicFacadeEvidence,
    WorthGraphAuthorityPublicFacadeProof,
};

pub(crate) fn validate_public_facade_evidence(
    matrix: &[WorthGraphAuthorityCloseoutMatrixRow],
) -> Result<(), WorthGraphAuthorityCloseoutViolation> {
    let mut proofs = HashSet::new();
    for row in matrix {
        let evidence = row.public_facade_evidence();
        validate_row_facade(row.source_id(), evidence)?;
        validate_contract_test_path(row.source_id(), evidence)?;
        proofs.insert(evidence.proof());
    }
    for proof in WorthGraphAuthorityPublicFacadeProof::ALL {
        if !proofs.contains(&proof) {
            return Err(WorthGraphAuthorityCloseoutViolation::PublicFacadeProofMissing(proof));
        }
    }
    Ok(())
}

fn validate_row_facade(
    source_id: &'static str,
    evidence: WorthGraphAuthorityPublicFacadeEvidence,
) -> Result<(), WorthGraphAuthorityCloseoutViolation> {
    let ordinary_api = evidence.ordinary_api();
    if ordinary_api.is_empty() {
        return Err(WorthGraphAuthorityCloseoutViolation::PublicFacadeMissing(
            source_id,
        ));
    }
    if ordinary_api.contains("certify_worth_graph_authority") {
        return Err(WorthGraphAuthorityCloseoutViolation::RawCertifierExposedAsFacade(source_id));
    }
    let expected_prefix = public_facade_api_prefix(evidence.proof());
    if !ordinary_api.starts_with(expected_prefix) {
        return Err(
            WorthGraphAuthorityCloseoutViolation::PublicFacadeRootMismatch {
                source_id,
                ordinary_api,
                expected_prefix,
            },
        );
    }
    let expected_api = expected_public_facade_api(evidence.proof());
    if ordinary_api != expected_api {
        return Err(
            WorthGraphAuthorityCloseoutViolation::PublicFacadeApiMismatch {
                source_id,
                ordinary_api,
                expected_api,
            },
        );
    }
    let expected_accessor = expected_public_facade_posture_accessor(evidence.proof());
    let posture_accessor = evidence.posture_accessor();
    if posture_accessor != expected_accessor {
        return Err(
            WorthGraphAuthorityCloseoutViolation::PublicFacadePostureMismatch {
                source_id,
                posture_accessor,
                expected_accessor,
            },
        );
    }
    Ok(())
}

fn public_facade_api_prefix(proof: WorthGraphAuthorityPublicFacadeProof) -> &'static str {
    match proof {
        WorthGraphAuthorityPublicFacadeProof::TopologyOperatorQuerySurface => {
            "worth_topo::facade::"
        }
        WorthGraphAuthorityPublicFacadeProof::SpatialEvidenceLedgerSurface => {
            "worth_spatial::facade::"
        }
        WorthGraphAuthorityPublicFacadeProof::KernelCloseoutReportSurface => {
            "worth_kernel::query_graph_authority_gate::"
        }
        WorthGraphAuthorityPublicFacadeProof::ForgeQueryConsumerKitSurface => "forge_query::",
    }
}

fn expected_public_facade_api(proof: WorthGraphAuthorityPublicFacadeProof) -> &'static str {
    match proof {
        WorthGraphAuthorityPublicFacadeProof::TopologyOperatorQuerySurface => {
            "worth_topo::facade::topology_operator_graph_obligation_catalog"
        }
        WorthGraphAuthorityPublicFacadeProof::SpatialEvidenceLedgerSurface => {
            "worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStageIndexProduct"
        }
        WorthGraphAuthorityPublicFacadeProof::KernelCloseoutReportSurface => {
            "worth_kernel::query_graph_authority_gate::current_worth_graph_authority_closeout_report"
        }
        WorthGraphAuthorityPublicFacadeProof::ForgeQueryConsumerKitSurface => {
            "forge_query::graph_obligation_consumer_kit"
        }
    }
}

fn expected_public_facade_posture_accessor(
    proof: WorthGraphAuthorityPublicFacadeProof,
) -> &'static str {
    match proof {
        WorthGraphAuthorityPublicFacadeProof::TopologyOperatorQuerySurface => {
            "TopologyOperatorGraphObligationCatalog::rows"
        }
        WorthGraphAuthorityPublicFacadeProof::SpatialEvidenceLedgerSurface => {
            "WorkloadEvidenceStageIndexProduct::counters"
        }
        WorthGraphAuthorityPublicFacadeProof::KernelCloseoutReportSurface => {
            "WorthGraphAuthorityCloseoutReport::counters"
        }
        WorthGraphAuthorityPublicFacadeProof::ForgeQueryConsumerKitSurface => {
            "ForgeQueryGraphObligationAdoptionProof::manifest"
        }
    }
}

fn validate_contract_test_path(
    source_id: &'static str,
    evidence: WorthGraphAuthorityPublicFacadeEvidence,
) -> Result<(), WorthGraphAuthorityCloseoutViolation> {
    let contract_test_path = evidence.contract_test_path();
    let path = workspace_root().join(contract_test_path);
    if !path.is_file() {
        return Err(WorthGraphAuthorityCloseoutViolation::PublicFacadeContractMissing(source_id));
    }
    let contract_source = std::fs::read_to_string(&path).map_err(|_| {
        WorthGraphAuthorityCloseoutViolation::PublicFacadeContractMissing(source_id)
    })?;
    for symbol in facade_contract_symbols(evidence) {
        if !contract_source.contains(&symbol) {
            return Err(
                WorthGraphAuthorityCloseoutViolation::PublicFacadeContractSymbolMissing {
                    source_id,
                    symbol,
                },
            );
        }
    }
    Ok(())
}

fn facade_contract_symbols(evidence: WorthGraphAuthorityPublicFacadeEvidence) -> Vec<String> {
    let mut symbols = Vec::new();
    if let Some(symbol) = evidence.ordinary_api().rsplit("::").next() {
        symbols.push(symbol.to_string());
    }
    for symbol in evidence.posture_accessor().split("::") {
        if !symbol.is_empty() {
            symbols.push(symbol.to_string());
        }
    }
    symbols.extend(
        evidence
            .contract_symbols()
            .iter()
            .map(|symbol| symbol.to_string()),
    );
    symbols.sort();
    symbols.dedup();
    symbols
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("worth-kernel should live two levels below the workspace root")
}
