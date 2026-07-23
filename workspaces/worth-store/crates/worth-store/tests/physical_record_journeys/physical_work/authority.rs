use tempfile::tempdir;
use worth_foundational::{
    aspects, AbsenceLaw, AspectEquivalenceBasis, AspectEvolutionPolicy, AspectMask, AspectValue,
    ProjectionMask, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_store::aspect_native::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectContractAdmission,
    StoreAspectIdentity, StoreAspectNativeDenial, StoreAspectPatchAuthorityInput,
    StoreAspectPatchBoundaryFact,
};
use worth_store::physical_runtime::{
    PhysicalReadWorkRequest, PhysicalWorkProfileDeclaration, PhysicalWorkSemanticBasis,
    PhysicalWorkSemanticBasisDenial, PhysicalWorkSubmissionDenial,
};

use super::fixture::{
    admitted_contract, alternative_physical_witness, security_scope, security_scope_from_authority,
    serving_from_initialization_with_work_profile, validated_value,
};

#[test]
fn semantic_basis_rejects_same_key_with_a_different_contract_revision() {
    let (contract_v1, identity, _, witness) = admitted_contract(1);
    let (_, _, admission_v2, _) = admitted_contract(2);
    let fact = projection_fact(&contract_v1, identity, witness, "revision-one");
    assert_eq!(
        PhysicalWorkSemanticBasis::projection(fact, admission_v2),
        Err(PhysicalWorkSemanticBasisDenial::ContractRevisionMismatch)
    );
}

#[test]
fn semantic_basis_rejects_same_identity_and_revision_with_a_different_shape() {
    let (string_contract, identity, _, witness) = admitted_contract(1);
    let integer_contract = aspects()
        .contract()
        .for_key(string_contract.key().clone())
        .identified_by(string_contract.identity())
        .at_revision(string_contract.revision())
        .scalar(ScalarAspectType::UInt64);
    let integer_admission =
        StoreAspectContractAdmission::new(identity.clone(), integer_contract, witness).unwrap();
    let fact = projection_fact(&string_contract, identity, witness, "string-shape");
    assert_eq!(
        PhysicalWorkSemanticBasis::projection(fact, integer_admission),
        Err(PhysicalWorkSemanticBasisDenial::ContractCanonicalMismatch)
    );
}

#[test]
fn request_security_scope_must_share_the_semantic_physical_witness() {
    let (contract, identity, admission, witness) = admitted_contract(1);
    let fact = projection_fact(&contract, identity, witness, "security-scope");
    let basis = PhysicalWorkSemanticBasis::projection(fact, admission).unwrap();
    assert_eq!(
        PhysicalReadWorkRequest::new(
            request_scope(),
            basis,
            security_scope(alternative_physical_witness()),
        ),
        Err(PhysicalWorkSubmissionDenial::SecurityScopeWitnessMismatch)
    );
}

#[test]
fn profile_security_authority_rejects_a_foreign_admitted_receipt_before_retention() {
    let root = tempdir().unwrap();
    let (contract, identity, admission, witness) = admitted_contract(1);
    let fact = projection_fact(&contract, identity, witness, "foreign-security-authority");
    let basis = PhysicalWorkSemanticBasis::projection(fact, admission.clone()).unwrap();
    let request = PhysicalReadWorkRequest::new(
        request_scope(),
        basis,
        security_scope_from_authority("store.physical.foreign_authority", witness),
    )
    .unwrap();
    let serving = serving_from_initialization_with_work_profile(
        root.path(),
        PhysicalWorkProfileDeclaration::new(security_scope(witness), [admission]).unwrap(),
    );

    assert!(matches!(
        serving
            .physical_read_submission()
            .submit(request)
            .into_raw(),
        TransitionOutcome::Denied(PhysicalWorkSubmissionDenial::SecurityAuthorityMismatch)
    ));
    assert_eq!(serving.close().work().declared(), 0);
}

#[test]
fn installed_profile_rejects_an_exact_contract_from_another_physical_boundary() {
    let root = tempdir().unwrap();
    let (contract, identity, installed, witness) = admitted_contract(1);
    let alternate_witness = alternative_physical_witness();
    let alternate =
        StoreAspectContractAdmission::new(identity.clone(), contract.clone(), alternate_witness)
            .unwrap();
    let fact = projection_fact(&contract, identity, alternate_witness, "other-boundary");
    let basis = PhysicalWorkSemanticBasis::projection(fact, alternate).unwrap();
    let request = PhysicalReadWorkRequest::new(
        request_scope(),
        basis,
        security_scope(alternate_witness),
    )
    .unwrap();
    let serving = serving_from_initialization_with_work_profile(
        root.path(),
        PhysicalWorkProfileDeclaration::new(security_scope(witness), [installed]).unwrap(),
    );
    assert!(matches!(
        serving
            .physical_read_submission()
            .submit(request)
            .into_raw(),
        TransitionOutcome::Denied(PhysicalWorkSubmissionDenial::SemanticContractNotInstalled)
    ));
    assert_eq!(serving.close().work().declared(), 0);
}

