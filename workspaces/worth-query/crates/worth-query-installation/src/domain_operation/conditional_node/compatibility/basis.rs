use worth_foundational::facade::{
    canonicalization, prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact, CanonicalBasisValue,
    CanonicalIntegerWidth, CanonicalizationRuleVersion,
};

use super::dimension::{
    WorthQueryPortableConditionalDependencyLocation as DependencyLocation,
    WorthQueryPortableConditionalDependencyPart as DependencyPart,
    WorthQueryPortableConditionalDimension as Dimension,
};
use super::output_basis::{append_context, append_outputs};
use super::value_basis::*;
use crate::domain_operation::{
    WorthQueryConditionalConditionClass, WorthQueryConditionalEvaluationCondition,
    WorthQueryDeltaThreshold, WorthQueryPortableConditionalNodeDeclaration,
    WorthQuerySemanticTruthDependency,
};

const DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("worth-query-portable-conditional");

pub(super) struct DimensionBasis {
    pub dimension: Dimension,
    pub foundational: CanonicalBasisReadyArtifact,
}

pub(super) fn portable_conditional_rule_version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("worth-query-portable-conditional-v1")
        .expect("static canonicalization version is valid")
}

pub(super) fn declaration_bases(
    declaration: &WorthQueryPortableConditionalNodeDeclaration,
    version: CanonicalizationRuleVersion,
) -> Vec<DimensionBasis> {
    let mut bases = vec![
        primitive(Dimension::Identity, text(declaration.identity()), &version),
        primitive(
            Dimension::Role,
            text(role_name(declaration.role())),
            &version,
        ),
        primitive(
            Dimension::DependencyWidth,
            unsigned(declaration.dependencies().len()),
            &version,
        ),
    ];
    for (index, dependency) in declaration.dependencies().iter().enumerate() {
        append_dependency(
            &mut bases,
            dependency,
            DependencyLocation::Declaration(index as u32),
            &version,
        );
    }
    append_outputs(&mut bases, declaration, &version);
    append_context(&mut bases, declaration, &version);
    append_condition(&mut bases, declaration.condition(), &version);
    append_evaluation_contract(&mut bases, declaration, &version);
    bases
}

fn append_dependency(
    bases: &mut Vec<DimensionBasis>,
    dependency: &WorthQuerySemanticTruthDependency,
    location: DependencyLocation,
    version: &CanonicalizationRuleVersion,
) {
    let dimension = |part| Dimension::Dependency {
        location: location.clone(),
        part,
    };
    bases.push(primitive(
        dimension(DependencyPart::GraphReadRole),
        text(dependency.graph_read_role().as_str()),
        version,
    ));
    let contract = canonicalization()
        .basis()
        .at(version.clone())
        .from_contract(dependency.contract().clone())
        .into_result()
        .expect("admitted dependency contract has a canonical basis");
    bases.push(DimensionBasis {
        dimension: dimension(DependencyPart::Contract),
        foundational: contract,
    });
    let mask = canonicalization()
        .basis()
        .at(version.clone())
        .from_mask(
            dependency.contract().key().clone(),
            dependency.projection_mask().clone(),
        )
        .into_result()
        .expect("admitted dependency mask has a canonical basis");
    bases.push(DimensionBasis {
        dimension: dimension(DependencyPart::ProjectionMask),
        foundational: mask,
    });
    bases.push(primitive(
        dimension(DependencyPart::Binding),
        text(dependency.binding().canonical_name()),
        version,
    ));
    bases.push(structured(
        dimension(DependencyPart::Locality),
        locality_values(dependency.locality()),
        version,
    ));
    bases.push(primitive(
        dimension(DependencyPart::RelevantChangeWidth),
        unsigned(dependency.relevant_changes().len()),
        version,
    ));
    bases.extend(
        dependency
            .relevant_changes()
            .iter()
            .enumerate()
            .map(|(index, change)| {
                primitive(
                    dimension(DependencyPart::RelevantChange(index as u32)),
                    text(change.canonical_name()),
                    version,
                )
            }),
    );
}

fn append_condition(
    bases: &mut Vec<DimensionBasis>,
    condition: &WorthQueryConditionalEvaluationCondition,
    version: &CanonicalizationRuleVersion,
) {
    bases.push(primitive(
        Dimension::ConditionClass,
        text(condition_class_name(condition.class())),
        version,
    ));
    match condition.class() {
        WorthQueryConditionalConditionClass::AspectFiltered => {
            append_condition_dependencies(bases, condition.dependencies(), version)
        }
        WorthQueryConditionalConditionClass::DeltaThreshold => {
            let (dependency, threshold) = condition
                .delta_threshold_contract()
                .expect("delta threshold class retains its contract");
            bases.push(primitive(
                Dimension::ConditionDependencyWidth,
                unsigned(1),
                version,
            ));
            append_dependency(
                bases,
                dependency,
                DependencyLocation::DeltaThreshold,
                version,
            );
            append_threshold(bases, threshold, version);
        }
        WorthQueryConditionalConditionClass::Temporal => bases.push(structured(
            Dimension::TemporalCondition,
            temporal_condition_values(
                condition
                    .temporal_condition()
                    .expect("temporal class retains its condition"),
            ),
            version,
        )),
        WorthQueryConditionalConditionClass::DomainSpecific => {
            append_domain_condition(bases, condition, version)
        }
        WorthQueryConditionalConditionClass::AlwaysEligible
        | WorthQueryConditionalConditionClass::OnDemand => {}
    }
}

