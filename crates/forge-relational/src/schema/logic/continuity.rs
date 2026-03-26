use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::sync::Arc;

use crate::schema::data::{
    CompatibilityObservation, DescriptorCanonicalizationVersion, DescriptorSemanticsVersion,
    FreeFormSchemaDiffIntent, HistoricalInterpretationSensitivity, LoweredSchemaTransitionPlan,
    ProposedSchemaTransition, SchemaBoundaryFingerprint, SchemaBridgeDescriptor,
    SchemaBridgeabilityClassification, SchemaContinuationClassification,
    SchemaContinuationDescriptor, SchemaDiffAtom, SchemaDiffDetail, SchemaLineageArtifact,
    SchemaLineageOrderingSemantics, SchemaReconciliationClassification,
    SchemaReconciliationDescriptor, SchemaReconciliationOrderingMode, SchemaReconciliationPolicy,
    SchemaStratum, SchemaSubscriberImpact, SubscriberBoundaryVisibility, ValidatedSchemaTransition,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaTransitionValidationError {
    EmptyDiff,
    UnstratifiedChange { element_name: Arc<str> },
    NarrowingWithoutPolicy { element_name: Arc<str> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaContinuityBundleIssue {
    IncompleteBundle,
    ContinuationDescriptorDrift {
        boundary_fingerprint: Option<SchemaBoundaryFingerprint>,
    },
    ReconciliationDescriptorDrift,
    ContinuationBoundaryFingerprintMismatch {
        boundary_fingerprint: SchemaBoundaryFingerprint,
    },
    DescriptorSemanticsVersionMismatch {
        expected: DescriptorSemanticsVersion,
        found: DescriptorSemanticsVersion,
    },
    DescriptorCanonicalizationVersionMismatch {
        expected: DescriptorCanonicalizationVersion,
        found: DescriptorCanonicalizationVersion,
    },
    VisibleBridgeProofMismatch,
    TargetSchemaVersionMismatch,
    LineageSchemaVersionMismatch,
    HistoricalReinterpretationViolation,
}

#[derive(Debug, Clone, Copy)]
pub struct ValidatedSchemaContinuityBundle<'a> {
    envelope: &'a crate::replay::data::CanonicalCommitEnvelope,
    transition: Option<&'a crate::schema::data::SchemaTransitionArtifact>,
    continuation: Option<&'a crate::schema::data::SchemaContinuationDescriptor>,
    reconciliation: Option<&'a crate::schema::data::SchemaReconciliationDescriptor>,
}

impl<'a> ValidatedSchemaContinuityBundle<'a> {
    pub fn envelope(&self) -> &'a crate::replay::data::CanonicalCommitEnvelope {
        self.envelope
    }

    pub fn transition(&self) -> Option<&'a crate::schema::data::SchemaTransitionArtifact> {
        self.transition
    }

    pub fn continuation(&self) -> Option<&'a crate::schema::data::SchemaContinuationDescriptor> {
        self.continuation
    }

    pub fn reconciliation(
        &self,
    ) -> Option<&'a crate::schema::data::SchemaReconciliationDescriptor> {
        self.reconciliation
    }
}

impl SchemaContinuityBundleIssue {
    pub fn detail(&self) -> String {
        match self {
            Self::IncompleteBundle => {
                "schema transition, continuation descriptor, and reconciliation descriptor must appear together".to_string()
            }
            Self::ContinuationDescriptorDrift { .. } => {
                "top-level continuation descriptor does not match schema transition artifact"
                    .to_string()
            }
            Self::ReconciliationDescriptorDrift => {
                "top-level reconciliation descriptor does not match schema transition artifact"
                    .to_string()
            }
            Self::ContinuationBoundaryFingerprintMismatch { .. } => {
                "continuation descriptor boundary fingerprint must match bridge boundary fingerprint"
                    .to_string()
            }
            Self::DescriptorSemanticsVersionMismatch { .. } => {
                "descriptor semantics version must agree across envelope, continuation descriptor, and reconciliation descriptor".to_string()
            }
            Self::DescriptorCanonicalizationVersionMismatch { .. } => {
                "descriptor canonicalization version must agree across continuation and reconciliation descriptors and remain supported by runtime policy".to_string()
            }
            Self::VisibleBridgeProofMismatch => {
                "visible bridge continuity requires explicit proof that surfaced boundary metadata is semantically ignorable".to_string()
            }
            Self::TargetSchemaVersionMismatch => {
                "transition target schema version does not match canonical envelope schema version"
                    .to_string()
            }
            Self::LineageSchemaVersionMismatch => {
                "reconciliation lineage target schema version does not match canonical envelope schema version"
                    .to_string()
            }
            Self::HistoricalReinterpretationViolation => {
                "historically sensitive boundaries may not publish as unchanged or transparently bridgeable continuity".to_string()
            }
        }
    }
}

