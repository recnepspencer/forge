use serde_json::Value;

pub(super) fn validate(observation: &Value) -> Result<(), String> {
    if observation.get("architecture").and_then(Value::as_str) == Some("x86_64")
        && observation
            .get("os_version")
            .and_then(Value::as_str)
            .is_some_and(qualified_windows_11_version)
    {
        Ok(())
    } else {
        Err("native boundary OS version is not qualified".to_owned())
    }
}

fn qualified_windows_11_version(version: &str) -> bool {
    version
        .strip_prefix("Microsoft Windows [Version 10.0.")
        .and_then(|tail| tail.strip_suffix(']'))
        .and_then(|tail| tail.split('.').next())
        .and_then(|build| build.parse::<u32>().ok())
        .is_some_and(|build| build >= 22_000)
}
