use super::super::read_repository_document;

mod manifest_capacity_transition;

const FINGERPRINT: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                           durability/mutation/request_fingerprint.rs";
const PREPARATION_FACADE: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                                  record_serving/publication/director/submission.rs";
const PREPARATION_OWNER: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                                 record_serving/publication/director/durable_preparation.rs";
const REGISTRY: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                         durability/mutation/idempotency/registry/admission.rs";
const POLICY_BASIS: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                            record_serving/work_semantics/durability/policy_binding_basis.rs";
const CANONICAL_RECORD: &str = "workspaces/worth-store/crates/worth-store-aspect-native/src/\
                               canonical_basis/physical_mutation_request.rs";
const CANONICAL_DECLARATIONS: &str =
    "workspaces/worth-store/crates/worth-store-aspect-native/src/canonical_basis.rs";
const CANONICAL_OWNERS: &str = "workspaces/worth-store/crates/worth-store-aspect-native/src/\
                               canonical_basis/canonical_basis_sources.rs";
const CANONICAL_CONSTRUCTION: &str =
    "workspaces/worth-store/crates/worth-store-aspect-native/src/canonical_basis/\
     canonical_basis_construction.rs";
const CANONICAL_DOMAINS: &str = "workspaces/worth-store/crates/worth-store-aspect-native/src/\
                                canonical_basis/canonical_basis_domains.rs";
const UI: &str = "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority_ui.rs";

#[test]
fn mutation_preparation_preserves_equivalence_authority_and_no_effect_ordering() {
    inspect(&sources()).unwrap();
}

#[test]
fn mutation_preparation_gate_rejects_allocation_policy_and_ordering_mutants() {
    let source = sources();

    let mut allocation_feedback = source.clone();
    allocation_feedback.fingerprint = allocation_feedback.fingerprint.replace(
        "pub security_bases: &'a [PhysicalMutationSecurityBasis],",
        "pub security_bases: &'a [PhysicalMutationSecurityBasis],\n    pub wal_allocation: u64,",
    );
    assert!(inspect(&allocation_feedback).is_err());

    let mut policy_output = source.clone();
    policy_output.policy_basis = policy_output.policy_basis.replace(
        "PhysicalSignalAspectRole::Dependency",
        "PhysicalSignalAspectRole::Output",
    );
    assert!(inspect(&policy_output).is_err());

    let mut eager_reservation = source.clone();
    eager_reservation.registry = eager_reservation.registry.replace(
        "if let Some(existing) = self.bindings.get(&key.identity()) {",
        "let mutation = reserve().map_err(PhysicalMutationIdempotencyRegistryAdmissionError::Reservation)?;\n\
         if let Some(existing) = self.bindings.get(&key.identity()) {",
    );
    assert!(inspect(&eager_reservation).is_err());

    let mut effectful_preparation = source;
    effectful_preparation.preparation = effectful_preparation.preparation.replace(
        "let payload = match canonical_payload(batch)",
        "let _ = director.publish(batch, placement, ManifestCapacityTransition::PreserveCurrent);\n\
         let payload = match canonical_payload(batch)",
    );
    assert!(inspect(&effectful_preparation).is_err());

    let mut uncertified_native_role = sources();
    uncertified_native_role.canonical_construction =
        uncertified_native_role.canonical_construction.replace(
            "StoreCanonicalBasisFieldRole::NativePhysicalMutationRequest",
            "StoreCanonicalBasisFieldRole::TerminalProjection",
        );
    assert!(inspect(&uncertified_native_role).is_err());
}

#[derive(Clone)]
struct PreparationSources {
    fingerprint: String,
    preparation_facade: String,
    preparation: String,
    registry: String,
    policy_basis: String,
    canonical_record: String,
    canonical_declarations: String,
    canonical_owners: String,
    canonical_construction: String,
    canonical_domains: String,
    ui: String,
}

fn sources() -> PreparationSources {
    PreparationSources {
        fingerprint: read_repository_document(FINGERPRINT).expect("read mutation fingerprint"),
        preparation_facade: read_repository_document(PREPARATION_FACADE)
            .expect("read mutation preparation facade"),
        preparation: read_repository_document(PREPARATION_OWNER)
            .expect("read mutation preparation owner"),
        registry: read_repository_document(REGISTRY).expect("read idempotency registry"),
        policy_basis: read_repository_document(POLICY_BASIS)
            .expect("read durability policy work basis"),
        canonical_record: read_repository_document(CANONICAL_RECORD)
            .expect("read canonical mutation record"),
        canonical_declarations: read_repository_document(CANONICAL_DECLARATIONS)
            .expect("read canonical declarations"),
        canonical_owners: read_repository_document(CANONICAL_OWNERS)
            .expect("read canonical ownership registry"),
        canonical_construction: read_repository_document(CANONICAL_CONSTRUCTION)
            .expect("read canonical construction"),
        canonical_domains: read_repository_document(CANONICAL_DOMAINS)
            .expect("read canonical domain registry"),
        ui: read_repository_document(UI).expect("read authority UI registry"),
    }
}

