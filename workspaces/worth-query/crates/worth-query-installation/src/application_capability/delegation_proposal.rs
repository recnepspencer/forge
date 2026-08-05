use crate::canonical_work::WorthQueryCanonicalWorkEvidence;
use worth_foundational::facade::{
    canonical_basis_value_for_aspect_value, canonicalization, prepare_canonical_basis_sequence,
    AspectFieldLocator, CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind,
    CanonicalBasisLocus, CanonicalBasisValue, CanonicalDigestAlgorithmId, CanonicalDigestId,
    CanonicalDigestWorkBudget, CanonicalIntegerWidth, CanonicalizationRuleVersion,
    LocatorAuthority,
};

const DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("worth-query.delegation-activation-proposal");
const RULE_VERSION: &str = "worth-query-delegation-activation-proposal-v1";

pub struct WorthQueryDelegationProposalIdentityBasis<'a> {
    pub target_capability_identity: [u8; 32],
    pub child_kind: u32,
    pub child_key: &'a str,
    pub fields:
        &'a std::collections::BTreeMap<AspectFieldLocator, worth_foundational::facade::AspectValue>,
    pub parent: (u32, (u32, u64, u32)),
    pub grantor: (u32, (u32, u64, u32)),
    pub grantee: (u32, (u32, u64, u32)),
    pub resource: (u32, (u32, u64, u32)),
    pub related: Option<(u32, (u32, u64, u32))>,
    pub activation_context: &'a [(u32, (u32, u64, u32))],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryDelegationProposalIdentityDenial;

pub fn derive_delegation_proposal_identity(
    basis: WorthQueryDelegationProposalIdentityBasis<'_>,
    budget: CanonicalDigestWorkBudget,
) -> Result<([u8; 32], WorthQueryCanonicalWorkEvidence), WorthQueryDelegationProposalIdentityDenial>
{
    let mut entries = Vec::new();
    entries.push(value(
        "target-capability",
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::BytesDigest(CanonicalDigestId::new(basis.target_capability_identity)),
    ));
    entries.push(kind("child.kind", basis.child_kind));
    entries.push(text("child.key", basis.child_key));
    for (index, (locator, aspect_value)) in basis.fields.iter().enumerate() {
        push_field(&mut entries, index, locator, aspect_value);
    }
    push_relation(&mut entries, "parent", basis.parent);
    push_relation(&mut entries, "grantor", basis.grantor);
    push_relation(&mut entries, "grantee", basis.grantee);
    push_relation(&mut entries, "resource", basis.resource);
    entries.push(value(
        "related.present",
        CanonicalBasisEntryKind::Value,
        CanonicalBasisValue::Bool(basis.related.is_some()),
    ));
    if let Some(related) = basis.related {
        push_relation(&mut entries, "related", related);
    }
    for (index, context) in basis.activation_context.iter().copied().enumerate() {
        push_relation(
            &mut entries,
            &format!("activation-context.{index}"),
            context,
        );
    }

    let version = CanonicalizationRuleVersion::new(RULE_VERSION)
        .ok_or(WorthQueryDelegationProposalIdentityDenial)?;
    let prepared = prepare_canonical_basis_sequence(version, DOMAIN, entries)
        .into_result()
        .map_err(|_| WorthQueryDelegationProposalIdentityDenial)?;
    let digest_ready = canonicalization()
        .digest()
        .for_sequence_with_budget(prepared, CanonicalDigestAlgorithmId::sha256(), budget)
        .into_result()
        .map_err(|_| WorthQueryDelegationProposalIdentityDenial)?;
    let derived = canonicalization().digest().derive(digest_ready);
    Ok((
        *derived.value().bytes(),
        WorthQueryCanonicalWorkEvidence::one_digest(derived.metadata().work()),
    ))
}

fn push_field(
    entries: &mut Vec<CanonicalBasisEntry>,
    index: usize,
    locator: &AspectFieldLocator,
    aspect_value: &worth_foundational::facade::AspectValue,
) {
    let prefix = format!("field.{index}");
    entries.push(text(
        format!("{prefix}.authority"),
        locator_authority(locator.aspect().authority()),
    ));
    entries.push(text(
        format!("{prefix}.aspect"),
        locator.aspect().aspect_key().as_str(),
    ));
    entries.push(value(
        format!("{prefix}.path-count"),
        CanonicalBasisEntryKind::Shape,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits32,
            value: u128::try_from(locator.field_path().fields().len())
                .expect("canonical field-path width fits u128"),
        },
    ));
    entries.extend(
        locator
            .field_path()
            .fields()
            .iter()
            .enumerate()
            .map(|(path_index, field)| text(format!("{prefix}.path.{path_index}"), field.as_str())),
    );
    entries.push(value(
        format!("{prefix}.value"),
        CanonicalBasisEntryKind::Value,
        canonical_basis_value_for_aspect_value(aspect_value),
    ));
}

fn push_relation(
    entries: &mut Vec<CanonicalBasisEntry>,
    role: &str,
    (relation, entity): (u32, (u32, u64, u32)),
) {
    entries.push(kind(format!("{role}.relation"), relation));
    entries.push(value(
        format!("{role}.entity"),
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::EntityRef {
            partition_id: entity.0,
            local_slot: entity.1,
            generation: entity.2,
        },
    ));
}