impl SchemaTransitionValidationError {
    pub fn detail(&self) -> String {
        match self {
            Self::EmptyDiff => {
                "schema transition must carry at least one classified diff atom".to_string()
            }
            Self::UnstratifiedChange { element_name } => {
                format!("schema change for '{element_name}' does not declare any schema strata")
            }
            Self::NarrowingWithoutPolicy { element_name } => {
                format!(
                    "schema narrowing for '{element_name}' requires an explicit preservation policy"
                )
            }
        }
    }
}

#[derive(Debug)]
struct NormalizedTransitionView<'a> {
    canonical_atoms: Vec<CanonicalSchemaDiffAtom<'a>>,
    changed_strata: Vec<SchemaStratum>,
    historical_interpretation: HistoricalInterpretationSensitivity,
}

#[derive(Debug, Clone)]
struct CanonicalSchemaDiffAtom<'a> {
    atom: &'a SchemaDiffAtom,
    element_name_sort_key: u64,
    normalized_strata: Vec<SchemaStratum>,
    normalized_detail: CanonicalSchemaDiffDetail<'a>,
}

#[derive(Debug, Clone)]
enum CanonicalSchemaDiffDetail<'a> {
    AddedField {
        field_name: &'a str,
        required: bool,
        default_expression: Option<&'a str>,
    },
    RemovedField {
        field_name: &'a str,
    },
    TypeChanged {
        field_name: &'a str,
        from_type: &'a str,
        to_type: &'a str,
    },
    EnumDomainExpanded {
        field_name: &'a str,
        added_variants: Vec<&'a str>,
    },
    InvariantContractChanged {
        contract_name: &'a str,
    },
    ProjectionContractChanged {
        projection_name: &'a str,
    },
    SubscriberContractChanged {
        contract_name: &'a str,
    },
    FreeText {
        detail: &'a str,
        declared_intent: FreeFormSchemaDiffIntent,
    },
}

pub fn validate_schema_transition(
    proposed: ProposedSchemaTransition,
    policy: Option<SchemaReconciliationPolicy>,
) -> Result<ValidatedSchemaTransition, SchemaTransitionValidationError> {
    if proposed.diff_atoms.is_empty() {
        return Err(SchemaTransitionValidationError::EmptyDiff);
    }

    for atom in &proposed.diff_atoms {
        if atom.strata.is_empty() {
            return Err(SchemaTransitionValidationError::UnstratifiedChange {
                element_name: atom.element.element_name.clone(),
            });
        }
        if is_narrowing(atom) && policy.is_none() {
            return Err(SchemaTransitionValidationError::NarrowingWithoutPolicy {
                element_name: atom.element.element_name.clone(),
            });
        }
    }

    Ok(classify_schema_transition(proposed, policy))
}

