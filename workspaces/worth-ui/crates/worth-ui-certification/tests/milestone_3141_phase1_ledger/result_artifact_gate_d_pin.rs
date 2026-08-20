use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

pub(super) fn validate(requirement: &str, artifact: &Value) -> Result<(), String> {
    if requirement != "P5-ATLAS-PINNING-01" {
        return Ok(());
    }
    let observation = &artifact["boundary_observation"];
    if observation["schema"] != "worth-ui-native-gate-d-pin-world-v3"
        || observation["mounted_bindings"].as_u64() != Some(1)
        || observation["pinned_layouts"].as_u64() != Some(3)
        || observation["presentations"].as_u64() != Some(4)
        || observation["atlas_transactions"].as_u64() != Some(4)
        || observation["native_peak_pin_count"].as_u64() != Some(49)
        || observation["observation_history_complete"].as_bool() != Some(true)
        || observation["physical_signal_runtimes"].as_u64() != Some(1)
        || observation["physical_signal_workers"].as_u64() != Some(1)
        || observation["alpha_entries"].as_u64() != Some(40)
        || observation["color_entries"].as_u64() != Some(1)
        || observation["query_close_complete"].as_bool() != Some(true)
        || !observation["closed_query_resources"]
            .as_u64()
            .is_some_and(|count| count > 0)
        || observation["terminal_zero"].as_bool() != Some(true)
    {
        return Err("Gate D pin product census is not exact".to_owned());
    }
    validate_frames(observation)
}

fn validate_frames(observation: &Value) -> Result<(), String> {
    if observation["native_frame_pin_counts"] != serde_json::json!([49, 49, 40, 0]) {
        return Err("Gate D pin frame counts are not exact".to_owned());
    }
    let frames = observation["native_frame_pins"]
        .as_array()
        .filter(|frames| frames.len() == 4)
        .ok_or_else(|| "Gate D pin world omits its four exact frames".to_owned())?;
    let first = frame_layout_keys(&frames[0])?;
    let second = frame_layout_keys(&frames[1])?;
    let third = frame_layout_keys(&frames[2])?;
    if first.len() != 3
        || first != second
        || third.len() != 2
        || !frames[3].as_array().is_some_and(Vec::is_empty)
        || first.values().map(BTreeSet::len).sum::<usize>() != 49
        || third.values().map(BTreeSet::len).sum::<usize>() != 40
        || third
            .iter()
            .any(|(layout, keys)| first.get(layout) != Some(keys))
    {
        return Err("Gate D pin frames do not preserve exact retained owners".to_owned());
    }
    let removed = first
        .iter()
        .find(|(layout, _)| !third.contains_key(*layout))
        .map(|(_, keys)| keys)
        .ok_or_else(|| "Gate D pin frames omit the removed layout".to_owned())?;
    let shared = third
        .values()
        .find(|keys| !keys.is_disjoint(removed))
        .ok_or_else(|| "Gate D pin frames omit shared retained raster keys".to_owned())?;
    if removed.difference(shared).next().is_none() || shared.difference(removed).next().is_none() {
        return Err("Gate D pin layouts are not overlapping and non-identical".to_owned());
    }
    Ok(())
}

fn frame_layout_keys(frame: &Value) -> Result<BTreeMap<&str, BTreeSet<&str>>, String> {
    let mut layouts = BTreeMap::new();
    for pin in frame
        .as_array()
        .ok_or_else(|| "Gate D pin frame is not an array".to_owned())?
    {
        let layout = exact_digest(&pin["layout"])?;
        let key = exact_digest(&pin["raster_key"])?;
        if !layouts
            .entry(layout)
            .or_insert_with(BTreeSet::new)
            .insert(key)
        {
            return Err("Gate D pin frame repeats a layout/raster-key pair".to_owned());
        }
    }
    Ok(layouts)
}

fn exact_digest(value: &Value) -> Result<&str, String> {
    value
        .as_str()
        .filter(|digest| digest.len() == 64 && digest.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .ok_or_else(|| "Gate D pin observation has an invalid digest".to_owned())
}
