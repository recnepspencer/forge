use worth_foundational::facade::{
    canonicalization, prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact, CanonicalBasisValue,
    CanonicalDigestAlgorithmId, CanonicalDigestDerivationDenial, CanonicalDigestId,
    CanonicalDigestWorkBudget, CanonicalIntegerWidth, CanonicalizationRuleVersion,
};
use worth_query_declaration::facade::application_capability::{
    ApplicationCapabilityFieldBinding, ApplicationCapabilityFieldDimension,
    ApplicationCapabilityRelationBinding, ApplicationCapabilityRelationDimension,
    ApplicationCapabilityRule, ApplicationCapabilityValueBinding,
    ErasedApplicationCapabilityContract,
};

use super::{
    WorthQueryApplicationCapabilityInstallationDenial,
    WorthQueryApplicationCapabilityInstallationDenialKind,
};
use crate::canonical_work::WorthQueryCanonicalWorkEvidence;

const DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("worth-query.application-capability-installation");
const RULE_VERSION: &str = "worth-query-application-capability-installation-v1";
const MAXIMUM_ENTRY_COUNT: u32 = 128;
const MAXIMUM_CANONICAL_BYTES: usize = 16 * 1_024;
const CAPABILITY_BUDGET: CanonicalDigestWorkBudget =
    match CanonicalDigestWorkBudget::new(MAXIMUM_ENTRY_COUNT, MAXIMUM_CANONICAL_BYTES) {
        Some(budget) => budget,
        None => panic!("fixed capability canonical-work budget is valid"),
    };

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryCapabilityCanonicalArtifact {
    basis: CanonicalBasisReadyArtifact,
    digest: CanonicalDigestId,
    work: WorthQueryCanonicalWorkEvidence,
}

impl WorthQueryCapabilityCanonicalArtifact {
    pub fn basis(&self) -> &CanonicalBasisReadyArtifact {
        &self.basis
    }

    pub const fn digest(&self) -> &CanonicalDigestId {
        &self.digest
    }

    pub const fn canonical_encoded_bytes(&self) -> usize {
        self.work.canonical_encoded_bytes()
    }

    pub const fn basis_preparation_count(&self) -> usize {
        self.work.basis_preparations() as usize
    }

    pub const fn digest_derivation_count(&self) -> usize {
        self.work.digest_derivations() as usize
    }

    pub const fn work(&self) -> WorthQueryCanonicalWorkEvidence {
        self.work
    }
}

pub(super) fn prepare_capability_basis(
    package_identity: &CanonicalDigestId,
    schema_identity: &CanonicalDigestId,
    contract: &ErasedApplicationCapabilityContract,
) -> Result<WorthQueryCapabilityCanonicalArtifact, WorthQueryApplicationCapabilityInstallationDenial>
{
    let mut builder = CapabilityBasisBuilder::new(contract.name());
    builder.text("family", "installed-capability");
    builder.digest("package", package_identity);
    builder.digest("schema", schema_identity);
    builder.text("name", contract.name());
    builder.text("operation", contract.operation());
    builder.text("input-type", contract.input_type());
    builder.text("grant-entity", contract.grant_entity());
    append_contract(&mut builder, contract);
    builder.finish()
}

