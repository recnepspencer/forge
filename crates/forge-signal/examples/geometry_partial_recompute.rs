use forge_signal::facade::*;

const AIRFRAME: Aspect = Aspect::new(0);
const WING_SKIN: Aspect = Aspect::new(1);
const TAIL_SKIN: Aspect = Aspect::new(2);

#[derive(Default)]
struct GeometryState {
    airframe_version: u64,
    wing_skin_version: u64,
    tail_skin_version: u64,
    changed_partition: String,
}

fn main() -> Result<(), SignalError> {
    // A wing panel changes.
    // We want the wing mesh to update and the tail mesh to stay put.
    // This is the kind of job where region-aware invalidation stops being a cute detail.
    let mut graph = SignalGraph::new();
    let airframe = graph.node().partitioned_output().build();
    let wing_skin = graph.node().on_demand().build();
    let tail_skin = graph.node().on_demand().build();

    graph.set_dependencies(
        wing_skin,
        [DependencyEdge::with_partition_scope(
            airframe,
            AIRFRAME,
            PartitionSubscription::whole_partition("wing"),
        )],
    )?;
    graph.set_dependencies(
        tail_skin,
        [DependencyEdge::with_partition_scope(
            airframe,
            AIRFRAME,
            PartitionSubscription::whole_partition("tail"),
        )],
    )?;

    let mut runtime = SignalRuntime::build_for::<GeometryState>(graph);
    let mut state = GeometryState {
        airframe_version: 1,
        wing_skin_version: 100,
        tail_skin_version: 200,
        changed_partition: String::new(),
    };

    let evaluate = |view: &mut EvaluationContext<'_, GeometryState>| {
        let result = if view.node() == airframe {
            let mut result = NodeEvaluationResult::from_version(AspectVersion::from_updates([(
                AIRFRAME,
                view.domain().airframe_version,
            )]));
            if !view.domain().changed_partition.is_empty() {
                result = result.with_changed_region(ChangedRegion::new(
                    view.domain().changed_partition.as_str(),
                ));
            }
            view.finish(result)
        } else if view.node() == wing_skin {
            let _airframe = view.read_aspect_version(airframe, AIRFRAME)?;
            view.finish(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(WING_SKIN, view.domain().wing_skin_version)]),
            ))
        } else {
            let _airframe = view.read_aspect_version(airframe, AIRFRAME)?;
            view.finish(NodeEvaluationResult::from_version(
                AspectVersion::from_updates([(TAIL_SKIN, view.domain().tail_skin_version)]),
            ))
        };
        Ok::<_, SignalError>(result)
    };

    runtime.transaction(&mut state, |tx| {
        tx.read_many(&[airframe, wing_skin, tail_skin], &evaluate)?;
        Ok(())
    })?;

    state.airframe_version += 1;
    state.wing_skin_version += 1;
    state.changed_partition = "wing".to_string();

    runtime.transaction(&mut state, |tx| {
        tx.mark_changed_with_regions(airframe, AIRFRAME, &[ChangedRegion::new("wing")])?;
        tx.read_many(&[wing_skin, tail_skin], &evaluate)?;
        Ok(())
    })?;
    state.changed_partition.clear();

    let versions = runtime.read_many(&[wing_skin, tail_skin], &state, &evaluate)?;
    assert_eq!(versions[0].get(WING_SKIN), 101);
    assert_eq!(versions[1].get(TAIL_SKIN), 200);

    let explanation = runtime.diagnostics().explain(wing_skin)?;
    let rendered = format!("{explanation}");
    assert!(
        rendered.contains("wing"),
        "the explanation should keep the changed region visible"
    );

    Ok(())
}
