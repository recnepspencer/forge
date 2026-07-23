use worth_foundational::facade::{
    compare_canonical_basis, prepare_canonical_basis_sequence, prepare_canonical_comparison,
    CanonicalBasisDomain, CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus,
    CanonicalBasisValue, CanonicalComparisonOutcome, CanonicalEquivalenceBasis,
    CanonicalEquivalentBasis, CanonicalIntegerWidth, CanonicalMismatchBasis,
    CanonicalizationRuleVersion,
};

use crate::domain_operation::{
    compare_portable_conditional_node_declarations, WorthQueryConditionalNodeLocation,
    WorthQueryDomainOperationSemanticClosure, WorthQueryOperationWorkflowContract,
    WorthQueryPortableConditionalComparisonEquivalent,
    WorthQueryPortableConditionalComparisonOutcome, WorthQueryPortableConditionalDimension,
    WorthQueryPortableConditionalNodeDeclaration, WorthQueryPortableDomainOperationDefinition,
};

const DOMAIN: CanonicalBasisDomain =
    CanonicalBasisDomain::Future("worth-query-operation-conditionals");

pub fn compare_portable_operation_conditionals(
    left: &WorthQueryPortableDomainOperationDefinition,
    right: &WorthQueryPortableDomainOperationDefinition,
) -> WorthQueryOperationConditionalComparisonOutcome {
    let left_width = conditional_inventory_width(left.semantics());
    let right_width = conditional_inventory_width(right.semantics());
    let mut foundational = Vec::new();
    let mut declarations = Vec::new();
    let mut comparison_count = 1_u32;

    match compare_values(
        [("width", unsigned(left_width))],
        [("width", unsigned(right_width))],
    ) {
        CanonicalComparisonOutcome::Equivalent(basis) => foundational.push(basis),
        CanonicalComparisonOutcome::Mismatched(basis) => {
            return mismatch(
                WorthQueryOperationConditionalDimension::InventoryWidth,
                basis,
                comparison_count,
            )
        }
        CanonicalComparisonOutcome::Unsupported(basis) => {
            return unsupported(
                WorthQueryOperationConditionalDimension::InventoryWidth,
                basis,
                comparison_count,
            )
        }
    }

    for (index, ((left_location, left_node), (right_location, right_node))) in
        canonical_conditional_inventory(left.semantics())
            .zip(canonical_conditional_inventory(right.semantics()))
            .enumerate()
    {
        comparison_count = comparison_count.saturating_add(1);
        let dimension = WorthQueryOperationConditionalDimension::Location(index as u32);
        match compare_values(
            location_values(&left_location),
            location_values(&right_location),
        ) {
            CanonicalComparisonOutcome::Equivalent(basis) => foundational.push(basis),
            CanonicalComparisonOutcome::Mismatched(basis) => {
                return mismatch(dimension, basis, comparison_count)
            }
            CanonicalComparisonOutcome::Unsupported(basis) => {
                return unsupported(dimension, basis, comparison_count)
            }
        }
        match compare_portable_conditional_node_declarations(left_node, right_node) {
            WorthQueryPortableConditionalComparisonOutcome::Equivalent(equivalent) => {
                comparison_count = comparison_count.saturating_add(equivalent.comparison_count());
                declarations.push(equivalent)
            }
            WorthQueryPortableConditionalComparisonOutcome::Mismatched(mismatch) => {
                comparison_count = comparison_count.saturating_add(mismatch.comparison_count());
                return WorthQueryOperationConditionalComparisonOutcome::Mismatched(
                    WorthQueryOperationConditionalComparisonMismatch {
                        dimension: WorthQueryOperationConditionalDimension::Declaration {
                            location: left_location,
                            dimension: mismatch.dimension().clone(),
                        },
                        foundational: mismatch.foundational_basis().clone(),
                        comparison_count,
                    },
                );
            }
            WorthQueryPortableConditionalComparisonOutcome::Unsupported(unsupported) => {
                comparison_count = comparison_count.saturating_add(unsupported.comparison_count());
                return WorthQueryOperationConditionalComparisonOutcome::Unsupported(
                    WorthQueryOperationConditionalComparisonUnsupported {
                        dimension: WorthQueryOperationConditionalDimension::Declaration {
                            location: left_location,
                            dimension: unsupported.dimension().clone(),
                        },
                        foundational: unsupported.foundational_basis().clone(),
                        comparison_count,
                    },
                );
            }
        }
    }

    WorthQueryOperationConditionalComparisonOutcome::Equivalent(
        WorthQueryOperationConditionalComparisonEquivalent {
            foundational,
            declarations,
        },
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryOperationConditionalDimension {
    InventoryWidth,
    Location(u32),
    Declaration {
        location: WorthQueryConditionalNodeLocation,
        dimension: WorthQueryPortableConditionalDimension,
    },
}

#[derive(Debug)]
pub enum WorthQueryOperationConditionalComparisonOutcome {
    Equivalent(WorthQueryOperationConditionalComparisonEquivalent),
    Mismatched(WorthQueryOperationConditionalComparisonMismatch),
    Unsupported(WorthQueryOperationConditionalComparisonUnsupported),
}

#[derive(Debug)]
pub struct WorthQueryOperationConditionalComparisonEquivalent {
    foundational: Vec<CanonicalEquivalentBasis>,
    declarations: Vec<WorthQueryPortableConditionalComparisonEquivalent>,
}

impl WorthQueryOperationConditionalComparisonEquivalent {
    pub fn node_count(&self) -> u32 {
        self.declarations.len() as u32
    }

    pub fn comparison_count(&self) -> u32 {
        self.foundational.len() as u32
            + self
                .declarations
                .iter()
                .map(|declaration| declaration.comparison_count())
                .sum::<u32>()
    }
}

#[derive(Debug)]
pub struct WorthQueryOperationConditionalComparisonMismatch {
    dimension: WorthQueryOperationConditionalDimension,
    foundational: CanonicalMismatchBasis,
    comparison_count: u32,
}

impl WorthQueryOperationConditionalComparisonMismatch {
    pub fn dimension(&self) -> &WorthQueryOperationConditionalDimension {
        &self.dimension
    }

    pub fn foundational_basis(&self) -> &CanonicalMismatchBasis {
        &self.foundational
    }

    pub fn comparison_count(&self) -> u32 {
        self.comparison_count
    }
}

#[derive(Debug)]
pub struct WorthQueryOperationConditionalComparisonUnsupported {
    dimension: WorthQueryOperationConditionalDimension,
    foundational: CanonicalMismatchBasis,
    comparison_count: u32,
}

impl WorthQueryOperationConditionalComparisonUnsupported {
    pub fn dimension(&self) -> &WorthQueryOperationConditionalDimension {
        &self.dimension
    }

    pub fn foundational_basis(&self) -> &CanonicalMismatchBasis {
        &self.foundational
    }

    pub fn comparison_count(&self) -> u32 {
        self.comparison_count
    }
}

// Definition admission already canonicalizes operation nodes, workflow stages,
// and each stage's nodes. Location variant order places operation nodes before
// workflow nodes, so this traversal is canonical without comparison-time sort.
fn canonical_conditional_inventory(
    semantics: &WorthQueryDomainOperationSemanticClosure,
) -> impl Iterator<
    Item = (
        WorthQueryConditionalNodeLocation,
        &WorthQueryPortableConditionalNodeDeclaration,
    ),
> {
    let operation = semantics.conditional_nodes.iter().map(|node| {
        (
            WorthQueryConditionalNodeLocation::operation(node.identity())
                .expect("admitted node identity is a valid location"),
            node,
        )
    });
    let stages = match &semantics.workflow {
        WorthQueryOperationWorkflowContract::Declared(workflow) => workflow.stages(),
        WorthQueryOperationWorkflowContract::NotRequired => &[],
    };
    operation.chain(stages.iter().flat_map(|stage| {
        stage.semantics().conditional_nodes.iter().map(move |node| {
            (
                WorthQueryConditionalNodeLocation::workflow_stage(
                    stage.identity(),
                    node.identity(),
                )
                .expect("admitted stage and node identities form a valid location"),
                node,
            )
        })
    }))
}

fn conditional_inventory_width(semantics: &WorthQueryDomainOperationSemanticClosure) -> usize {
    let workflow = match &semantics.workflow {
        WorthQueryOperationWorkflowContract::Declared(workflow) => workflow
            .stages()
            .iter()
            .map(|stage| stage.semantics().conditional_nodes.len())
            .sum(),
        WorthQueryOperationWorkflowContract::NotRequired => 0,
    };
    semantics.conditional_nodes.len() + workflow
}

fn location_values(
    location: &WorthQueryConditionalNodeLocation,
) -> Vec<(&'static str, CanonicalBasisValue)> {
    match location {
        WorthQueryConditionalNodeLocation::Operation { node_identity } => {
            vec![("kind", text("operation")), ("node", text(node_identity))]
        }
        WorthQueryConditionalNodeLocation::WorkflowStage {
            stage_identity,
            node_identity,
        } => vec![
            ("kind", text("workflow-stage")),
            ("stage", text(stage_identity)),
            ("node", text(node_identity)),
        ],
    }
}

fn compare_values(
    left: impl IntoIterator<Item = (&'static str, CanonicalBasisValue)>,
    right: impl IntoIterator<Item = (&'static str, CanonicalBasisValue)>,
) -> CanonicalComparisonOutcome {
    let left = prepare_sequence(left);
    let right = prepare_sequence(right);
    let ready =
        prepare_canonical_comparison(CanonicalEquivalenceBasis::ExactCanonicalBasis, left, right)
            .into_result()
            .expect("canonical comparison preparation is infallible");
    compare_canonical_basis(&ready)
}

fn prepare_sequence(
    values: impl IntoIterator<Item = (&'static str, CanonicalBasisValue)>,
) -> worth_foundational::facade::CanonicalBasisReadyArtifact {
    let entries = values.into_iter().map(|(locus, value)| {
        CanonicalBasisEntry::new(
            DOMAIN,
            CanonicalBasisLocus::Named(locus.into()),
            CanonicalBasisEntryKind::Value,
            value,
        )
    });
    prepare_canonical_basis_sequence(version(), DOMAIN, entries)
        .into_result()
        .expect("operation conditional inventory basis is nonempty")
}

fn version() -> CanonicalizationRuleVersion {
    CanonicalizationRuleVersion::new("worth-query-operation-conditionals-v1")
        .expect("static canonicalization version is valid")
}

fn text(value: impl Into<String>) -> CanonicalBasisValue {
    CanonicalBasisValue::ExactText(value.into().into())
}

fn unsigned(value: usize) -> CanonicalBasisValue {
    CanonicalBasisValue::UnsignedInteger {
        width: CanonicalIntegerWidth::Bits64,
        value: value as u128,
    }
}

fn mismatch(
    dimension: WorthQueryOperationConditionalDimension,
    foundational: CanonicalMismatchBasis,
    comparison_count: u32,
) -> WorthQueryOperationConditionalComparisonOutcome {
    WorthQueryOperationConditionalComparisonOutcome::Mismatched(
        WorthQueryOperationConditionalComparisonMismatch {
            dimension,
            foundational,
            comparison_count,
        },
    )
}

fn unsupported(
    dimension: WorthQueryOperationConditionalDimension,
    foundational: CanonicalMismatchBasis,
    comparison_count: u32,
) -> WorthQueryOperationConditionalComparisonOutcome {
    WorthQueryOperationConditionalComparisonOutcome::Unsupported(
        WorthQueryOperationConditionalComparisonUnsupported {
            dimension,
            foundational,
            comparison_count,
        },
    )
}