fn append_contract(
    builder: &mut CapabilityBasisBuilder,
    contract: &ErasedApplicationCapabilityContract,
) {
    let target = contract.target();
    append_value_binding(builder, "target.action", target.action());
    append_relation(builder, "target.resource", target.resource());
    append_relation_dimension(builder, "target.relation", target.relation());
    append_field_dimension(builder, "target.field", target.field());
    append_value_binding(builder, "target.purpose", target.purpose());

    let constraints = contract.constraints();
    append_field_dimension(builder, "constraints.amount", constraints.amount());
    match constraints.cardinality() {
        worth_query_declaration::facade::application_capability::ApplicationCapabilityCardinalityDimension::One => {
            builder.text("constraints.cardinality", "one");
        }
        worth_query_declaration::facade::application_capability::ApplicationCapabilityCardinalityDimension::Many => {
            builder.text("constraints.cardinality", "many");
        }
        worth_query_declaration::facade::application_capability::ApplicationCapabilityCardinalityDimension::Bounded(limit) => {
            builder.text("constraints.cardinality", "bounded");
            builder.u32("constraints.cardinality-limit", limit);
        }
    }
    append_field(
        builder,
        "constraints.workflow-stage",
        constraints.workflow_stage(),
    );
    append_field(
        builder,
        "constraints.validity.not-before",
        constraints.validity().not_before(),
    );
    append_field(
        builder,
        "constraints.validity.not-after",
        constraints.validity().not_after(),
    );
    builder.text("constraints.context", constraints.context());

    let delegation = contract.delegation();
    append_relation(builder, "delegation.parent", delegation.parent());
    append_relation(builder, "delegation.grantor", delegation.grantor());
    append_relation(builder, "delegation.grantee", delegation.grantee());
    append_field(builder, "delegation.limit", delegation.limit());
    builder.text("delegation.provenance", delegation.provenance());
    append_composition(builder, contract);
}

fn append_composition(
    builder: &mut CapabilityBasisBuilder,
    contract: &ErasedApplicationCapabilityContract,
) {
    let composition = contract.composition();
    for (locus, rule) in [
        ("composition.allow", composition.decision().allow()),
        ("composition.deny", composition.decision().deny()),
        ("composition.conflict", composition.decision().conflict()),
        (
            "composition.separation-of-duty",
            composition.actors().separation_of_duty(),
        ),
        (
            "composition.distinct-actor",
            composition.actors().distinct_actor(),
        ),
        (
            "composition.delegation",
            composition.propagation().delegation(),
        ),
        (
            "composition.disclosure",
            composition.propagation().disclosure(),
        ),
    ] {
        append_rule(builder, locus, rule);
    }
}

fn append_field(
    builder: &mut CapabilityBasisBuilder,
    prefix: &str,
    field: &ApplicationCapabilityFieldBinding,
) {
    builder.text(format!("{prefix}.entity"), field.entity());
    builder.text(format!("{prefix}.aspect"), field.aspect());
    builder.text(format!("{prefix}.field"), field.field());
    builder.text(format!("{prefix}.value-type"), field.value_type());
}

fn append_value_binding(
    builder: &mut CapabilityBasisBuilder,
    prefix: &str,
    binding: &ApplicationCapabilityValueBinding,
) {
    append_field(builder, &format!("{prefix}.field-binding"), binding.field());
    builder.aspect_value(format!("{prefix}.value"), binding.value());
}

fn append_relation(
    builder: &mut CapabilityBasisBuilder,
    prefix: &str,
    relation: &ApplicationCapabilityRelationBinding,
) {
    builder.text(format!("{prefix}.relation"), relation.relation());
    builder.text(format!("{prefix}.from"), relation.from());
    builder.text(format!("{prefix}.to"), relation.to());
}

fn append_field_dimension(
    builder: &mut CapabilityBasisBuilder,
    prefix: &str,
    dimension: &ApplicationCapabilityFieldDimension,
) {
    match dimension {
        ApplicationCapabilityFieldDimension::NotApplicable => {
            builder.text(format!("{prefix}.posture"), "not-applicable");
        }
        ApplicationCapabilityFieldDimension::Bound(field) => {
            builder.text(format!("{prefix}.posture"), "bound");
            append_field(builder, prefix, field);
        }
    }
}

fn append_relation_dimension(
    builder: &mut CapabilityBasisBuilder,
    prefix: &str,
    dimension: &ApplicationCapabilityRelationDimension,
) {
    match dimension {
        ApplicationCapabilityRelationDimension::NotApplicable => {
            builder.text(format!("{prefix}.posture"), "not-applicable");
        }
        ApplicationCapabilityRelationDimension::Bound(relation) => {
            builder.text(format!("{prefix}.posture"), "bound");
            append_relation(builder, prefix, relation);
        }
    }
}

