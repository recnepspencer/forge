use worth_foundational::facade::{
    canonicalization, prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue, CanonicalDigestAlgorithmId,
    CanonicalDigestId, CanonicalDigestWorkBudget, CanonicalIntegerWidth,
    CanonicalizationRuleVersion,
};
use worth_query_installation::facade::{
    ApplicationSchemaBindingIdentity, WorthQueryCanonicalWorkEvidence,
};
use worth_relational::facade::identity::EntityId;

use crate::domain_computation::execution_runtime::WorthQueryRuntimeAuthorityIdentity;

const DOMAIN: CanonicalBasisDomain = CanonicalBasisDomain::Future("worth-query.operation-scope");
const RULE_VERSION: &str = "worth-query-operation-scope-v4";
const SCOPE_BUDGET: CanonicalDigestWorkBudget = match CanonicalDigestWorkBudget::new(6, 64 * 1_024)
{
    Some(budget) => budget,
    None => panic!("fixed operation scope canonical-work budget is valid"),
};

pub(super) struct PreparedOperationScopeIdentity {
    pub(super) digest: CanonicalDigestId,
    pub(super) work: WorthQueryCanonicalWorkEvidence,
}

pub(super) fn derive_operation_scope_identity(
    runtime: WorthQueryRuntimeAuthorityIdentity,
    binding: &ApplicationSchemaBindingIdentity,
    operation_authority_identity: &str,
    principal: EntityId,
    scope: EntityId,
) -> Result<PreparedOperationScopeIdentity, ()> {
    let version = CanonicalizationRuleVersion::new(RULE_VERSION)
        .expect("the fixed operation-scope rule is valid");
    let basis = prepare_canonical_basis_sequence(
        version,
        DOMAIN,
        [
            unsigned("runtime-authority", runtime.as_u64()),
            digest("package", binding.package_identity()),
            digest("schema", binding.schema_identity()),
            text(
                "installed-operation-authority",
                operation_authority_identity,
            ),
            entity("principal", principal),
            entity("scope", scope),
        ],
    )
    .into_result()
    .expect("the typed operation-scope basis is valid");
    let ready = canonicalization()
        .digest()
        .for_sequence_with_budget(basis, CanonicalDigestAlgorithmId::sha256(), SCOPE_BUDGET)
        .into_result()
        .map_err(|_| ())?;
    let derived = canonicalization().digest().derive(ready);
    Ok(PreparedOperationScopeIdentity {
        digest: CanonicalDigestId::new(*derived.value().bytes()),
        work: WorthQueryCanonicalWorkEvidence::one_digest(derived.metadata().work()),
    })
}

fn text(locus: &'static str, value: &str) -> CanonicalBasisEntry {
    entry(
        locus,
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::ExactText(value.to_owned().into()),
    )
}

fn unsigned(locus: &'static str, value: u64) -> CanonicalBasisEntry {
    entry(
        locus,
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: value.into(),
        },
    )
}

fn entity(locus: &'static str, value: EntityId) -> CanonicalBasisEntry {
    entry(
        locus,
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::EntityRef {
            partition_id: value.partition_id.0,
            local_slot: value.local_slot.0,
            generation: value.generation.0,
        },
    )
}

fn digest(
    locus: &'static str,
    value: &worth_foundational::facade::CanonicalDigestId,
) -> CanonicalBasisEntry {
    entry(
        locus,
        CanonicalBasisEntryKind::Identity,
        CanonicalBasisValue::BytesDigest(*value),
    )
}

fn entry(
    locus: &'static str,
    kind: CanonicalBasisEntryKind,
    value: CanonicalBasisValue,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        DOMAIN,
        CanonicalBasisLocus::Named(locus.into()),
        kind,
        value,
    )
}
