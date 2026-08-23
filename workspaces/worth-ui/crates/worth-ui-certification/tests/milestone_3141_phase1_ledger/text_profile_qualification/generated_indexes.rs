use std::path::Path;
use std::process::Command;

pub(super) fn validate(profile_root: &Path) -> Result<(), String> {
    let repository = profile_root
        .ancestors()
        .nth(4)
        .ok_or("text profile is outside the repository")?;
    let script = repository.join("scripts/ci/build_worth_ui_text_profile.py");
    let output = Command::new(python())
        .arg(script)
        .arg("--check")
        .current_dir(repository)
        .output()
        .map_err(|error| format!("text profile index builder did not run: {error}"))?;
    if output.status.success() {
        return Ok(());
    }
    Err(format!(
        "generated text profile indexes drifted: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    ))
}

fn python() -> String {
    std::env::var("PYTHON").unwrap_or_else(|_| "python".to_owned())
}
