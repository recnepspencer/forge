//! Bounded canonical identity for one exact capability-revocation target.

use worth_foundational::facade::{
    canonical_basis_value_for_aspect_value, canonicalization, prepare_canonical_basis_sequence,
    AspectValue, CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind,
    CanonicalBasisLocus, CanonicalBasisValue, CanonicalDigestAlgorithmId, CanonicalDigestId,
    CanonicalDigestWorkBudget, CanonicalIntegerWidth, CanonicalizationRuleVersion,
};

use crate::canonical_work::WorthQueryCanonicalWorkEvidence;

const DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("worth-query.capability-revocation-proposal");
const RULE_VERSION: &str = "worth-query-capability-revocation-proposal-v1";

pub struct WorthQueryCapabilityRevocationProposalBasis<'a> {
    pub capability: [u8; 32],
    pub resource: (u32, u64, u32),
    pub target_kind: u32,
    pub resource_relation: u32,
    pub target_entity: &'a str,
    pub target_aspect: &'a str,
    pub target_field: &'a str,
    pub target_value_type: &'a str,
    pub target_value: &'a AspectValue,
    pub status_aspect: &'a str,
    pub status_field: &'a str,
    pub active: &'a AspectValue,
    pub revoked: &'a AspectValue,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryCapabilityRevocationProposalDenial;

pub fn derive_capability_revocation_proposal_identity(
    basis: WorthQueryCapabilityRevocationProposalBasis<'_>,
    budget: CanonicalDigestWorkBudget,
) -> Result<([u8; 32], WorthQueryCanonicalWorkEvidence), WorthQueryCapabilityRevocationProposalDenial>
{
    let entries = vec![
        entry(
            "capability",
            CanonicalBasisEntryKind::Identity,
            CanonicalBasisValue::BytesDigest(CanonicalDigestId::new(basis.capability)),
        ),
        entry(
            "command-resource",
            CanonicalBasisEntryKind::Identity,
            CanonicalBasisValue::EntityRef {
                partition_id: basis.resource.0,
                local_slot: basis.resource.1,
                generation: basis.resource.2,
            },
        ),
        integer("target-kind", basis.target_kind),
        integer("resource-relation", basis.resource_relation),
        text("target.entity", basis.target_entity),
        text("target.aspect", basis.target_aspect),
        text("target.field", basis.target_field),
        text("target.value-type", basis.target_value_type),
        entry(
            "target.value",
            CanonicalBasisEntryKind::Value,
            canonical_basis_value_for_aspect_value(basis.target_value),
        ),
        text("status.aspect", basis.status_aspect),
        text("status.field", basis.status_field),
        entry(
            "status.active",
            CanonicalBasisEntryKind::Value,
            canonical_basis_value_for_aspect_value(basis.active),
        ),
        entry(
            "status.revoked",
            CanonicalBasisEntryKind::Value,
            canonical_basis_value_for_aspect_value(basis.revoked),
        ),
    ];
    let version = CanonicalizationRuleVersion::new(RULE_VERSION)
        .ok_or(WorthQueryCapabilityRevocationProposalDenial)?;
    let prepared = prepare_canonical_basis_sequence(version, DOMAIN, entries)
        .into_result()
        .map_err(|_| WorthQueryCapabilityRevocationProposalDenial)?;
    let ready = canonicalization()
        .digest()
        .for_sequence_with_budget(prepared, CanonicalDigestAlgorithmId::sha256(), budget)
        .into_result()
        .map_err(|_| WorthQueryCapabilityRevocationProposalDenial)?;
    let derived = canonicalization().digest().derive(ready);
    Ok((
        *derived.value().bytes(),
        WorthQueryCanonicalWorkEvidence::one_digest(derived.metadata().work()),
    ))
}

fn integer(locus: &str, value: u32) -> CanonicalBasisEntry {
    entry(
        locus,
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits32,
            value: u128::from(value),
        },
    )
}

fn text(locus: &str, value: &str) -> CanonicalBasisEntry {
    entry(
        locus,
        CanonicalBasisEntryKind::Locator,
        CanonicalBasisValue::ExactText(value.to_owned().into()),
    )
}

fn entry(
    locus: &str,
    kind: CanonicalBasisEntryKind,
    value: CanonicalBasisValue,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        DOMAIN,
        CanonicalBasisLocus::Named(locus.to_owned().into()),
        kind,
        value,
    )
}
