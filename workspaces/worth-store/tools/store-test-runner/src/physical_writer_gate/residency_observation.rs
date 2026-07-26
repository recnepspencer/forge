#[test]
fn store_observation_owns_its_public_identity_policy_and_snapshots() {
    let root = super::workspace_root().join(
        "crates/worth-store/src/physical_runtime/record_serving/residency/residency_observation",
    );
    let observation = std::fs::read_to_string(root.join("mod.rs"))
        .expect("read Store physical residency observation");
    let counters = std::fs::read_to_string(root.join("counters.rs"))
        .expect("read Store physical residency counter snapshot");
    let allocations = std::fs::read_to_string(root.join("allocations.rs"))
        .expect("read Store physical residency allocation snapshot");

    inspect_store_residency_observation(&observation, &counters, &allocations)
        .unwrap_or_else(|failure| panic!("{failure}"));
}

#[test]
fn store_observation_gate_rejects_lower_snapshot_exports() {
    let lower_return = r#"
        store: StableStoreIdentity,
        admitted_policy: AdmittedPhysicalRecordResidencyPolicy,
        pub const fn store_identity(self) -> StableStoreIdentity { self.store }
        pub const fn admitted_policy(self) -> AdmittedPhysicalRecordResidencyPolicy {
            self.admitted_policy
        }
        pub const fn counters(self) -> worth_store_buffer_pool::PhysicalResidencyCounters {
            todo!()
        }
        pub const fn allocations(
            self,
        ) -> PhysicalResidencyAllocationSnapshot {
            todo!()
        }
    "#;
    let denial = inspect_store_residency_observation(
        lower_return,
        PRIVATE_COUNTER_WRAPPER,
        PRIVATE_ALLOCATION_WRAPPERS,
    )
    .expect_err("a public lower-pool snapshot return must be rejected");
    assert!(denial.contains("lower pool snapshot"));

    let public_field = PRIVATE_COUNTER_WRAPPER.replacen("inner:", "pub inner:", 1);
    let denial = inspect_store_residency_observation(
        STORE_OWNED_RETURNS,
        &public_field,
        PRIVATE_ALLOCATION_WRAPPERS,
    )
    .expect_err("a public lower-pool snapshot field must be rejected");
    assert!(denial.contains("private Store wrapper"));
}

const STORE_OWNED_RETURNS: &str = r#"
    pub const fn counters(self) -> PhysicalResidencyCounterSnapshot { todo!() }
    pub const fn allocations(self) -> PhysicalResidencyAllocationSnapshot { todo!() }
"#;

const PRIVATE_COUNTER_WRAPPER: &str = r#"
    pub struct PhysicalResidencyCounterSnapshot {
        inner: worth_store_buffer_pool::PhysicalResidencyCounters,
    }
"#;

const PRIVATE_ALLOCATION_WRAPPERS: &str = r#"
    pub struct PhysicalResidencyAllocationSnapshot {
        inner: worth_store_buffer_pool::PhysicalResidencyAllocationEventSnapshot,
    }
    pub struct PhysicalResidencyAllocationEventSnapshot {
        inner: worth_store_buffer_pool::PhysicalResidencyAllocationEventCounters,
    }
"#;

fn inspect_store_residency_observation(
    observation: &str,
    counters: &str,
    allocations: &str,
) -> Result<(), String> {
    reject_public_lower_snapshots([observation, counters, allocations])?;
    require_snapshot_wrappers(counters, allocations)?;
    require_observation_basis(observation)
}

fn reject_public_lower_snapshots(sources: [&str; 3]) -> Result<(), String> {
    for signature in sources.into_iter().flat_map(public_function_signatures) {
        if [
            "worth_store_buffer_pool::PhysicalResidencyCounters",
            "worth_store_buffer_pool::PhysicalResidencyAllocationEventSnapshot",
            "worth_store_buffer_pool::PhysicalResidencyAllocationEventCounters",
        ]
        .into_iter()
        .any(|lower| signature.contains(lower))
        {
            return Err(
                "C.6 Store observation exposes a lower pool snapshot through a public signature"
                    .to_owned(),
            );
        }
    }
    Ok(())
}