pub(crate) fn classify_schema_transition(
    proposed: ProposedSchemaTransition,
    policy: Option<SchemaReconciliationPolicy>,
) -> ValidatedSchemaTransition {
    let mut reconciliation = SchemaReconciliationClassification::Additive;
    let mut continuation = SchemaContinuationClassification::ContinueUnchanged;
    let mut bridgeability = SchemaBridgeabilityClassification::Transparent;

    for atom in &proposed.diff_atoms {
        reconciliation =
            max_reconciliation_classification(reconciliation, classify_reconciliation(atom));
        continuation = max_continuation_classification(continuation, classify_continuation(atom));
        bridgeability =
            max_bridgeability_classification(bridgeability, classify_bridgeability(atom));
    }

    if matches!(
        reconciliation,
        SchemaReconciliationClassification::TypeIncompatible
            | SchemaReconciliationClassification::StructuralIncompatible
    ) {
        continuation = SchemaContinuationClassification::Rejected;
        bridgeability = SchemaBridgeabilityClassification::Rejected;
    }

    if (continuation == SchemaContinuationClassification::Rejected
        || bridgeability == SchemaBridgeabilityClassification::Rejected)
        && reconciliation == SchemaReconciliationClassification::Additive
    {
        reconciliation = max_reconciliation_classification(
            reconciliation,
            SchemaReconciliationClassification::StructuralIncompatible,
        );
    }

    ValidatedSchemaTransition {
        proposed,
        compatibility_observation: if continuation == SchemaContinuationClassification::Rejected
            && matches!(
                reconciliation,
                SchemaReconciliationClassification::TypeIncompatible
                    | SchemaReconciliationClassification::StructuralIncompatible
            ) {
            CompatibilityObservation::RejectedInAllLayers
        } else {
            CompatibilityObservation::NonRejectedInAtLeastOneLayer
        },
        reconciliation,
        continuation,
        bridgeability: if is_contract_upgrade_policy(policy)
            && continuation == SchemaContinuationClassification::ContinueWithContractUpgrade
        {
            SchemaBridgeabilityClassification::ContractUpgradeOnly
        } else {
            bridgeability
        },
    }
}

pub fn lower_schema_transition(
    validated: ValidatedSchemaTransition,
    policy: Option<SchemaReconciliationPolicy>,
    semantics_version: DescriptorSemanticsVersion,
    canonicalization_version: DescriptorCanonicalizationVersion,
) -> LoweredSchemaTransitionPlan {
    let normalized_transition = normalize_transition(&validated.proposed.diff_atoms);
    let fingerprint = fingerprint_transition_from_normalized(&normalized_transition);
    let bridge = SchemaBridgeDescriptor::new_with_visibility(
        fingerprint,
        semantics_version,
        canonicalization_version,
        validated.continuation,
        validated.bridgeability,
        strongest_boundary_visibility(&validated.proposed.diff_atoms),
        normalized_transition.historical_interpretation,
        normalized_transition.changed_strata,
    );
    let continuation_descriptor =
        SchemaContinuationDescriptor::new(fingerprint, bridge, validated.proposed.diff_atoms.len());
    let reconciliation_descriptor = SchemaReconciliationDescriptor::new(
        semantics_version,
        canonicalization_version,
        validated.reconciliation,
        policy.unwrap_or(SchemaReconciliationPolicy::RejectLossyNarrowing),
        SchemaLineageArtifact::new(
            validated.proposed.target_schema_id.clone(),
            validated.proposed.target_schema_version_id,
            vec![validated.proposed.source_schema_id.clone()],
            vec![validated.proposed.source_schema_version_id],
            None,
            SchemaReconciliationOrderingMode::CanonicalizedPair,
            SchemaLineageOrderingSemantics::SymmetricResult,
        ),
    );

    LoweredSchemaTransitionPlan::new(
        validated,
        continuation_descriptor,
        reconciliation_descriptor,
    )
}