fn append_condition_dependencies(
    bases: &mut Vec<DimensionBasis>,
    dependencies: &[WorthQuerySemanticTruthDependency],
    version: &CanonicalizationRuleVersion,
) {
    bases.push(primitive(
        Dimension::ConditionDependencyWidth,
        unsigned(dependencies.len()),
        version,
    ));
    for (index, dependency) in dependencies.iter().enumerate() {
        append_dependency(
            bases,
            dependency,
            DependencyLocation::AspectFilter(index as u32),
            version,
        );
    }
}

fn append_threshold(
    bases: &mut Vec<DimensionBasis>,
    threshold: &WorthQueryDeltaThreshold,
    version: &CanonicalizationRuleVersion,
) {
    bases.push(primitive(
        Dimension::DeltaThresholdValue,
        aspect_value(threshold.value()),
        version,
    ));
    bases.push(primitive(
        Dimension::DeltaThresholdUnit,
        text(threshold.unit().as_str()),
        version,
    ));
    bases.push(primitive(
        Dimension::DeltaThresholdValueFamily,
        text(value_family_name(threshold.value_family())),
        version,
    ));
    bases.push(primitive(
        Dimension::DeltaThresholdComparisonDomain,
        text(comparison_domain_name(threshold.comparison_domain())),
        version,
    ));
    bases.push(primitive(
        Dimension::DeltaThresholdBoundary,
        text(boundary_name(threshold.boundary())),
        version,
    ));
}

fn append_domain_condition(
    bases: &mut Vec<DimensionBasis>,
    condition: &WorthQueryConditionalEvaluationCondition,
    version: &CanonicalizationRuleVersion,
) {
    bases.push(primitive(
        Dimension::DomainConditionFamily,
        text(
            condition
                .portable_family_identity()
                .expect("domain condition retains its family")
                .as_str(),
        ),
        version,
    ));
    let parameters = condition.domain_specific_parameters();
    bases.push(primitive(
        Dimension::DomainConditionParameterWidth,
        unsigned(parameters.len()),
        version,
    ));
    for (index, parameter) in parameters.iter().enumerate() {
        let index = index as u32;
        bases.push(primitive(
            Dimension::DomainConditionParameterName(index),
            text(parameter.name()),
            version,
        ));
        bases.push(structured(
            Dimension::DomainConditionParameterValue(index),
            parameter_values(parameter.value()),
            version,
        ));
    }
}

fn append_evaluation_contract(
    bases: &mut Vec<DimensionBasis>,
    declaration: &WorthQueryPortableConditionalNodeDeclaration,
    version: &CanonicalizationRuleVersion,
) {
    bases.extend([
        structured(
            Dimension::Trigger,
            trigger_values(declaration.trigger()),
            version,
        ),
        structured(
            Dimension::DependencyComparator,
            comparator_values(declaration.dependency_comparator()),
            version,
        ),
        structured(
            Dimension::OutputEquivalence,
            output_equivalence_values(declaration.output_equivalence()),
            version,
        ),
        structured(
            Dimension::ArtifactReuseEquivalence,
            artifact_reuse_values(declaration.artifact_reuse_equivalence()),
            version,
        ),
        primitive(
            Dimension::Maintenance,
            text(super::super::declaration::maintenance_name(
                declaration.maintenance(),
            )),
            version,
        ),
        primitive(
            Dimension::ArtifactPosture,
            text(super::super::declaration::artifact_name(
                declaration.artifact(),
            )),
            version,
        ),
        primitive(
            Dimension::OutputRelationship,
            text(super::super::declaration::output_relationship_name(
                declaration.output_relationship(),
            )),
            version,
        ),
    ]);
}

pub(super) fn primitive(
    dimension: Dimension,
    value: CanonicalBasisValue,
    version: &CanonicalizationRuleVersion,
) -> DimensionBasis {
    structured(dimension, [("value", value)], version)
}

pub(super) fn structured(
    dimension: Dimension,
    values: impl IntoIterator<Item = (&'static str, CanonicalBasisValue)>,
    version: &CanonicalizationRuleVersion,
) -> DimensionBasis {
    let entries = values.into_iter().map(|(locus, value)| {
        CanonicalBasisEntry::new(
            DOMAIN,
            CanonicalBasisLocus::Named(locus.into()),
            CanonicalBasisEntryKind::Value,
            value,
        )
    });
    let foundational = prepare_canonical_basis_sequence(version.clone(), DOMAIN, entries)
        .into_result()
        .expect("a typed conditional dimension is a valid canonical basis");
    DimensionBasis {
        dimension,
        foundational,
    }
}

pub(super) fn text(value: impl Into<String>) -> CanonicalBasisValue {
    CanonicalBasisValue::ExactText(value.into().into())
}

pub(super) fn unsigned(value: usize) -> CanonicalBasisValue {
    CanonicalBasisValue::UnsignedInteger {
        width: CanonicalIntegerWidth::Bits64,
        value: value as u128,
    }
}