#[test]
fn semantic_basis_rejects_a_patch_outside_the_admitted_mutation_mask() {
    let (_, _, _, witness) = admitted_contract(1);
    let vocabulary = aspects().vocabulary();
    let key = vocabulary.key("store.physical.work.masked").unwrap();
    let shape = aspects()
        .struct_fields()
        .optional("left", ScalarAspectType::String)
        .optional("right", ScalarAspectType::String)
        .finish()
        .unwrap();
    let contract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(vocabulary.identity(73))
        .at_revision(vocabulary.revision(1))
        .struct_with(
            shape,
            aspects().mask_contract().struct_fields(),
            AbsenceLaw::Optional,
            AspectEquivalenceBasis::DeclaredStructFields,
            AspectEvolutionPolicy::Frozen,
        );
    let admitted_mask = aspects().mutation_mask().fields(["left"]).unwrap();
    let patch_mask = aspects().mutation_mask().fields(["right"]).unwrap();
    let admission = StoreAspectContractAdmission::new(
        StoreAspectIdentity::from_aspect_key(key.clone()),
        contract.clone(),
        witness,
    )
    .unwrap()
    .with_mutation_mask(admitted_mask)
    .unwrap();
    let patch = match aspects()
        .patch()
        .field_level(&contract, &patch_mask)
        .set_field(
            vocabulary.field_key("right").unwrap(),
            AspectValue::String("outside-mask".into()),
        )
        .finish()
    {
        TransitionOutcome::Success(patch) => patch,
        outcome => panic!("patch should construct: {outcome:?}"),
    };
    let fact = StoreAspectPatchBoundaryFact::from_authoritative_patch(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectPatchAuthorityInput::new(patch, witness),
    )
    .unwrap();
    assert_eq!(
        PhysicalWorkSemanticBasis::mutation(fact, admission),
        Err(PhysicalWorkSemanticBasisDenial::MutationMaskMismatch)
    );
}

#[test]
fn semantic_basis_rejects_mutation_without_an_admitted_mutation_mask() {
    let (contract, identity, _, witness) = admitted_contract(1);
    let admission = StoreAspectContractAdmission::new(identity.clone(), contract.clone(), witness)
        .unwrap()
        .admit_projection_mask(AspectMask::<ProjectionMask>::whole_aspect())
        .unwrap();
    let patch = match aspects()
        .patch()
        .whole_aspect()
        .set(validated_value(&contract, "mask-required"))
        .finish()
    {
        TransitionOutcome::Success(patch) => patch,
        outcome => panic!("patch should construct: {outcome:?}"),
    };
    let fact = StoreAspectPatchBoundaryFact::from_authoritative_patch(
        identity,
        StoreAspectPatchAuthorityInput::new(patch, witness),
    )
    .unwrap();

    assert_eq!(
        PhysicalWorkSemanticBasis::mutation(fact, admission),
        Err(PhysicalWorkSemanticBasisDenial::MutationMaskMismatch)
    );
}

#[test]
fn contract_admission_rejects_structurally_typed_but_contract_illegal_masks() {
    let (contract, identity, _, witness) = admitted_contract(1);
    let mutation = aspects()
        .mutation_mask()
        .fields(["not-a-scalar-field"])
        .unwrap();
    let diagnostic = aspects()
        .diagnostic_mask()
        .fields(["not-a-scalar-field"])
        .unwrap();
    let projection = aspects()
        .projection_mask()
        .fields(["not-a-scalar-field"])
        .unwrap();

    assert_eq!(
        StoreAspectContractAdmission::new(identity.clone(), contract.clone(), witness)
            .unwrap()
            .admit_projection_mask(projection),
        Err(StoreAspectNativeDenial::ProjectionMaskNotAdmitted)
    );

    assert_eq!(
        StoreAspectContractAdmission::new(identity.clone(), contract.clone(), witness)
            .unwrap()
            .admit_mutation_mask(mutation),
        Err(StoreAspectNativeDenial::MutationMaskNotAdmitted)
    );
    assert_eq!(
        StoreAspectContractAdmission::new(identity, contract, witness)
            .unwrap()
            .admit_diagnostic_mask(diagnostic),
        Err(StoreAspectNativeDenial::DiagnosticMaskNotAdmitted)
    );
}