fn append_rule(
    builder: &mut CapabilityBasisBuilder,
    prefix: &str,
    rule: &ApplicationCapabilityRule,
) {
    match rule {
        ApplicationCapabilityRule::NotApplicable => {
            builder.text(format!("{prefix}.posture"), "not-applicable");
        }
        ApplicationCapabilityRule::Policy(policy) => {
            builder.text(format!("{prefix}.posture"), "policy");
            builder.text(format!("{prefix}.policy"), policy);
        }
    }
}

struct CapabilityBasisBuilder {
    subject: String,
    entries: Vec<CanonicalBasisEntry>,
}

impl CapabilityBasisBuilder {
    fn new(subject: &str) -> Self {
        Self {
            subject: subject.to_string(),
            entries: Vec::with_capacity(80),
        }
    }

    fn text(&mut self, locus: impl Into<String>, value: impl AsRef<str>) {
        let locus = locus.into();
        let value = value.as_ref();
        self.entries.push(entry(
            locus,
            CanonicalBasisValue::ExactText(value.to_owned().into()),
        ));
    }

    fn digest(&mut self, locus: impl Into<String>, value: &CanonicalDigestId) {
        self.entries
            .push(entry(locus, CanonicalBasisValue::BytesDigest(*value)));
    }

    fn u32(&mut self, locus: impl Into<String>, value: u32) {
        let locus = locus.into();
        self.entries.push(entry(
            locus,
            CanonicalBasisValue::UnsignedInteger {
                width: CanonicalIntegerWidth::Bits32,
                value: value.into(),
            },
        ));
    }

    fn aspect_value(
        &mut self,
        locus: impl Into<String>,
        value: &worth_foundational::facade::AspectValue,
    ) {
        let locus = locus.into();
        let canonical = worth_foundational::facade::canonical_basis_value_for_aspect_value(value);
        self.entries.push(entry(locus, canonical));
    }

    fn finish(
        self,
    ) -> Result<
        WorthQueryCapabilityCanonicalArtifact,
        WorthQueryApplicationCapabilityInstallationDenial,
    > {
        let version = CanonicalizationRuleVersion::new(RULE_VERSION)
            .expect("the installed capability rule is valid");
        let basis = prepare_canonical_basis_sequence(version, DOMAIN, self.entries)
            .into_result()
            .expect("installed capability basis loci are unique and typed");
        let ready = canonicalization()
            .digest()
            .for_sequence_with_budget(
                basis.clone(),
                CanonicalDigestAlgorithmId::sha256(),
                CAPABILITY_BUDGET,
            )
            .into_result()
            .map_err(|denial| canonical_denial(&self.subject, denial))?;
        let derived = canonicalization().digest().derive(ready);
        Ok(WorthQueryCapabilityCanonicalArtifact {
            basis,
            digest: CanonicalDigestId::new(*derived.value().bytes()),
            work: WorthQueryCanonicalWorkEvidence::one_digest(derived.metadata().work()),
        })
    }
}

fn entry(locus: impl Into<String>, value: CanonicalBasisValue) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        DOMAIN,
        CanonicalBasisLocus::Named(locus.into().into()),
        CanonicalBasisEntryKind::Identity,
        value,
    )
}

fn canonical_denial(
    subject: &str,
    denial: CanonicalDigestDerivationDenial,
) -> WorthQueryApplicationCapabilityInstallationDenial {
    let kind = match denial {
        CanonicalDigestDerivationDenial::EntryLimitExceeded { .. } => {
            WorthQueryApplicationCapabilityInstallationDenialKind::CanonicalEntryLimitExceeded
        }
        CanonicalDigestDerivationDenial::EncodedByteLimitExceeded { .. } => {
            WorthQueryApplicationCapabilityInstallationDenialKind::CanonicalByteLimitExceeded
        }
        _ => WorthQueryApplicationCapabilityInstallationDenialKind::CanonicalDigestSlotRejected,
    };
    super::denial::WorthQueryApplicationCapabilityInstallationDenial::new(kind, subject)
}
