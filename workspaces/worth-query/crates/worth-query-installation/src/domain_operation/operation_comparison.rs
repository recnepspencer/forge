mod comparison_width;
mod conditional_inventory;
mod dimension;
mod evidence;
mod structural;
mod workflow_structure;

#[cfg(test)]
mod tests;

use worth_foundational::facade::{
    compare_canonical_basis, compare_canonical_exports, prepare_canonical_comparison,
    CanonicalComparisonOutcome, CanonicalEquivalenceBasis, CanonicalExportComparisonOutcome,
};

pub use conditional_inventory::{
    compare_portable_operation_conditionals, WorthQueryOperationConditionalComparisonEquivalent,
    WorthQueryOperationConditionalComparisonMismatch,
    WorthQueryOperationConditionalComparisonOutcome,
    WorthQueryOperationConditionalComparisonUnsupported, WorthQueryOperationConditionalDimension,
};
pub use dimension::{
    WorthQueryPortableOperationCostDimension, WorthQueryPortableOperationDimension,
    WorthQueryPortableOperationSupportDimension,
};
pub use evidence::{
    WorthQueryPortableOperationComparisonEquivalent, WorthQueryPortableOperationComparisonMismatch,
    WorthQueryPortableOperationComparisonMismatchCategory,
    WorthQueryPortableOperationComparisonOutcome, WorthQueryPortableOperationComparisonUnsupported,
    WorthQueryPortableOperationComparisonWork,
};

use self::evidence::{EquivalentEvidence, MismatchEvidence};
use super::WorthQueryPortableDomainOperationDefinition;

/// Compares the complete owner-retained portable meaning of two operations.
///
/// Equivalent evidence is portable only. Query must still establish runtime,
/// installation, generation, basis, lifecycle, and relationship authority.
pub fn compare_portable_domain_operations(
    left: &WorthQueryPortableDomainOperationDefinition,
    right: &WorthQueryPortableDomainOperationDefinition,
) -> WorthQueryPortableOperationComparisonOutcome {
    let mut work = WorthQueryPortableOperationComparisonWork::default();

    if let Err(mismatch) = structural::compare_identity(left, right, &mut work) {
        return mismatch.into_outcome(work);
    }
    let native = match compare_native(left, right, &mut work) {
        Ok(evidence) => evidence,
        Err(mismatch) => return mismatch.into_outcome(work),
    };
    if let Err(mismatch) = structural::compare_before_conditionals(left, right, &mut work) {
        return mismatch.into_outcome(work);
    }

    work.submit_conditional_inventory(left, right);
    let conditional = match compare_portable_operation_conditionals(left, right) {
        WorthQueryOperationConditionalComparisonOutcome::Equivalent(evidence) => {
            work.record_conditional_comparisons(evidence.comparison_count());
            evidence
        }
        WorthQueryOperationConditionalComparisonOutcome::Mismatched(mismatch) => {
            work.record_conditional_comparisons(mismatch.comparison_count());
            return MismatchEvidence::foundational(
                WorthQueryPortableOperationDimension::Conditional(mismatch.dimension().clone()),
                mismatch.foundational_basis().clone(),
            )
            .into_outcome(work);
        }
        WorthQueryOperationConditionalComparisonOutcome::Unsupported(unsupported) => {
            work.record_conditional_comparisons(unsupported.comparison_count());
            return MismatchEvidence::unsupported(
                WorthQueryPortableOperationDimension::Conditional(unsupported.dimension().clone()),
                unsupported.foundational_basis().clone(),
            )
            .into_outcome(work);
        }
    };

    if let Err(mismatch) = structural::compare_after_conditionals(left, right, &mut work) {
        return mismatch.into_outcome(work);
    }

    WorthQueryPortableOperationComparisonOutcome::Equivalent(
        WorthQueryPortableOperationComparisonEquivalent::new(
            EquivalentEvidence::new(native.0, native.1, conditional),
            work,
        ),
    )
}

fn compare_native(
    left: &WorthQueryPortableDomainOperationDefinition,
    right: &WorthQueryPortableDomainOperationDefinition,
    work: &mut WorthQueryPortableOperationComparisonWork,
) -> Result<
    (
        worth_foundational::facade::CanonicalEquivalentBasis,
        worth_foundational::facade::CanonicalEquivalentBasis,
    ),
    MismatchEvidence,
> {
    let left = &left.semantics().native_projection;
    let right = &right.semantics().native_projection;
    let contract = compare_native_basis(
        left.canonical_contract_basis(),
        right.canonical_contract_basis(),
        WorthQueryPortableOperationDimension::NativeContract,
        work,
    )?;
    let mask = compare_native_basis(
        left.canonical_mask_basis(),
        right.canonical_mask_basis(),
        WorthQueryPortableOperationDimension::NativeProjectionMask,
        work,
    )?;
    work.inspect_canonical_export();
    match compare_canonical_exports(left.canonical_export(), right.canonical_export()) {
        CanonicalExportComparisonOutcome::Equivalent => Ok((contract, mask)),
        CanonicalExportComparisonOutcome::Mismatched(mismatch) => {
            Err(MismatchEvidence::foundational(
                WorthQueryPortableOperationDimension::NativeExport,
                mismatch,
            ))
        }
        CanonicalExportComparisonOutcome::ManifestMismatch(mismatch) => {
            Err(MismatchEvidence::export_manifest(
                WorthQueryPortableOperationDimension::NativeExport,
                mismatch,
            ))
        }
    }
}

fn compare_native_basis(
    left: &worth_foundational::facade::CanonicalBasisReadyArtifact,
    right: &worth_foundational::facade::CanonicalBasisReadyArtifact,
    dimension: WorthQueryPortableOperationDimension,
    work: &mut WorthQueryPortableOperationComparisonWork,
) -> Result<worth_foundational::facade::CanonicalEquivalentBasis, MismatchEvidence> {
    work.inspect_foundational_dimension();
    let ready = prepare_canonical_comparison(
        CanonicalEquivalenceBasis::ExactCanonicalBasis,
        left.clone(),
        right.clone(),
    )
    .into_result()
    .expect("retained native bases are comparison-ready");
    match compare_canonical_basis(&ready) {
        CanonicalComparisonOutcome::Equivalent(evidence) => Ok(evidence),
        CanonicalComparisonOutcome::Mismatched(mismatch) => {
            Err(MismatchEvidence::foundational(dimension, mismatch))
        }
        CanonicalComparisonOutcome::Unsupported(mismatch) => {
            Err(MismatchEvidence::unsupported(dimension, mismatch))
        }
    }
}
