use super::super::read_repository_document;

const REQUEST: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                       record_serving/admission/request.rs";
const TRANSITION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                          record_serving/admission/transition.rs";
const CONSTRUCTION: &str =
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/construction.rs";
const PARTS: &str =
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/parts.rs";
const LIFECYCLE: &str =
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/lifecycle.rs";
const SERVING: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                       record_serving/lifecycle/serving_runtime.rs";
const OBSERVATION: &str =
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/observation/policy.rs";

#[test]
fn durability_owner_is_mandatory_and_exhaustive_across_the_runtime_lifecycle() {
    inspect(&sources()).unwrap();
}

#[test]
fn lifecycle_gate_rejects_optional_failure_omission_and_observation_mutants() {
    let source = sources();

    let mut optional = source.clone();
    optional.lifecycle = optional.lifecycle.replace(
        "durability_owner: crate::physical_runtime::durability::ReopenedPhysicalDurabilityRuntimeOwner",
        "durability_owner: Option<crate::physical_runtime::durability::ReopenedPhysicalDurabilityRuntimeOwner>",
    );
    assert!(inspect(&optional).is_err());

    let mut omitted_failure_owner = source.clone();
    omitted_failure_owner.construction = omitted_failure_owner.construction.replace(
        "durability: crate::physical_runtime::durability::PhysicalDurabilityRuntimeOwner",
        "omitted_durability_owner: ()",
    );
    assert!(inspect(&omitted_failure_owner).is_err());

    let mut bypassed_binding = source.clone();
    bypassed_binding.transition = bypassed_binding
        .transition
        .replace("bind_policy_to_runtime(", "bypass_policy_binding(");
    assert!(inspect(&bypassed_binding).is_err());

    let mut omitted_observation = source;
    omitted_observation.serving = omitted_observation
        .serving
        .replace("pub fn durability_observation(", "fn omitted_observation(");
    assert!(inspect(&omitted_observation).is_err());
}

#[derive(Clone)]
struct LifecycleSources {
    request: String,
    transition: String,
    construction: String,
    parts: String,
    lifecycle: String,
    serving: String,
    observation: String,
}

fn sources() -> LifecycleSources {
    LifecycleSources {
        request: read_repository_document(REQUEST).expect("read durability requests"),
        transition: read_repository_document(TRANSITION).expect("read serving transition"),
        construction: read_repository_document(CONSTRUCTION).expect("read instance construction"),
        parts: read_repository_document(PARTS).expect("read serving parts"),
        lifecycle: read_repository_document(LIFECYCLE).expect("read instance lifecycle"),
        serving: read_repository_document(SERVING).expect("read serving facade"),
        observation: read_repository_document(OBSERVATION).expect("read durability observation"),
    }
}

fn inspect(source: &LifecycleSources) -> Result<(), &'static str> {
    let request = compact(&source.request);
    if request.contains("Option<crate::physical_runtime::AdmittedPhysicalDurabilityPolicy>")
        || request.matches("durability:").count() < 4
    {
        return Err("initialization and open must require admitted durability");
    }
    if source.transition.matches("bind_policy_to_runtime(").count() != 2
        || !binding_precedes_residency(&source.transition, "fn initialize(")
        || !binding_precedes_residency(&source.transition, "fn open(")
    {
        return Err("both admission paths must bind durability before residency");
    }
    let construction = compact(&source.construction);
    if construction
        .matches("durability:crate::physical_runtime::durability::PhysicalDurabilityRuntimeOwner")
        .count()
        != 2
        || !construction.contains("drop(self.durability);")
    {
        return Err("foundation and construction failure must retain the owner");
    }
    if !compact(&source.parts).contains(
        "durability:crate::physical_runtime::durability::ReopenedPhysicalDurabilityRuntimeOwner",
    ) {
        return Err("serving parts must retain the owner");
    }
    let lifecycle = compact(&source.lifecycle);
    if lifecycle
        .contains("Option<crate::physical_runtime::durability::ReopenedPhysicalDurabilityRuntimeOwner>")
        || !lifecycle.contains(
            "durability_owner:crate::physical_runtime::durability::ReopenedPhysicalDurabilityRuntimeOwner",
        )
        || !lifecycle.contains("drop(self.durability_owner);")
    {
        return Err("shutdown must retain one mandatory owner through media release");
    }
    if !source.serving.contains("pub fn durability_observation(")
        || !source
            .observation
            .contains("pub struct PhysicalDurabilityObservation")
    {
        return Err("serving must expose observation without owner authority");
    }
    Ok(())
}

fn compact(source: &str) -> String {
    source
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect()
}

fn binding_precedes_residency(source: &str, start: &str) -> bool {
    let Some(body) = source.split_once(start).map(|(_, body)| body) else {
        return false;
    };
    let Some(binding) = body.find("bind_policy_to_runtime(") else {
        return false;
    };
    let Some(residency) = body.find("PhysicalResidencyOwner::admit(") else {
        return false;
    };
    binding < residency
}