fn require_snapshot_wrappers(counters: &str, allocations: &str) -> Result<(), String> {
    require_private_lower_field(
        counters,
        "PhysicalResidencyCounterSnapshot",
        "worth_store_buffer_pool::PhysicalResidencyCounters",
    )?;
    require_private_lower_field(
        allocations,
        "PhysicalResidencyAllocationSnapshot",
        "worth_store_buffer_pool::PhysicalResidencyAllocationEventSnapshot",
    )?;
    require_private_lower_field(
        allocations,
        "PhysicalResidencyAllocationEventSnapshot",
        "worth_store_buffer_pool::PhysicalResidencyAllocationEventCounters",
    )?;
    for (source, required) in [
        (counters, "pub struct PhysicalResidencyCounterSnapshot"),
        (
            allocations,
            "pub struct PhysicalResidencyAllocationSnapshot",
        ),
        (
            allocations,
            "pub struct PhysicalResidencyAllocationEventSnapshot",
        ),
    ] {
        if !source.contains(required) {
            return Err(format!(
                "C.6 Store observation wrapper is missing: {required}"
            ));
        }
    }
    Ok(())
}

fn require_observation_basis(observation: &str) -> Result<(), String> {
    for required in [
        "store: worth_store_physical_format::store_namespace::StableStoreIdentity,",
        "admitted_policy:",
        "pub const fn store_identity(",
        "pub const fn admitted_policy(",
        "pub const fn counters(self) -> PhysicalResidencyCounterSnapshot",
        "pub const fn allocations(self) -> PhysicalResidencyAllocationSnapshot",
    ] {
        if !observation.contains(required) {
            return Err(format!(
                "C.6 Store observation is missing Store-owned identity, policy, or snapshot contract: {required}"
            ));
        }
    }
    Ok(())
}

fn require_private_lower_field(source: &str, wrapper: &str, lower: &str) -> Result<(), String> {
    let contract = struct_contract(source, wrapper)?;
    let private_field = format!("inner: {lower},");
    if !contract.contains(&private_field)
        || ["pub inner:", "pub(crate) inner:", "pub(super) inner:"]
            .into_iter()
            .any(|visibility| contract.contains(visibility))
    {
        return Err(format!(
            "C.6 lower pool snapshot escaped its private Store wrapper: {wrapper}"
        ));
    }
    Ok(())
}

fn struct_contract<'source>(source: &'source str, name: &str) -> Result<&'source str, String> {
    let marker = format!("pub struct {name}");
    let start = source
        .find(&marker)
        .ok_or_else(|| format!("C.6 Store observation wrapper is missing: {name}"))?;
    let tail = &source[start..];
    let end = tail
        .find('}')
        .ok_or_else(|| format!("C.6 Store observation wrapper is malformed: {name}"))?
        + 1;
    Ok(&tail[..end])
}

fn public_function_signatures(source: &str) -> Vec<String> {
    let mut signatures = Vec::new();
    let mut current = None::<String>;
    for line in source.lines() {
        let trimmed = line.trim_start();
        if current.is_none() && is_externally_public_function(trimmed) {
            current = Some(String::new());
        }
        let Some(signature) = current.as_mut() else {
            continue;
        };
        signature.push_str(trimmed);
        signature.push(' ');
        if trimmed.contains('{') || trimmed.ends_with(';') {
            signatures.push(current.take().expect("public signature is present"));
        }
    }
    signatures
}

fn is_externally_public_function(line: &str) -> bool {
    [
        "pub fn ",
        "pub const fn ",
        "pub async fn ",
        "pub unsafe fn ",
    ]
    .into_iter()
    .any(|prefix| line.starts_with(prefix))
}