fn kind(locus: impl Into<String>, kind: u32) -> CanonicalBasisEntry {
    value(
        locus,
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits32,
            value: u128::from(kind),
        },
    )
}

fn text(locus: impl Into<String>, text: impl Into<String>) -> CanonicalBasisEntry {
    value(
        locus,
        CanonicalBasisEntryKind::Locator,
        CanonicalBasisValue::ExactText(text.into().into()),
    )
}

fn value(
    locus: impl Into<String>,
    kind: CanonicalBasisEntryKind,
    value: CanonicalBasisValue,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        DOMAIN,
        CanonicalBasisLocus::Named(locus.into().into()),
        kind,
        value,
    )
}

const fn locator_authority(authority: LocatorAuthority) -> &'static str {
    match authority {
        LocatorAuthority::Authoritative => "authoritative",
        LocatorAuthority::Derived => "derived",
        LocatorAuthority::Projected => "projected",
        LocatorAuthority::SupportOnly => "support-only",
        LocatorAuthority::Planned => "planned",
        LocatorAuthority::ReceiptBearing => "receipt-bearing",
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use worth_foundational::facade::{
        AspectFieldLocator, AspectKey, AspectValue, CanonicalDigestWorkBudget, CanonicalFieldPath,
        FieldKey, LocatorAuthority,
    };

    use super::{derive_delegation_proposal_identity, WorthQueryDelegationProposalIdentityBasis};

    #[test]
    fn exact_proposal_is_stable_and_each_changed_governed_axis_changes_identity() {
        let baseline = digest(None);
        assert_eq!(baseline.0, digest(None).0);
        for axis in [
            Axis::Target,
            Axis::ChildKind,
            Axis::ChildKey,
            Axis::FirstField,
            Axis::SecondField,
            Axis::ParentRelation,
            Axis::Parent,
            Axis::Grantor,
            Axis::Grantee,
            Axis::Resource,
            Axis::RelatedPresence,
            Axis::Related,
            Axis::FirstContextRelation,
            Axis::FirstContext,
            Axis::SecondContext,
        ] {
            assert_ne!(baseline.0, digest(Some(axis)).0, "axis {axis:?}");
        }
        assert_eq!(baseline.1.basis_preparations(), 1);
        assert_eq!(baseline.1.digest_derivations(), 1);
        assert_eq!(baseline.1.canonical_entries(), 28);
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Axis {
        Target,
        ChildKind,
        ChildKey,
        FirstField,
        SecondField,
        ParentRelation,
        Parent,
        Grantor,
        Grantee,
        Resource,
        RelatedPresence,
        Related,
        FirstContextRelation,
        FirstContext,
        SecondContext,
    }

    fn digest(
        axis: Option<Axis>,
    ) -> (
        [u8; 32],
        crate::canonical_work::WorthQueryCanonicalWorkEvidence,
    ) {
        let fields = BTreeMap::from([
            (
                field("Capability", "remaining"),
                AspectValue::UInt64(if axis == Some(Axis::FirstField) { 8 } else { 7 }),
            ),
            (
                field("Capability", "status"),
                AspectValue::UInt64(if axis == Some(Axis::SecondField) {
                    2
                } else {
                    1
                }),
            ),
        ]);
        let activation_context = [
            (
                if axis == Some(Axis::FirstContextRelation) {
                    27
                } else {
                    17
                },
                entity(if axis == Some(Axis::FirstContext) {
                    52
                } else {
                    51
                }),
            ),
            (
                18,
                entity(if axis == Some(Axis::SecondContext) {
                    54
                } else {
                    53
                }),
            ),
        ];
        derive_delegation_proposal_identity(
            WorthQueryDelegationProposalIdentityBasis {
                target_capability_identity: [if axis == Some(Axis::Target) { 4 } else { 3 }; 32],
                child_kind: if axis == Some(Axis::ChildKind) { 19 } else { 9 },
                child_key: if axis == Some(Axis::ChildKey) {
                    "child-2"
                } else {
                    "child-1"
                },
                fields: &fields,
                parent: (
                    if axis == Some(Axis::ParentRelation) {
                        21
                    } else {
                        11
                    },
                    entity(if axis == Some(Axis::Parent) { 61 } else { 41 }),
                ),
                grantor: (
                    12,
                    entity(if axis == Some(Axis::Grantor) { 62 } else { 42 }),
                ),
                grantee: (
                    13,
                    entity(if axis == Some(Axis::Grantee) { 63 } else { 43 }),
                ),
                resource: (
                    14,
                    entity(if axis == Some(Axis::Resource) { 64 } else { 44 }),
                ),
                related: if axis == Some(Axis::RelatedPresence) {
                    None
                } else {
                    Some((
                        15,
                        entity(if axis == Some(Axis::Related) { 65 } else { 45 }),
                    ))
                },
                activation_context: &activation_context,
            },
            CanonicalDigestWorkBudget::new(64, 256 * 1_024).expect("bounded test digest"),
        )
        .expect("canonical proposal")
    }

    fn field(aspect: &str, field: &str) -> AspectFieldLocator {
        AspectFieldLocator::new(
            LocatorAuthority::Authoritative,
            AspectKey::new(aspect).expect("test aspect"),
            CanonicalFieldPath::single(FieldKey::new(field).expect("test field")),
        )
    }

    fn entity(slot: u64) -> (u32, u64, u32) {
        (0, slot, 1)
    }
}