pub fn validate_schema_continuity_bundle(
    envelope: &crate::replay::data::CanonicalCommitEnvelope,
) -> Result<ValidatedSchemaContinuityBundle<'_>, SchemaContinuityBundleIssue> {
    let has_transition = envelope.schema_transition.is_some();
    let has_continuation = envelope.schema_continuation_descriptor.is_some();
    let has_reconciliation = envelope.schema_reconciliation_descriptor.is_some();
    if has_transition != has_continuation || has_transition != has_reconciliation {
        return Err(SchemaContinuityBundleIssue::IncompleteBundle);
    }

    let Some(transition) = &envelope.schema_transition else {
        return Ok(ValidatedSchemaContinuityBundle {
            envelope,
            transition: None,
            continuation: None,
            reconciliation: None,
        });
    };
    let Some(continuation) = envelope.schema_continuation_descriptor.as_ref() else {
        return Err(SchemaContinuityBundleIssue::IncompleteBundle);
    };
    let Some(reconciliation) = envelope.schema_reconciliation_descriptor.as_ref() else {
        return Err(SchemaContinuityBundleIssue::IncompleteBundle);
    };

    if transition.continuation_descriptor != *continuation {
        return Err(SchemaContinuityBundleIssue::ContinuationDescriptorDrift {
            boundary_fingerprint: Some(continuation.boundary_fingerprint),
        });
    }
    if transition.reconciliation_descriptor != *reconciliation {
        return Err(SchemaContinuityBundleIssue::ReconciliationDescriptorDrift);
    }
    if continuation.boundary_fingerprint != continuation.bridge.boundary_fingerprint {
        return Err(
            SchemaContinuityBundleIssue::ContinuationBoundaryFingerprintMismatch {
                boundary_fingerprint: continuation.boundary_fingerprint,
            },
        );
    }
    if continuation.bridge.semantics_version != envelope.descriptor_semantics_version {
        return Err(
            SchemaContinuityBundleIssue::DescriptorSemanticsVersionMismatch {
                expected: envelope.descriptor_semantics_version,
                found: continuation.bridge.semantics_version,
            },
        );
    }
    if reconciliation.semantics_version != envelope.descriptor_semantics_version {
        return Err(
            SchemaContinuityBundleIssue::DescriptorSemanticsVersionMismatch {
                expected: envelope.descriptor_semantics_version,
                found: reconciliation.semantics_version,
            },
        );
    }
    if continuation.bridge.canonicalization_version != reconciliation.canonicalization_version {
        return Err(
            SchemaContinuityBundleIssue::DescriptorCanonicalizationVersionMismatch {
                expected: continuation.bridge.canonicalization_version,
                found: reconciliation.canonicalization_version,
            },
        );
    }
    if continuation.bridge.continuation
        == crate::schema::data::SchemaContinuationClassification::ContinueWithVisibleBridge
        && continuation.bridge.boundary_visibility
            != crate::schema::data::SubscriberBoundaryVisibility::VisibleSemanticallyIgnorable
    {
        return Err(SchemaContinuityBundleIssue::VisibleBridgeProofMismatch);
    }
    if transition.target_schema_version_id != envelope.schema_version {
        return Err(SchemaContinuityBundleIssue::TargetSchemaVersionMismatch);
    }
    if reconciliation.resulting_lineage.resulting_schema_version_id != envelope.schema_version {
        return Err(SchemaContinuityBundleIssue::LineageSchemaVersionMismatch);
    }
    if continuation.bridge.historical_interpretation
        != crate::schema::data::HistoricalInterpretationSensitivity::NotSensitive
        && matches!(
            continuation.bridge.continuation,
            crate::schema::data::SchemaContinuationClassification::ContinueUnchanged
                | crate::schema::data::SchemaContinuationClassification::ContinueWithTransparentBridge
        )
    {
        return Err(SchemaContinuityBundleIssue::HistoricalReinterpretationViolation);
    }

    Ok(ValidatedSchemaContinuityBundle {
        envelope,
        transition: Some(transition),
        continuation: Some(continuation),
        reconciliation: Some(reconciliation),
    })
}

fn fingerprint_transition_from_normalized(
    normalized_transition: &NormalizedTransitionView<'_>,
) -> SchemaBoundaryFingerprint {
    let mut hasher = Sha256::new();
    for atom in &normalized_transition.canonical_atoms {
        write_atom_to_hasher(&mut hasher, atom);
    }

    let digest: [u8; 32] = hasher.finalize().into();
    SchemaBoundaryFingerprint::new(digest)
}

fn normalize_transition(diff_atoms: &[SchemaDiffAtom]) -> NormalizedTransitionView<'_> {
    let mut canonical_atoms = Vec::with_capacity(diff_atoms.len());
    let mut changed_strata = BTreeSet::new();
    let mut historical_interpretation = HistoricalInterpretationSensitivity::NotSensitive;

    for atom in diff_atoms {
        for stratum in &atom.strata {
            changed_strata.insert(*stratum);
        }
        historical_interpretation = strongest_historical_interpretation(
            historical_interpretation,
            atom.historical_interpretation,
        );
        canonical_atoms.push(CanonicalSchemaDiffAtom::new(atom));
    }

    canonical_atoms.sort_unstable_by(compare_atoms_canonically);

    NormalizedTransitionView {
        canonical_atoms,
        changed_strata: changed_strata.into_iter().collect(),
        historical_interpretation,
    }
}

