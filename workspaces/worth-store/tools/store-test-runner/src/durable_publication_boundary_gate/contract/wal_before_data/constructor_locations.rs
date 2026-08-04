use std::collections::BTreeMap;
use std::path::Path;

use super::super::super::repository_root;

const RUNTIME_ROOT: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime";
const EXPECTED: &[(&str, &str)] = &[
    (
        "WalDurablePhysicalMutation::new(",
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/grouping/member_settlement.rs",
    ),
    (
        "DataDispatchedPhysicalMutation::new(",
            "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/durable_data/effect_progression.rs",
    ),
    (
        "DataSettledPhysicalMutation::new(",
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/data/writeback_join.rs",
    ),
    (
        "CompletionBoundPhysicalWalBarrierSettlement(PhysicalWalBarrierSettlement {",
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/grouping/member_settlement.rs",
    ),
    (
        "CompletionBoundPhysicalWalGroupBarrierSettlement(Self",
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/grouping/wal_barrier/settlement.rs",
    ),
    (
        "CompletionBoundPhysicalDataSettlement(dispatched)",
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/data/writeback_join.rs",
    ),
];

#[test]
fn stronger_wal_and_data_states_have_one_exact_production_constructor_location() {
    let occurrences = production_occurrences().expect("scan physical runtime constructors");
    validate(&occurrences).unwrap();
}

#[test]
fn constructor_location_gate_rejects_extra_missing_and_relocated_authority() {
    let occurrences = production_occurrences().expect("scan physical runtime constructors");

    let mut extra = occurrences.clone();
    extra
        .get_mut(EXPECTED[0].0)
        .unwrap()
        .push("controlled/competing_owner.rs".to_owned());
    assert!(validate(&extra).is_err());

    let mut missing = occurrences.clone();
    missing.get_mut(EXPECTED[1].0).unwrap().clear();
    assert!(validate(&missing).is_err());

    let mut relocated = occurrences;
    relocated.get_mut(EXPECTED[2].0).unwrap()[0] = "controlled/relocated_owner.rs".to_owned();
    assert!(validate(&relocated).is_err());
}

fn production_occurrences() -> Result<BTreeMap<&'static str, Vec<String>>, String> {
    let repository = repository_root();
    let mut occurrences = EXPECTED
        .iter()
        .map(|(token, _)| (*token, Vec::new()))
        .collect::<BTreeMap<_, _>>();
    let runtime = repository.join(RUNTIME_ROOT);
    visit(&repository, &runtime, &mut occurrences)?;
    Ok(occurrences)
}

fn visit(
    repository: &Path,
    directory: &Path,
    occurrences: &mut BTreeMap<&'static str, Vec<String>>,
) -> Result<(), String> {
    for entry in std::fs::read_dir(directory)
        .map_err(|error| format!("cannot inspect {}: {error}", directory.display()))?
    {
        let path = entry
            .map_err(|error| format!("cannot inspect {}: {error}", directory.display()))?
            .path();
        if path.is_dir() {
            visit(repository, &path, occurrences)?;
        } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
            let source = std::fs::read_to_string(&path)
                .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
            let relative = path
                .strip_prefix(repository)
                .expect("runtime source is under repository")
                .to_string_lossy()
                .replace('\\', "/");
            for (token, paths) in occurrences.iter_mut() {
                paths.extend(source.match_indices(token).map(|_| relative.clone()));
            }
        }
    }
    Ok(())
}

fn validate(occurrences: &BTreeMap<&str, Vec<String>>) -> Result<(), String> {
    for (token, expected_path) in EXPECTED {
        let actual = occurrences
            .get(token)
            .ok_or_else(|| format!("constructor token `{token}` was not scanned"))?;
        if actual.as_slice() != [*expected_path] {
            return Err(format!(
                "governed constructor `{token}` must exist once at `{expected_path}`; actual {actual:?}"
            ));
        }
    }
    Ok(())
}
