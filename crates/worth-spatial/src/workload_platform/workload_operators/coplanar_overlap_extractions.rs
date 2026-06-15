use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::planar_contracts::coplanar_overlap_contract::CoplanarOverlapContractReceipt;

use super::coplanar_overlap::CoplanarOverlapOperatorDenial;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CoplanarOverlapOperatorExtraction {
    fact_digest: String,
    candidate_pair_breadth: usize,
    segment_contacts_certified: usize,
    shared_intervals: usize,
    overlap_islands: usize,
    containment_relations: usize,
    policy_required_exits: usize,
    ambiguous_contacts: usize,
}

impl CoplanarOverlapOperatorExtraction {
    pub(super) fn from_receipt(receipt: &CoplanarOverlapContractReceipt) -> Self {
        let counters = receipt.counters();
        Self {
            fact_digest: receipt.fact_digest().to_string(),
            candidate_pair_breadth: counters.candidate_pair_breadth(),
            segment_contacts_certified: counters.segment_contacts_certified(),
            shared_intervals: counters.shared_intervals(),
            overlap_islands: counters.overlap_islands(),
            containment_relations: counters.containment_relations(),
            policy_required_exits: counters.policy_required_exits(),
            ambiguous_contacts: receipt.ambiguous_contacts().len(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ExtractionSummary {
    pub(super) extraction_identities: Vec<String>,
    pub(super) receipt_count: usize,
    pub(super) candidate_pair_breadth: usize,
    pub(super) segment_contacts_certified: usize,
    pub(super) shared_intervals: usize,
    pub(super) overlap_islands: usize,
    pub(super) containment_relations: usize,
    pub(super) policy_required_exits: usize,
    pub(super) ambiguous_contacts: usize,
}

pub(super) fn extraction_summary(
    extractions: &[CoplanarOverlapOperatorExtraction],
) -> Result<ExtractionSummary, CoplanarOverlapOperatorDenial> {
    if extractions.is_empty() {
        return Err(CoplanarOverlapOperatorDenial::MissingOverlapExtractionReceipts);
    }
    let summary = ExtractionSummary {
        extraction_identities: extractions
            .iter()
            .map(|extraction| extraction.fact_digest.clone())
            .collect(),
        receipt_count: extractions.len(),
        candidate_pair_breadth: extractions
            .iter()
            .map(|extraction| extraction.candidate_pair_breadth)
            .sum(),
        segment_contacts_certified: extractions
            .iter()
            .map(|extraction| extraction.segment_contacts_certified)
            .sum(),
        shared_intervals: extractions
            .iter()
            .map(|extraction| extraction.shared_intervals)
            .sum(),
        overlap_islands: extractions
            .iter()
            .map(|extraction| extraction.overlap_islands)
            .sum(),
        containment_relations: extractions
            .iter()
            .map(|extraction| extraction.containment_relations)
            .sum(),
        policy_required_exits: extractions
            .iter()
            .map(|extraction| extraction.policy_required_exits)
            .sum(),
        ambiguous_contacts: extractions
            .iter()
            .map(|extraction| extraction.ambiguous_contacts)
            .sum(),
    };
    if summary.candidate_pair_breadth == 0
        || summary.segment_contacts_certified + summary.shared_intervals + summary.overlap_islands
            == 0
    {
        return Err(CoplanarOverlapOperatorDenial::SyntheticOverlapExtraction);
    }
    Ok(summary)
}

pub(super) fn operator_digest(
    projection: &str,
    transform: &str,
    retained_replay: &str,
    extraction_identities: &[String],
    summary: &ExtractionSummary,
) -> String {
    let mut parts = vec![
        "coplanar-overlap-operator".to_string(),
        format!("projection:{projection}"),
        format!("transform:{transform}"),
        format!("retained_replay:{retained_replay}"),
        format!("extraction_receipts:{}", summary.receipt_count),
        format!("candidate_pair_breadth:{}", summary.candidate_pair_breadth),
        format!(
            "segment_contacts_certified:{}",
            summary.segment_contacts_certified
        ),
        format!("shared_intervals:{}", summary.shared_intervals),
        format!("overlap_islands:{}", summary.overlap_islands),
        format!("ambiguous_contacts:{}", summary.ambiguous_contacts),
        format!("policy_required_exits:{}", summary.policy_required_exits),
    ];
    parts.extend(
        extraction_identities
            .iter()
            .map(|identity| format!("extraction:{identity}")),
    );
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}