impl<'a> CanonicalSchemaDiffAtom<'a> {
    fn new(atom: &'a SchemaDiffAtom) -> Self {
        let mut normalized_strata = atom.strata.clone();
        normalized_strata.sort_unstable();
        normalized_strata.dedup();
        Self {
            atom,
            element_name_sort_key: non_authority_sort_key(atom.element.element_name.as_bytes()),
            normalized_detail: CanonicalSchemaDiffDetail::new(&atom.detail),
            normalized_strata,
        }
    }
}

impl<'a> CanonicalSchemaDiffDetail<'a> {
    fn new(detail: &'a SchemaDiffDetail) -> Self {
        match detail {
            SchemaDiffDetail::AddedField {
                field_name,
                required,
                default_expression,
            } => Self::AddedField {
                field_name: field_name.as_ref(),
                required: *required,
                default_expression: default_expression.as_deref(),
            },
            SchemaDiffDetail::RemovedField { field_name } => Self::RemovedField {
                field_name: field_name.as_ref(),
            },
            SchemaDiffDetail::TypeChanged {
                field_name,
                from_type,
                to_type,
            } => Self::TypeChanged {
                field_name: field_name.as_ref(),
                from_type: from_type.as_ref(),
                to_type: to_type.as_ref(),
            },
            SchemaDiffDetail::EnumDomainExpanded {
                field_name,
                added_variants,
            } => {
                let mut normalized_variants = added_variants
                    .iter()
                    .map(|variant| variant.as_ref())
                    .collect::<Vec<_>>();
                normalized_variants.sort_unstable();
                normalized_variants.dedup();
                Self::EnumDomainExpanded {
                    field_name: field_name.as_ref(),
                    added_variants: normalized_variants,
                }
            }
            SchemaDiffDetail::InvariantContractChanged { contract_name } => {
                Self::InvariantContractChanged {
                    contract_name: contract_name.as_ref(),
                }
            }
            SchemaDiffDetail::ProjectionContractChanged { projection_name } => {
                Self::ProjectionContractChanged {
                    projection_name: projection_name.as_ref(),
                }
            }
            SchemaDiffDetail::SubscriberContractChanged { contract_name } => {
                Self::SubscriberContractChanged {
                    contract_name: contract_name.as_ref(),
                }
            }
            SchemaDiffDetail::FreeText {
                detail,
                declared_intent,
            } => Self::FreeText {
                detail: detail.as_ref(),
                declared_intent: *declared_intent,
            },
        }
    }
}

fn compare_atoms_canonically(
    left: &CanonicalSchemaDiffAtom<'_>,
    right: &CanonicalSchemaDiffAtom<'_>,
) -> Ordering {
    left.atom
        .element
        .schema_id
        .0
        .cmp(&right.atom.element.schema_id.0)
        .then_with(|| {
            left.atom
                .element
                .schema_version_id
                .cmp(&right.atom.element.schema_version_id)
        })
        .then_with(|| left.atom.element.kind.cmp(&right.atom.element.kind))
        .then_with(|| left.atom.element.kind_id.cmp(&right.atom.element.kind_id))
        .then_with(|| left.element_name_sort_key.cmp(&right.element_name_sort_key))
        .then_with(|| {
            left.atom
                .element
                .element_name
                .cmp(&right.atom.element.element_name)
        })
        .then_with(|| left.normalized_strata.cmp(&right.normalized_strata))
        .then_with(|| {
            left.atom
                .publication_impact
                .cmp(&right.atom.publication_impact)
        })
        .then_with(|| {
            left.atom
                .subscriber_impact
                .cmp(&right.atom.subscriber_impact)
        })
        .then_with(|| {
            left.atom
                .historical_interpretation
                .cmp(&right.atom.historical_interpretation)
        })
        .then_with(|| compare_detail_canonically(&left.normalized_detail, &right.normalized_detail))
}

fn compare_detail_canonically(
    left: &CanonicalSchemaDiffDetail<'_>,
    right: &CanonicalSchemaDiffDetail<'_>,
) -> Ordering {
    detail_sort_key(left)
        .cmp(&detail_sort_key(right))
        .then_with(|| detail_cmp_payload(left, right))
}