fn inspect(source: &PreparationSources) -> Result<(), &'static str> {
    inspect_fingerprint(&source.fingerprint)?;
    inspect_preparation(&source.preparation_facade, &source.preparation)?;
    inspect_registry(&source.registry)?;
    inspect_policy_basis(&source.policy_basis)?;
    inspect_canonical_basis(source)?;
    if !source
        .ui
        .contains("physical_mutation_preparation_authority_is_sealed.rs")
    {
        return Err(
            "mutation construction and allocation feedback compile attack is not registered",
        );
    }
    Ok(())
}

fn inspect_fingerprint(source: &str) -> Result<(), &'static str> {
    let input = between(
        source,
        "pub(in crate::physical_runtime) struct PhysicalMutationFingerprintInput<'a> {",
        "\n}",
    )
    .ok_or("fingerprint input is absent")?;
    for required in [
        "store:",
        "durability_policy:",
        "scope:",
        "payload:",
        "durability_request:",
        "operation_family:",
        "security_bases:",
    ] {
        if !input.contains(required) {
            return Err("fingerprint omitted an effect-relevant field");
        }
    }
    for forbidden in [
        "deadline",
        "lease",
        "runtime",
        "operation_identity",
        "group",
        "wal",
        "allocation",
        "queue",
        "schedule",
        "cancellation",
        "completion",
        "observation",
    ] {
        if input.to_ascii_lowercase().contains(forbidden) {
            return Err("attempt-local fact entered request equivalence");
        }
    }
    if !source.contains("store.physical.mutation.request-fingerprint.v1")
        || !source.contains("security_bases.sort_unstable();")
    {
        return Err("fingerprint version or canonical security ordering drifted");
    }
    Ok(())
}

fn inspect_preparation(facade: &str, owner: &str) -> Result<(), &'static str> {
    let facade_body = function_body(facade, "pub fn prepare_durable_append(")
        .ok_or("public durable preparation boundary is absent")?;
    let facade_body = compact(facade_body);
    if !facade_body.contains("director.prepare_durable_append(")
        || !facade_body.contains(
            "batch,placement,PhysicalManifestCapacityTransition::PreserveCurrent,request,",
        )
    {
        return Err("public durable preparation facade bypasses its semantic owner");
    }
    for required in [
        "batch.preflight(",
        "preflight_placement(",
        "prepare_canonical_payload(",
        "record_append_scope_identity(",
        "PhysicalMutationRequestFingerprint::derive(",
        "admit_unallocated_with(",
        "reserve_mutation_identity()",
        "PreparedPhysicalMutation::new(",
    ] {
        if !owner.contains(required) {
            return Err("durable preparation omitted a required admission stage");
        }
    }
    if owner.contains(".publish(")
        || owner.contains("execute_physical_work")
        || owner.contains("wal")
        || owner.contains("acknowledg")
    {
        return Err("Phase 2 preparation can begin a physical effect");
    }
    Ok(())
}

fn inspect_registry(source: &str) -> Result<(), &'static str> {
    let body = function_body(
        source,
        "pub(in crate::physical_runtime) fn admit_unallocated_with<E>(",
    )
    .ok_or("atomic idempotency admission is absent")?;
    let body = compact(body);
    let existing = body
        .find("self.bindings.get(&key.identity())")
        .ok_or("duplicate lookup is absent")?;
    let fresh_validation = body
        .find("self.validate_fresh_admission(&key)")
        .ok_or("fresh admission validation is absent")?;
    let reserve = body
        .find("reserve().map_err")
        .ok_or("fresh operation reservation is absent")?;
    if !(existing < fresh_validation && fresh_validation < reserve) {
        return Err("duplicate lookup, fresh validation, and reservation ordering drifted");
    }
    let validation = compact(
        function_body(source, "fn validate_fresh_admission<")
            .ok_or("fresh admission validation owner is absent")?,
    );
    let expiry = validation
        .find("key.lease().is_expired_at(self.generation)")
        .ok_or("expiry check is absent")?;
    let live = validation
        .find("self.bindings.len()>=self.live_limit.get().get()asusize")
        .ok_or("live binding bound is absent")?;
    let pending = validation
        .find("self.pending_binding_count()>=self.pending_limit.get().get()asusize")
        .ok_or("pending bound is absent")?;
    if !(expiry < live && live < pending) {
        return Err("expiry, live binding, and pending bounds changed order");
    }
    let classification = compact(
        function_body(source, "fn classify_existing_binding<")
            .ok_or("existing binding classification owner is absent")?,
    );
    for required in [
        "PhysicalMutationIdempotencyBindingState::Unsealed(existing)",
        "PhysicalMutationIdempotencyBindingState::GroupSealed{basis:existing,..}",
        "PhysicalMutationIdempotencyBindingState::RebuiltUnresolved{basis:existing,..}",
        "PhysicalMutationIdempotencyBindingState::WalBound{basis:existing,..}",
        "PhysicalMutationIdempotencyRegistryAdmission::DuplicateUnresolved",
        "PhysicalMutationIdempotencyBindingState::Terminal{fate,..}",
        "fate.duplicate_observation(fingerprint)",
        "PhysicalMutationIdempotencyRegistryAdmission::ProvenNoEffect",
    ] {
        if !classification.contains(required) {
            return Err("idempotency replay lost an unresolved or terminal state");
        }
    }
    let pending_count = compact(
        function_body(source, "fn pending_binding_count(").ok_or("pending count is absent")?,
    );
    if !pending_count.contains("PhysicalMutationIdempotencyBindingState::Unsealed(_)")
        || !pending_count.contains("PhysicalMutationIdempotencyBindingState::GroupSealed{..}")
        || !pending_count.contains("PhysicalMutationIdempotencyBindingState::RebuiltUnresolved{..}")
        || !pending_count.contains("PhysicalMutationIdempotencyBindingState::WalBound{..}")
        || pending_count.contains("PhysicalMutationIdempotencyBindingState::Terminal{..}")
    {
        return Err("terminal no-effect facts entered unresolved-capacity accounting");
    }
    Ok(())
}

