use std::collections::HashMap;

use super::oracle::{adjudicate, removal_expectation, OracleExpectation, OracleRect};

mod attribution;
mod production;

pub(super) use production::{produce_maximum_overlap, ProducedMaximumDelta, ProducedUnchanged};

pub(super) struct MountedPresentationWorld {
    identity: String,
    version: u16,
    baseline: Box<[OracleRect]>,
    authored_instances: Box<[worth_ui_host_contract::UiMountedInstanceIdentity]>,
    semantic_surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    initial_frame: worth_ui_host_contract::UiMountedFrameIdentity,
}

impl MountedPresentationWorld {
    pub(super) fn maximum_overlap(
        transcript: &worth_ui_host_headless::UiHeadlessMountedFrameTranscript,
        authored_instances: Box<[worth_ui_host_contract::UiMountedInstanceIdentity]>,
        semantic_surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    ) -> Self {
        let manifest = include_str!("control_points.toml")
            .parse::<toml::Value>()
            .expect("versioned control-point manifest");
        let identity = text(&manifest, "world_identity").to_owned();
        let version = integer(&manifest, "world_version") as u16;
        let maximum = integer(&manifest, "maximum_rectangles") as usize;
        let controls = manifest["filled_rect"]
            .as_array()
            .expect("filled rectangle controls");
        let baseline = (0..maximum)
            .map(|identity| expected_rect(identity, controls))
            .collect::<Vec<_>>();
        assert_exact_rows(transcript, &baseline);
        assert_exact_order(transcript, &baseline);
        attribution::assert_initial_attribution(transcript, &authored_instances, semantic_surface);
        Self {
            identity,
            version,
            baseline: baseline.into_boxed_slice(),
            authored_instances,
            semantic_surface,
            initial_frame: transcript.frame(),
        }
    }

    pub(super) fn identity(&self) -> &str {
        &self.identity
    }

    pub(super) const fn version(&self) -> u16 {
        self.version
    }

    pub(super) fn baseline(&self) -> &[OracleRect] {
        &self.baseline
    }

    pub(super) fn assert_removal_delta(&self, delta: &ProducedMaximumDelta) {
        let count = delta.changed_rows;
        let expected_successor = &self.baseline[count..];
        let expected = removal_expectation(&self.baseline, count);
        let candidate = removal_candidate(delta);
        adjudicate(&expected, &candidate).unwrap_or_else(|denial| {
            let mismatch = expected
                .damage
                .iter()
                .zip(&candidate.damage)
                .position(|(expected, candidate)| expected != candidate)
                .map(|index| (index, expected.damage[index], candidate.damage[index]));
            panic!(
                "production removal {count} mismatched: {denial:?}; damage-lengths={:?}; first={mismatch:?}",
                (expected.damage.len(), candidate.damage.len())
            )
        });
        assert_exact_cost(delta, count);
        assert_exact_rows(&delta.transcript, expected_successor);
        assert_exact_order(&delta.transcript, expected_successor);
        assert_exact_damage(&delta.transcript, &self.baseline[..count]);
        attribution::assert_exact_attribution(
            &delta.transcript,
            &delta.authored_instances,
            self.semantic_surface,
        );
        assert_eq!(
            delta.authored_instances.as_ref(),
            &self.authored_instances[count..]
        );
        attribution::assert_exact_frame_attribution(
            &delta.transcript,
            &vec![self.initial_frame; delta.authored_instances.len()],
        );
    }

    pub(super) fn assert_unchanged(&self, unchanged: &ProducedUnchanged) {
        assert_eq!(unchanged.native_work_count, 0);
        assert_eq!(unchanged.cost.delta_rows_carried(), 0);
        assert_eq!(unchanged.cost.draw_list_mutations(), 0);
        assert_eq!(unchanged.cost.order_mutations(), 0);
        assert_eq!(unchanged.cost.logical_damage_regions(), 0);
    }

    pub(super) fn assert_restoration(&self, delta: &ProducedMaximumDelta) {
        assert_exact_rows(&delta.transcript, &self.baseline);
        assert_exact_order(&delta.transcript, &self.baseline);
        assert_eq!(delta.draw_mutations, delta.changed_rows as u64);
        assert_eq!(delta.order_mutations, delta.changed_rows as u64);
        assert_eq!(delta.damage_regions, delta.changed_rows as u64);
        assert_eq!(
            delta.delta_rows_carried,
            (delta.changed_rows * 3) as u64
        );
        attribution::assert_exact_attribution(
            &delta.transcript,
            &delta.authored_instances,
            self.semantic_surface,
        );
        assert!(delta.authored_instances[..delta.changed_rows]
            .iter()
            .zip(&self.authored_instances[..delta.changed_rows])
            .all(|(restored, original)| restored != original));
        assert_eq!(
            delta.authored_instances[delta.changed_rows..],
            self.authored_instances[delta.changed_rows..]
        );
        let expected_frames = (0..self.baseline.len())
            .map(|index| {
                if index < delta.changed_rows {
                    delta.transcript.frame()
                } else {
                    self.initial_frame
                }
            })
            .collect::<Vec<_>>();
        attribution::assert_exact_frame_attribution(&delta.transcript, &expected_frames);
    }
}

