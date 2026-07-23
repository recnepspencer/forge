mod basis;
mod dimension;
mod output_basis;
mod value_basis;

#[cfg(test)]
mod tests;

use worth_foundational::facade::{
    compare_canonical_basis, prepare_canonical_comparison, CanonicalComparisonOutcome,
    CanonicalEquivalenceBasis, CanonicalEquivalentBasis, CanonicalMismatchBasis,
};

use self::basis::{declaration_bases, portable_conditional_rule_version};
pub use self::dimension::{
    WorthQueryPortableConditionalDependencyLocation, WorthQueryPortableConditionalDependencyPart,
    WorthQueryPortableConditionalDimension, WorthQueryPortableConditionalOutputPart,
};
use super::WorthQueryPortableConditionalNodeDeclaration;

/// Owner-native comparison of the complete portable meaning of two
/// conditional node declarations.
///
/// An equivalent result proves only portable semantic sameness. Runtime,
/// installation, generation, lowering, and lifecycle admission remain Query's
/// responsibility at the operational compatibility boundary.
pub fn compare_portable_conditional_node_declarations(
    left: &WorthQueryPortableConditionalNodeDeclaration,
    right: &WorthQueryPortableConditionalNodeDeclaration,
) -> WorthQueryPortableConditionalComparisonOutcome {
    let version = portable_conditional_rule_version();
    compare_with_versions(left, version.clone(), right, version)
}

/// Canonical owner-produced material for the complete portable conditional
/// meaning. This is a representation for enclosing semantic artifacts, not an
/// installation or execution authority.
pub fn portable_conditional_node_canonical_material(
    declaration: &WorthQueryPortableConditionalNodeDeclaration,
) -> String {
    let bases = declaration_bases(declaration, portable_conditional_rule_version());
    let mut material = String::from("worth-query-portable-conditional-material-v1;");
    for (index, basis) in bases.into_iter().enumerate() {
        let canonical = worth_foundational::facade::canonical_basis_sequence_material(
            basis.foundational.payload(),
        );
        material.push_str("dimension[");
        material.push_str(&index.to_string());
        material.push_str("]#");
        material.push_str(&canonical.len().to_string());
        material.push(':');
        material.push_str(&canonical);
        material.push(';');
    }
    material
}

#[derive(Debug)]
pub enum WorthQueryPortableConditionalComparisonOutcome {
    Equivalent(WorthQueryPortableConditionalComparisonEquivalent),
    Mismatched(WorthQueryPortableConditionalComparisonMismatch),
    Unsupported(WorthQueryPortableConditionalComparisonUnsupported),
}

#[derive(Debug)]
pub struct WorthQueryPortableConditionalComparisonEquivalent {
    foundational: Vec<CanonicalEquivalentBasis>,
}

impl WorthQueryPortableConditionalComparisonEquivalent {
    pub fn foundational_bases(&self) -> &[CanonicalEquivalentBasis] {
        &self.foundational
    }

    pub fn comparison_count(&self) -> u32 {
        self.foundational.len() as u32
    }
}

#[derive(Debug)]
pub struct WorthQueryPortableConditionalComparisonMismatch {
    dimension: WorthQueryPortableConditionalDimension,
    foundational: CanonicalMismatchBasis,
    comparison_count: u32,
}

impl WorthQueryPortableConditionalComparisonMismatch {
    pub fn dimension(&self) -> &WorthQueryPortableConditionalDimension {
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
pub struct WorthQueryPortableConditionalComparisonUnsupported {
    dimension: WorthQueryPortableConditionalDimension,
    foundational: CanonicalMismatchBasis,
    comparison_count: u32,
}

impl WorthQueryPortableConditionalComparisonUnsupported {
    pub fn dimension(&self) -> &WorthQueryPortableConditionalDimension {
        &self.dimension
    }

    pub fn foundational_basis(&self) -> &CanonicalMismatchBasis {
        &self.foundational
    }

    pub fn comparison_count(&self) -> u32 {
        self.comparison_count
    }
}

fn compare_with_versions(
    left: &WorthQueryPortableConditionalNodeDeclaration,
    left_version: worth_foundational::facade::CanonicalizationRuleVersion,
    right: &WorthQueryPortableConditionalNodeDeclaration,
    right_version: worth_foundational::facade::CanonicalizationRuleVersion,
) -> WorthQueryPortableConditionalComparisonOutcome {
    let left_bases = declaration_bases(left, left_version);
    let right_bases = declaration_bases(right, right_version);
    let expected_comparisons = left_bases.len();
    let expected_right_comparisons = right_bases.len();
    let mut equivalent = Vec::with_capacity(left_bases.len().min(right_bases.len()));

    for (index, (left, right)) in left_bases.into_iter().zip(right_bases).enumerate() {
        assert_eq!(
            left.dimension, right.dimension,
            "conditional basis topology must diverge only after an earlier typed mismatch"
        );
        let dimension = left.dimension;
        let ready = prepare_canonical_comparison(
            CanonicalEquivalenceBasis::ExactCanonicalBasis,
            left.foundational,
            right.foundational,
        )
        .into_result()
        .expect("canonical comparison preparation is infallible");
        match compare_canonical_basis(&ready) {
            CanonicalComparisonOutcome::Equivalent(basis) => equivalent.push(basis),
            CanonicalComparisonOutcome::Mismatched(foundational) => {
                return WorthQueryPortableConditionalComparisonOutcome::Mismatched(
                    WorthQueryPortableConditionalComparisonMismatch {
                        dimension,
                        foundational,
                        comparison_count: index as u32 + 1,
                    },
                );
            }
            CanonicalComparisonOutcome::Unsupported(foundational) => {
                return WorthQueryPortableConditionalComparisonOutcome::Unsupported(
                    WorthQueryPortableConditionalComparisonUnsupported {
                        dimension,
                        foundational,
                        comparison_count: index as u32 + 1,
                    },
                );
            }
        }
    }

    assert_eq!(expected_comparisons, expected_right_comparisons);
    assert_eq!(equivalent.len(), expected_comparisons);
    WorthQueryPortableConditionalComparisonOutcome::Equivalent(
        WorthQueryPortableConditionalComparisonEquivalent {
            foundational: equivalent,
        },
    )
}
