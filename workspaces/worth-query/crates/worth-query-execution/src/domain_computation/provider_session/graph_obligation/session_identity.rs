use worth_foundational::facade::{
    canonicalization, prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue, CanonicalDigestAlgorithmId,
    CanonicalDigestDerivationDenial, CanonicalDigestId, CanonicalDigestWorkBudget,
    CanonicalIntegerWidth, CanonicalizationRuleVersion,
};
use worth_query_installation::facade::WorthQueryCanonicalWorkEvidence;

use super::session_affinity::{
    WorthQueryGraphWorkAccessContextAffinity, WorthQueryGraphWorkSessionAffinity,
};

const SESSION_IDENTITY_BUDGET: CanonicalDigestWorkBudget =
    match CanonicalDigestWorkBudget::new(24, 64 * 1024) {
        Some(budget) => budget,
        None => panic!("fixed graph-work session identity budget is valid"),
    };

pub(super) fn derive_session_identity(
    plan_identity: &CanonicalDigestId,
    affinity: &WorthQueryGraphWorkSessionAffinity,
) -> Result<(CanonicalDigestId, WorthQueryCanonicalWorkEvidence), CanonicalDigestDerivationDenial> {
    let domain = CanonicalBasisDomain::Future("worth-query.graph-work-session");
    let mut entries = Vec::with_capacity(14);
    push_digest(&mut entries, domain, "plan", *plan_identity);
    push_unsigned(
        &mut entries,
        domain,
        "runtime",
        affinity.runtime_authority().as_u64(),
    );
    push_digest(
        &mut entries,
        domain,
        "package",
        *affinity.binding_identity().package_identity(),
    );
    push_digest(
        &mut entries,
        domain,
        "schema",
        *affinity.binding_identity().schema_identity(),
    );
    push_text(
        &mut entries,
        domain,
        "subject-authority",
        affinity.subject_authority_identity(),
    );
    push_entity(
        &mut entries,
        domain,
        "principal",
        affinity.principal_entity_id(),
    );
    match affinity.access_context() {
        WorthQueryGraphWorkAccessContextAffinity::Entity(entity) => {
            push_text(&mut entries, domain, "access-context-kind", "entity");
            push_entity(&mut entries, domain, "scope", *entity);
        }
        WorthQueryGraphWorkAccessContextAffinity::InstalledCapability(identity) => {
            push_text(
                &mut entries,
                domain,
                "access-context-kind",
                "installed-capability",
            );
            push_digest(&mut entries, domain, "access-context-capability", *identity);
        }
    }
    push_text(
        &mut entries,
        domain,
        "branch",
        &affinity.branch().relational_branch().0,
    );
    affinity.basis().encode(&mut entries, domain);
    push_text(
        &mut entries,
        domain,
        "provider",
        affinity.provider_identity(),
    );
    push_unsigned(
        &mut entries,
        domain,
        "managed-run",
        affinity.managed_run_ordinal(),
    );
    let basis = prepare_canonical_basis_sequence(
        CanonicalizationRuleVersion::new("worth-query-graph-work-session-v1")
            .expect("fixed session canonicalization rule is valid"),
        domain,
        entries,
    )
    .into_result()
    .expect("a graph-work session basis is nonempty");
    let ready = canonicalization()
        .digest()
        .for_sequence_with_budget(
            basis,
            CanonicalDigestAlgorithmId::sha256(),
            SESSION_IDENTITY_BUDGET,
        )
        .into_result()?;
    let derived = canonicalization().digest().derive(ready);
    Ok((
        CanonicalDigestId::new(*derived.value().bytes()),
        WorthQueryCanonicalWorkEvidence::one_digest(derived.metadata().work()),
    ))
}

pub(super) fn push_text(
    entries: &mut Vec<CanonicalBasisEntry>,
    domain: CanonicalBasisDomain,
    locus: &'static str,
    value: &str,
) {
    entries.push(entry(
        domain,
        locus,
        CanonicalBasisValue::ExactText(value.to_owned().into()),
    ));
}

pub(super) fn push_unsigned(
    entries: &mut Vec<CanonicalBasisEntry>,
    domain: CanonicalBasisDomain,
    locus: &'static str,
    value: u64,
) {
    entries.push(entry(
        domain,
        locus,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: value.into(),
        },
    ));
}

pub(super) fn push_digest(
    entries: &mut Vec<CanonicalBasisEntry>,
    domain: CanonicalBasisDomain,
    locus: &'static str,
    value: CanonicalDigestId,
) {
    entries.push(entry(
        domain,
        locus,
        CanonicalBasisValue::BytesDigest(value),
    ));
}

fn push_entity(
    entries: &mut Vec<CanonicalBasisEntry>,
    domain: CanonicalBasisDomain,
    locus: &'static str,
    value: worth_relational::facade::identity::EntityId,
) {
    push_unsigned(
        entries,
        domain,
        entity_locus(locus, "partition"),
        value.partition_value_u64(),
    );
    push_unsigned(
        entries,
        domain,
        entity_locus(locus, "slot"),
        value.local_slot_value(),
    );
    push_unsigned(
        entries,
        domain,
        entity_locus(locus, "generation"),
        u64::from(value.generation_value()),
    );
}

fn entity_locus(prefix: &str, component: &str) -> &'static str {
    match (prefix, component) {
        ("principal", "partition") => "principal-partition",
        ("principal", "slot") => "principal-slot",
        ("principal", "generation") => "principal-generation",
        ("scope", "partition") => "scope-partition",
        ("scope", "slot") => "scope-slot",
        ("scope", "generation") => "scope-generation",
        _ => unreachable!("session entity loci are fixed"),
    }
}

fn entry(
    domain: CanonicalBasisDomain,
    locus: &'static str,
    value: CanonicalBasisValue,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain,
        CanonicalBasisLocus::Named(locus.into()),
        CanonicalBasisEntryKind::Field,
        value,
    )
}