#[test]
fn installed_profile_rejects_a_different_valid_mutation_mask() {
    let root = tempdir().unwrap();
    let (_, _, _, witness) = admitted_contract(1);
    let vocabulary = aspects().vocabulary();
    let key = vocabulary.key("store.physical.work.mask-binding").unwrap();
    let shape = aspects()
        .struct_fields()
        .optional("left", ScalarAspectType::String)
        .optional("right", ScalarAspectType::String)
        .finish()
        .unwrap();
    let contract = aspects()
        .contract()
        .for_key(key.clone())
        .identified_by(vocabulary.identity(74))
        .at_revision(vocabulary.revision(1))
        .struct_with(
            shape,
            aspects().mask_contract().struct_fields(),
            AbsenceLaw::Optional,
            AspectEquivalenceBasis::DeclaredStructFields,
            AspectEvolutionPolicy::Frozen,
        );
    let installed = StoreAspectContractAdmission::new(
        StoreAspectIdentity::from_aspect_key(key.clone()),
        contract.clone(),
        witness,
    )
    .unwrap()
    .admit_projection_mask(AspectMask::<ProjectionMask>::whole_aspect())
    .unwrap()
    .with_mutation_mask(aspects().mutation_mask().fields(["left"]).unwrap())
    .unwrap();
    let submitted = StoreAspectContractAdmission::new(
        StoreAspectIdentity::from_aspect_key(key.clone()),
        contract.clone(),
        witness,
    )
    .unwrap()
    .with_mutation_mask(aspects().mutation_mask().fields(["right"]).unwrap())
    .unwrap();
    let patch_mask = aspects().mutation_mask().fields(["right"]).unwrap();
    let patch = match aspects()
        .patch()
        .field_level(&contract, &patch_mask)
        .set_field(
            vocabulary.field_key("right").unwrap(),
            AspectValue::String("right-only".into()),
        )
        .finish()
    {
        TransitionOutcome::Success(patch) => patch,
        outcome => panic!("patch should construct: {outcome:?}"),
    };
    let fact = StoreAspectPatchBoundaryFact::from_authoritative_patch(
        StoreAspectIdentity::from_aspect_key(key),
        StoreAspectPatchAuthorityInput::new(patch, witness),
    )
    .unwrap();
    let basis = PhysicalWorkSemanticBasis::mutation(fact, submitted).unwrap();
    let request = worth_store::physical_runtime::PhysicalMutationWorkRequest::exact_write(
        request_scope(),
        basis,
        security_scope(witness),
        worth_store_physical_backend::ArtifactRangeWriteDurabilityRequirement::BufferedWrite,
    )
    .unwrap();
    let serving = serving_from_initialization_with_work_profile(
        root.path(),
        PhysicalWorkProfileDeclaration::new(security_scope(witness), [installed]).unwrap(),
    );
    assert!(matches!(
        serving
            .physical_mutation_submission()
            .submit(request)
            .into_raw(),
        TransitionOutcome::Denied(PhysicalWorkSubmissionDenial::SemanticContractNotInstalled)
    ));
    assert_eq!(serving.close().work().declared(), 0);
}

fn projection_fact(
    contract: &worth_foundational::AspectContract,
    identity: worth_store::aspect_native::StoreAspectIdentity,
    witness: worth_store::aspect_native::StorePhysicalBoundaryWitness,
    value: &str,
) -> StoreAspectBoundaryFact {
    let state = match aspects()
        .authoritative_state()
        .admit([validated_value(contract, value)])
    {
        TransitionOutcome::Success(state) => state,
        outcome => panic!("state should admit: {outcome:?}"),
    };
    StoreAspectBoundaryFact::from_admitted_state(
        identity,
        StoreAspectAuthorityInput::new(state, witness),
    )
    .unwrap()
}

fn request_scope() -> worth_store::physical_runtime::PhysicalWorkScope {
    worth_store::physical_runtime::PhysicalWorkScope::one(
        worth_store_physical_format::RecordFrameCoordinate::new(
            worth_store_physical_format::RecordArtifactFile::BootstrapCatalog,
            0,
            8,
        )
        .unwrap(),
    )
}