fn detail_sort_key(detail: &CanonicalSchemaDiffDetail<'_>) -> u8 {
    match detail {
        CanonicalSchemaDiffDetail::AddedField { .. } => 1,
        CanonicalSchemaDiffDetail::RemovedField { .. } => 2,
        CanonicalSchemaDiffDetail::TypeChanged { .. } => 3,
        CanonicalSchemaDiffDetail::EnumDomainExpanded { .. } => 4,
        CanonicalSchemaDiffDetail::InvariantContractChanged { .. } => 5,
        CanonicalSchemaDiffDetail::ProjectionContractChanged { .. } => 6,
        CanonicalSchemaDiffDetail::SubscriberContractChanged { .. } => 7,
        CanonicalSchemaDiffDetail::FreeText { .. } => 8,
    }
}

fn detail_cmp_payload(
    left: &CanonicalSchemaDiffDetail<'_>,
    right: &CanonicalSchemaDiffDetail<'_>,
) -> Ordering {
    match (left, right) {
        (
            CanonicalSchemaDiffDetail::AddedField {
                field_name: lf,
                required: lr,
                default_expression: ld,
            },
            CanonicalSchemaDiffDetail::AddedField {
                field_name: rf,
                required: rr,
                default_expression: rd,
            },
        ) => lf.cmp(rf).then_with(|| lr.cmp(rr)).then_with(|| ld.cmp(rd)),
        (
            CanonicalSchemaDiffDetail::RemovedField { field_name: lf },
            CanonicalSchemaDiffDetail::RemovedField { field_name: rf },
        ) => lf.cmp(rf),
        (
            CanonicalSchemaDiffDetail::TypeChanged {
                field_name: lf,
                from_type: lfrom,
                to_type: lto,
            },
            CanonicalSchemaDiffDetail::TypeChanged {
                field_name: rf,
                from_type: rfrom,
                to_type: rto,
            },
        ) => lf
            .cmp(rf)
            .then_with(|| lfrom.cmp(rfrom))
            .then_with(|| lto.cmp(rto)),
        (
            CanonicalSchemaDiffDetail::EnumDomainExpanded {
                field_name: lf,
                added_variants: lv,
            },
            CanonicalSchemaDiffDetail::EnumDomainExpanded {
                field_name: rf,
                added_variants: rv,
            },
        ) => lf.cmp(rf).then_with(|| lv.cmp(rv)),
        (
            CanonicalSchemaDiffDetail::InvariantContractChanged { contract_name: lf },
            CanonicalSchemaDiffDetail::InvariantContractChanged { contract_name: rf },
        ) => lf.cmp(rf),
        (
            CanonicalSchemaDiffDetail::ProjectionContractChanged {
                projection_name: lf,
            },
            CanonicalSchemaDiffDetail::ProjectionContractChanged {
                projection_name: rf,
            },
        ) => lf.cmp(rf),
        (
            CanonicalSchemaDiffDetail::SubscriberContractChanged { contract_name: lf },
            CanonicalSchemaDiffDetail::SubscriberContractChanged { contract_name: rf },
        ) => lf.cmp(rf),
        (
            CanonicalSchemaDiffDetail::FreeText {
                detail: lf,
                declared_intent: li,
            },
            CanonicalSchemaDiffDetail::FreeText {
                detail: rf,
                declared_intent: ri,
            },
        ) => lf.cmp(rf).then_with(|| li.cmp(ri)),
        _ => Ordering::Equal,
    }
}

fn write_atom_to_hasher(hasher: &mut Sha256, atom: &CanonicalSchemaDiffAtom<'_>) {
    update_tagged_bytes(hasher, atom.atom.element.schema_id.0.as_bytes());
    hasher.update(atom.atom.element.schema_version_id.0.to_le_bytes());
    hasher.update([atom.atom.element.kind as u8]);
    match atom.atom.element.kind_id {
        Some(kind_id) => {
            hasher.update([1]);
            hasher.update(kind_id.0.to_le_bytes());
        }
        None => hasher.update([0]),
    }
    update_tagged_bytes(hasher, atom.atom.element.element_name.as_bytes());
    hasher.update((atom.normalized_strata.len() as u64).to_le_bytes());
    for stratum in &atom.normalized_strata {
        hasher.update([*stratum as u8]);
    }
    hasher.update([atom.atom.publication_impact as u8]);
    hasher.update([atom.atom.subscriber_impact as u8]);
    hasher.update([atom.atom.historical_interpretation as u8]);
    write_detail_to_hasher(hasher, &atom.normalized_detail);
}