fn compact(source: &str) -> String {
    source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}

fn inspect_policy_basis(source: &str) -> Result<(), &'static str> {
    if !source.contains("PhysicalSignalAspectRole::Dependency")
        || source.contains("admit_mutation_mask")
    {
        return Err("Foundational durability basis became output or mutation authority");
    }
    for family in [
        "WalAppend",
        "DurabilityBarrier",
        "CheckpointCapture",
        "RootPublication",
    ] {
        if !source.contains(&format!("PhysicalWorkSignalFamily::{family}")) {
            return Err("durability policy basis lost an exact Signal family");
        }
    }
    Ok(())
}

fn inspect_canonical_basis(source: &PreparationSources) -> Result<(), &'static str> {
    if !source
        .canonical_record
        .contains("StorePhysicalMutationRequestCanonicalFields")
        || !source
            .canonical_record
            .contains("store.physical.mutation.request-fingerprint.v1")
    {
        return Err("aspect-native mutation canonical record drifted");
    }
    for declaration in [
        "PhysicalMutationRequestFingerprint",
        "StorePhysicalMutationRequest",
        "NativePhysicalMutationRequest",
        "PhysicalMutation",
    ] {
        if !source.canonical_declarations.contains(declaration) {
            return Err("aspect-native mutation declaration drifted");
        }
    }
    for owner_fact in [
        "const PHYSICAL_MUTATION_REQUEST:",
        "StoreCanonicalBasisSourceKind::StorePhysicalMutationRequest",
        "StoreCanonicalBasisFamily::PhysicalMutationRequestFingerprint",
        "\"physical mutation request equivalence\"",
        "StoreCanonicalBasisLane::PhysicalMutation",
    ] {
        if !source.canonical_owners.contains(owner_fact) {
            return Err("aspect-native mutation ownership drifted");
        }
    }
    let body = function_body(
        &source.canonical_construction,
        "fn prepare_physical_mutation_request(",
    )
    .ok_or("aspect-native mutation construction is absent")?;
    for construction_fact in [
        "certify_canonical_basis_source(",
        "StoreCanonicalBasisSourceKind::StorePhysicalMutationRequest",
        "certify_canonical_basis_field_role(",
        "StoreCanonicalBasisFieldRole::NativePhysicalMutationRequest",
        "source.into_canonical_entries()",
    ] {
        if !body.contains(construction_fact) {
            return Err("aspect-native mutation construction certification drifted");
        }
    }
    if !source
        .canonical_domains
        .contains("StoreCanonicalBasisFamily::PhysicalMutationRequestFingerprint")
        || !source
            .canonical_domains
            .contains("store.physical.mutation.request-fingerprint.v1")
    {
        return Err("aspect-native mutation domain registry drifted");
    }
    Ok(())
}

fn between<'a>(source: &'a str, start: &str, end: &str) -> Option<&'a str> {
    let (_, tail) = source.split_once(start)?;
    tail.split_once(end).map(|(body, _)| body)
}

fn function_body<'a>(source: &'a str, signature: &str) -> Option<&'a str> {
    let start = source.find(signature)?;
    let open = source[start..].find('{')? + start;
    let mut depth = 0_u32;
    for (offset, character) in source[open..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&source[open + 1..open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}
