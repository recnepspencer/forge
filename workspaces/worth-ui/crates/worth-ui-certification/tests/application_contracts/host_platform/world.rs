use std::collections::HashMap;

use super::oracle::OracleRect;

mod production;

pub(super) use production::{produce_maximum_overlap, ProducedMaximumDelta};

pub(super) struct MountedPresentationWorld {
    identity: String,
    version: u16,
    baseline: Box<[OracleRect]>,
}

impl MountedPresentationWorld {
    pub(super) fn maximum_overlap(
        transcript: &worth_ui_host_headless::UiHeadlessMountedFrameTranscript,
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
        assert_eq!(transcript.filled_rects().len(), maximum);
        let mut observed = transcript.filled_rects().iter().collect::<Vec<_>>();
        observed.sort_by_key(|row| row.layer_semantic_order());
        let baseline = observed
            .into_iter()
            .enumerate()
            .map(|(identity, observed)| {
                let row = OracleRect {
                    identity: identity as u16,
                    bounds: box_values(observed.bounds()),
                    rgba: observed.color().channels(),
                    order: observed.layer_semantic_order() as u16,
                };
                if let Some(expected) = controls.get(identity) {
                    assert_eq!(row.bounds, array4(expected, "x", "y", "width", "height"));
                    assert_eq!(row.rgba, rgba(expected));
                    assert_eq!(u64::from(row.order), integer(expected, "order"));
                } else {
                    assert_eq!(row.bounds, [0, 0, 160, 96]);
                    assert_eq!(row.rgba, [47, 129, 247, 255]);
                    assert_eq!(row.order, identity as u16);
                }
                row
            })
            .collect::<Vec<_>>();
        assert_eq!(
            baseline.len(),
            maximum,
            "the entire maximum world is produced"
        );
        Self {
            identity,
            version,
            baseline: baseline.into_boxed_slice(),
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
        assert_exact_cost(delta, count, expected_successor.is_empty());
        assert_exact_rows(&delta.transcript, expected_successor);
        assert_exact_order(&delta.transcript, expected_successor);
        assert_exact_damage(
            &delta.transcript,
            &self.baseline[..count],
            expected_successor.first(),
        );
    }
}

fn assert_exact_cost(delta: &ProducedMaximumDelta, count: usize, successor_is_empty: bool) {
    let count = count as u64;
    let successor_anchor = u64::from(!successor_is_empty);
    let order = count + successor_anchor;
    let damage = count + successor_anchor;
    assert_eq!(delta.draw_mutations, count);
    assert_eq!(delta.order_mutations, order);
    assert_eq!(delta.damage_regions, damage);
    assert_eq!(delta.delta_rows_carried, count + order + damage);
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
    successor_anchor: Option<&OracleRect>,
) {
    let observed = transcript
        .logical_damage()
        .iter()
        .map(|damage| box_values(damage.bounds()))
        .collect::<Vec<_>>();
    let mut expected = removed.iter().map(|row| row.bounds).collect::<Vec<_>>();
    expected.extend(successor_anchor.map(|row| row.bounds));
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