fn write_detail_to_hasher(hasher: &mut Sha256, detail: &CanonicalSchemaDiffDetail<'_>) {
    hasher.update([detail_sort_key(detail)]);
    match detail {
        CanonicalSchemaDiffDetail::AddedField {
            field_name,
            required,
            default_expression,
        } => {
            update_tagged_bytes(hasher, field_name.as_bytes());
            hasher.update([u8::from(*required)]);
            match default_expression {
                Some(expr) => {
                    hasher.update([1]);
                    update_tagged_bytes(hasher, expr.as_bytes());
                }
                None => hasher.update([0]),
            }
        }
        CanonicalSchemaDiffDetail::RemovedField { field_name } => {
            update_tagged_bytes(hasher, field_name.as_bytes());
        }
        CanonicalSchemaDiffDetail::TypeChanged {
            field_name,
            from_type,
            to_type,
        } => {
            update_tagged_bytes(hasher, field_name.as_bytes());
            update_tagged_bytes(hasher, from_type.as_bytes());
            update_tagged_bytes(hasher, to_type.as_bytes());
        }
        CanonicalSchemaDiffDetail::EnumDomainExpanded {
            field_name,
            added_variants,
        } => {
            update_tagged_bytes(hasher, field_name.as_bytes());
            hasher.update((added_variants.len() as u64).to_le_bytes());
            for variant in added_variants {
                update_tagged_bytes(hasher, variant.as_bytes());
            }
        }
        CanonicalSchemaDiffDetail::InvariantContractChanged { contract_name }
        | CanonicalSchemaDiffDetail::SubscriberContractChanged { contract_name } => {
            update_tagged_bytes(hasher, contract_name.as_bytes());
        }
        CanonicalSchemaDiffDetail::ProjectionContractChanged { projection_name } => {
            update_tagged_bytes(hasher, projection_name.as_bytes());
        }
        CanonicalSchemaDiffDetail::FreeText {
            detail,
            declared_intent,
        } => {
            update_tagged_bytes(hasher, detail.as_bytes());
            hasher.update([*declared_intent as u8]);
        }
    }
}

fn update_tagged_bytes(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
}

