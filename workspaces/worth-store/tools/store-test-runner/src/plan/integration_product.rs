use std::{collections::BTreeSet, path::Path};

use crate::{
    catalog::{TestCatalog, TestTarget},
    classification::CiTestLane,
};

use super::{offline_observer_build, TestExecutionUnit};

pub(super) fn integration_lane(
    selected: CiTestLane,
    shard: Option<(usize, usize)>,
    catalog: &TestCatalog,
    workspace_root: &Path,
) -> Result<Vec<TestExecutionUnit>, String> {
    let targets = catalog
        .targets()
        .iter()
        .filter(|target| target.lane == selected)
        .collect::<Vec<_>>();
    if targets.is_empty() {
        return Ok(Vec::new());
    }

    let target_names = exact_lane_target_names(selected, catalog, &targets)?;
    let filter = targets
        .iter()
        .map(|target| format!("binary_id(={}::{})", target.package, target.name))
        .collect::<Vec<_>>()
        .join(" + ");
    let features = required_features(&targets);
    let arguments = integration_arguments(target_names, filter, features, shard);
    let tests = TestExecutionUnit::cargo(
        format!("{selected}::nextest"),
        format!("Cargo targets classified as {selected}"),
        workspace_root,
        arguments,
        None,
    );
    if includes_physical_record_journeys(&targets) {
        Ok(vec![offline_observer_build(workspace_root), tests])
    } else {
        Ok(vec![tests])
    }
}

fn exact_lane_target_names<'a>(
    selected: CiTestLane,
    catalog: &'a TestCatalog,
    targets: &[&'a TestTarget],
) -> Result<BTreeSet<&'a str>, String> {
    let target_names = targets
        .iter()
        .map(|target| target.name.as_str())
        .collect::<BTreeSet<_>>();
    for name in &target_names {
        if let Some(conflict) = catalog
            .targets()
            .iter()
            .find(|target| target.name == *name && target.lane != selected)
        {
            return Err(format!(
                "Cargo target name `{name}` crosses the {selected} and {} lanes at {}::{}; exact workspace target selection would compile the wrong lane",
                conflict.lane, conflict.package, conflict.name
            ));
        }
    }
    Ok(target_names)
}

fn required_features(targets: &[&TestTarget]) -> Vec<String> {
    let mut features = targets
        .iter()
        .flat_map(|target| {
            target
                .required_features
                .iter()
                .map(|feature| format!("{}/{}", target.package, feature))
        })
        .collect::<Vec<_>>();
    features.sort();
    features.dedup();
    features
}

fn integration_arguments(
    target_names: BTreeSet<&str>,
    filter: String,
    features: Vec<String>,
    shard: Option<(usize, usize)>,
) -> Vec<String> {
    let mut arguments = vec![
        "nextest".into(),
        "run".into(),
        "--workspace".into(),
        "--no-fail-fast".into(),
        "--filterset".into(),
        filter,
    ];
    for target in target_names {
        arguments.extend(["--test".into(), target.into()]);
    }
    for feature in features {
        arguments.extend(["--features".into(), feature]);
    }
    if let Some((index, count)) = shard {
        arguments.extend([
            "--partition".into(),
            format!("hash:{}/{}", index + 1, count),
        ]);
    }
    arguments
}

fn includes_physical_record_journeys(targets: &[&TestTarget]) -> bool {
    targets
        .iter()
        .any(|target| target.package == "worth-store" && target.name == "physical_record_journeys")
}