fn removal_candidate(delta: &ProducedMaximumDelta) -> OracleExpectation {
    let layers = delta
        .transcript
        .filled_rects()
        .iter()
        .map(|row| (row.command_identity(), row.layer_semantic_order()))
        .collect::<HashMap<_, _>>();
    let ordered_identities = delta
        .transcript
        .paint_order()
        .iter()
        .map(|identity| u16::try_from(layers[&identity.command()]).expect("profile order fits"))
        .collect();
    let damage = delta
        .transcript
        .logical_damage()
        .iter()
        .map(|region| box_values(region.bounds()))
        .collect::<Vec<_>>();
    OracleExpectation {
        owner_delta_count: usize::try_from(delta.draw_mutations).expect("mutation count fits"),
        vacated_damage_count: damage.len(),
        damage,
        ordered_identities,
    }
}

fn expected_rect(identity: usize, controls: &[toml::Value]) -> OracleRect {
    let order = u16::try_from(identity).expect("maximum rectangle identity fits the profile");
    let Some(expected) = controls.get(identity) else {
        return OracleRect {
            identity: order,
            bounds: [0, 0, 160, 96],
            rgba: [47, 129, 247, 255],
            order,
        };
    };
    OracleRect {
        identity: order,
        bounds: array4(expected, "x", "y", "width", "height"),
        rgba: rgba(expected),
        order: u16::try_from(integer(expected, "order")).expect("control order fits profile"),
    }
}

fn assert_exact_cost(delta: &ProducedMaximumDelta, count: usize) {
    let count = count as u64;
    assert_eq!(delta.draw_mutations, count);
    assert_eq!(delta.order_mutations, count);
    assert_eq!(delta.damage_regions, count);
    assert_eq!(delta.delta_rows_carried, count * 3);
}

fn assert_exact_rows(
    transcript: &worth_ui_host_headless::UiHeadlessMountedFrameTranscript,
    expected: &[OracleRect],
) {
    let mut observed = transcript.filled_rects().iter().collect::<Vec<_>>();
    observed.sort_by_key(|row| row.layer_semantic_order());
    assert_eq!(observed.len(), expected.len());
    for (row, expected) in observed.into_iter().zip(expected) {
        assert_eq!(box_values(row.bounds()), expected.bounds);
        assert_eq!(row.color().channels(), expected.rgba);
        assert_eq!(row.layer_semantic_order(), u32::from(expected.order));
    }
}

fn assert_exact_order(
    transcript: &worth_ui_host_headless::UiHeadlessMountedFrameTranscript,
    expected: &[OracleRect],
) {
    let layers = transcript
        .filled_rects()
        .iter()
        .map(|row| (row.command_identity(), row.layer_semantic_order()))
        .collect::<HashMap<_, _>>();
    let observed = transcript
        .paint_order()
        .iter()
        .map(|identity| layers[&identity.command()])
        .collect::<Vec<_>>();
    let expected = expected
        .iter()
        .map(|row| u32::from(row.order))
        .collect::<Vec<_>>();
    assert_eq!(observed, expected);
}

fn assert_exact_damage(
    transcript: &worth_ui_host_headless::UiHeadlessMountedFrameTranscript,
    removed: &[OracleRect],
) {
    let observed = transcript
        .logical_damage()
        .iter()
        .map(|damage| box_values(damage.bounds()))
        .collect::<Vec<_>>();
    let expected = removed.iter().map(|row| row.bounds).collect::<Vec<_>>();
    assert_eq!(observed, expected);
}

fn text<'a>(value: &'a toml::Value, key: &str) -> &'a str {
    value[key]
        .as_str()
        .unwrap_or_else(|| panic!("{key} string"))
}

fn integer(value: &toml::Value, key: &str) -> u64 {
    value[key]
        .as_integer()
        .and_then(|value| u64::try_from(value).ok())
        .unwrap_or_else(|| panic!("{key} unsigned integer"))
}

fn array4(value: &toml::Value, a: &str, b: &str, c: &str, d: &str) -> [u16; 4] {
    [a, b, c, d].map(|key| integer(value, key) as u16)
}

fn rgba(value: &toml::Value) -> [u8; 4] {
    let channels = value["rgba"].as_array().expect("rgba array");
    std::array::from_fn(|index| channels[index].as_integer().unwrap() as u8)
}

fn box_values(bounds: worth_ui_runtime::facade::mounted::UiMountedCanonicalBox) -> [u16; 4] {
    [bounds.x(), bounds.y(), bounds.width(), bounds.height()].map(|value| value as u16)
}