fn non_authority_sort_key(bytes: &[u8]) -> u64 {
    const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET_BASIS;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

fn classify_reconciliation(atom: &SchemaDiffAtom) -> SchemaReconciliationClassification {
    match &atom.detail {
        SchemaDiffDetail::AddedField { .. }
        | SchemaDiffDetail::EnumDomainExpanded { .. }
        | SchemaDiffDetail::ProjectionContractChanged { .. }
        | SchemaDiffDetail::SubscriberContractChanged { .. } => {
            SchemaReconciliationClassification::Additive
        }
        SchemaDiffDetail::RemovedField { .. } => SchemaReconciliationClassification::Narrowing,
        SchemaDiffDetail::TypeChanged { .. } => {
            SchemaReconciliationClassification::TypeIncompatible
        }
        SchemaDiffDetail::InvariantContractChanged { .. } => {
            if atom.strata.contains(&SchemaStratum::BehavioralSemantics)
                || atom
                    .strata
                    .contains(&SchemaStratum::EntityIdentitySemantics)
                || atom.strata.contains(&SchemaStratum::LineageSemantics)
            {
                SchemaReconciliationClassification::StructuralIncompatible
            } else {
                SchemaReconciliationClassification::Additive
            }
        }
        SchemaDiffDetail::FreeText {
            declared_intent, ..
        } => match declared_intent {
            FreeFormSchemaDiffIntent::Additive => SchemaReconciliationClassification::Additive,
            FreeFormSchemaDiffIntent::StructuralIncompatible => {
                SchemaReconciliationClassification::StructuralIncompatible
            }
        },
    }
}

fn classify_continuation(atom: &SchemaDiffAtom) -> SchemaContinuationClassification {
    match atom.subscriber_impact {
        SchemaSubscriberImpact::None => SchemaContinuationClassification::ContinueUnchanged,
        SchemaSubscriberImpact::ConsumableSurfaceChanged => {
            if atom.historical_interpretation == HistoricalInterpretationSensitivity::NotSensitive
                && atom.boundary_visibility
                    == SubscriberBoundaryVisibility::VisibleSemanticallyIgnorable
            {
                SchemaContinuationClassification::ContinueWithVisibleBridge
            } else {
                SchemaContinuationClassification::RequireRenegotiation
            }
        }
        SchemaSubscriberImpact::ContractUpgradeRequired => {
            SchemaContinuationClassification::ContinueWithContractUpgrade
        }
        SchemaSubscriberImpact::RenegotiationRequired => {
            SchemaContinuationClassification::RequireRenegotiation
        }
    }
}

fn strongest_boundary_visibility(diff_atoms: &[SchemaDiffAtom]) -> SubscriberBoundaryVisibility {
    diff_atoms
        .iter()
        .map(|atom| atom.boundary_visibility)
        .max()
        .unwrap_or(SubscriberBoundaryVisibility::NotVisible)
}

fn strongest_historical_interpretation(
    current: HistoricalInterpretationSensitivity,
    candidate: HistoricalInterpretationSensitivity,
) -> HistoricalInterpretationSensitivity {
    if candidate.sensitivity_rank() > current.sensitivity_rank() {
        candidate
    } else {
        current
    }
}

fn classify_bridgeability(atom: &SchemaDiffAtom) -> SchemaBridgeabilityClassification {
    match classify_continuation(atom) {
        SchemaContinuationClassification::ContinueUnchanged
        | SchemaContinuationClassification::ContinueWithTransparentBridge => {
            SchemaBridgeabilityClassification::Transparent
        }
        SchemaContinuationClassification::ContinueWithVisibleBridge => {
            SchemaBridgeabilityClassification::SubscriberVisible
        }
        SchemaContinuationClassification::ContinueWithContractUpgrade => {
            SchemaBridgeabilityClassification::ContractUpgradeOnly
        }
        SchemaContinuationClassification::RequireRenegotiation => {
            SchemaBridgeabilityClassification::RenegotiationOnly
        }
        SchemaContinuationClassification::Rejected => SchemaBridgeabilityClassification::Rejected,
    }
}

fn is_narrowing(atom: &SchemaDiffAtom) -> bool {
    matches!(atom.detail, SchemaDiffDetail::RemovedField { .. })
}

fn is_contract_upgrade_policy(policy: Option<SchemaReconciliationPolicy>) -> bool {
    matches!(
        policy,
        Some(SchemaReconciliationPolicy::RequireExplicitProjection)
    )
}

fn max_reconciliation_classification(
    current: SchemaReconciliationClassification,
    candidate: SchemaReconciliationClassification,
) -> SchemaReconciliationClassification {
    use SchemaReconciliationClassification::*;
    match (current, candidate) {
        (StructuralIncompatible, _) | (_, StructuralIncompatible) => StructuralIncompatible,
        (TypeIncompatible, _) | (_, TypeIncompatible) => TypeIncompatible,
        (Narrowing, _) | (_, Narrowing) => Narrowing,
        _ => Additive,
    }
}

fn max_continuation_classification(
    current: SchemaContinuationClassification,
    candidate: SchemaContinuationClassification,
) -> SchemaContinuationClassification {
    use SchemaContinuationClassification::*;
    match (current, candidate) {
        (Rejected, _) | (_, Rejected) => Rejected,
        (RequireRenegotiation, _) | (_, RequireRenegotiation) => RequireRenegotiation,
        (ContinueWithContractUpgrade, _) | (_, ContinueWithContractUpgrade) => {
            ContinueWithContractUpgrade
        }
        (ContinueWithVisibleBridge, _) | (_, ContinueWithVisibleBridge) => {
            ContinueWithVisibleBridge
        }
        (ContinueWithTransparentBridge, _) | (_, ContinueWithTransparentBridge) => {
            ContinueWithTransparentBridge
        }
        _ => ContinueUnchanged,
    }
}

fn max_bridgeability_classification(
    current: SchemaBridgeabilityClassification,
    candidate: SchemaBridgeabilityClassification,
) -> SchemaBridgeabilityClassification {
    use SchemaBridgeabilityClassification::*;
    match (current, candidate) {
        (Rejected, _) | (_, Rejected) => Rejected,
        (RenegotiationOnly, _) | (_, RenegotiationOnly) => RenegotiationOnly,
        (ContractUpgradeOnly, _) | (_, ContractUpgradeOnly) => ContractUpgradeOnly,
        (SubscriberVisible, _) | (_, SubscriberVisible) => SubscriberVisible,
        _ => Transparent,
    }
}
