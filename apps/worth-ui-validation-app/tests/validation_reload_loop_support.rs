#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use worth_ui_validation_app::reload::ValidationSourcePackage;
use worth_ui_validation_app::sample_source::{
    VALIDATION_SAMPLE_APPEARANCE_SOURCE, VALIDATION_SAMPLE_COMMAND_PROJECTION_SOURCE,
    VALIDATION_SAMPLE_COMMAND_SOURCE, VALIDATION_SAMPLE_COMPONENT_SOURCE,
    VALIDATION_SAMPLE_DENSITY_SOURCE, VALIDATION_SAMPLE_MODULE_PATH, VALIDATION_SAMPLE_SOURCE,
    VALIDATION_SAMPLE_THEME_SOURCE,
};
use worth_ui_validation_app::{
    ValidationRuntimeWorkbench, ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch,
};

pub fn runtime_workbench() -> ValidationRuntimeWorkbench {
    ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(ValidationWorkbenchAuthoredInputs::new(
            ValidationSourcePackage::sample(),
        ))
        .expect("validation app launches through Worth UI")
        .into_runtime_workbench()
}

pub fn meaningfully_changed_source() -> String {
    VALIDATION_SAMPLE_SOURCE.replace(
        "interaction_payload \"submit.secondary\"",
        "interaction_payload \"submit.reload_loop\"",
    )
}

pub struct ReloadLoopFixture {
    pub source_path: PathBuf,
    pub theme_path: PathBuf,
    pub command_path: PathBuf,
    pub command_projection_path: PathBuf,
    pub component_path: PathBuf,
    pub appearance_path: PathBuf,
    pub density_path: PathBuf,
}

impl ReloadLoopFixture {
    pub fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "worth-ui-validation-reload-loop-{}",
            unique_suffix()
        ));
        fs::create_dir_all(&root).expect("fixture root should be created");
        let source_path = root.join("header.wui");
        let theme_path = root.join("header.theme");
        let command_path = root.join("header.commands");
        let command_projection_path = root.join("header.projections");
        let component_path = root.join("header.components");
        let appearance_path = root.join("header.appearance");
        let density_path = root.join("header.density");
        fs::write(&source_path, VALIDATION_SAMPLE_SOURCE)
            .expect("source fixture should be written");
        fs::write(&theme_path, VALIDATION_SAMPLE_THEME_SOURCE)
            .expect("theme fixture should be written");
        fs::write(&command_path, VALIDATION_SAMPLE_COMMAND_SOURCE)
            .expect("command fixture should be written");
        fs::write(
            &command_projection_path,
            VALIDATION_SAMPLE_COMMAND_PROJECTION_SOURCE,
        )
        .expect("command projection fixture should be written");
        fs::write(&component_path, VALIDATION_SAMPLE_COMPONENT_SOURCE)
            .expect("component fixture should be written");
        fs::write(&appearance_path, VALIDATION_SAMPLE_APPEARANCE_SOURCE)
            .expect("appearance fixture should be written");
        fs::write(&density_path, VALIDATION_SAMPLE_DENSITY_SOURCE)
            .expect("density fixture should be written");
        Self {
            source_path,
            theme_path,
            command_path,
            command_projection_path,
            component_path,
            appearance_path,
            density_path,
        }
    }

    pub fn start_loop(&self) -> worth_ui_validation_app::reload::ValidationReloadLoop {
        worth_ui_validation_app::reload::ValidationReloadLoop::start(
            worth_ui_validation_app::reload::ValidationReloadLoopConfig::new(&self.theme_path)
                .with_source_path(&self.source_path)
                .with_command_path(&self.command_path)
                .with_command_projection_path(&self.command_projection_path)
                .with_component_path(&self.component_path)
                .with_appearance_path(&self.appearance_path)
                .with_density_path(&self.density_path),
        )
        .expect("reload loop should start from readable fixture files")
    }

    pub fn write_source(&self, source_text: &str) {
        fs::write(&self.source_path, source_text).expect("source fixture should be writable");
    }

    pub fn write_theme(&self, theme_text: &str) {
        fs::write(&self.theme_path, theme_text).expect("theme fixture should be writable");
    }

    pub fn write_command(&self, command_text: &str) {
        fs::write(&self.command_path, command_text).expect("command fixture should be writable");
    }

    pub fn write_command_projection(&self, projection_text: &str) {
        fs::write(&self.command_projection_path, projection_text)
            .expect("command projection fixture should be writable");
    }

    pub fn write_component(&self, component_text: &str) {
        fs::write(&self.component_path, component_text)
            .expect("component fixture should be writable");
    }

    pub fn delete_source(&self) {
        fs::remove_file(&self.source_path).expect("source fixture should be removable");
    }
}

pub fn packaged_validation_source_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("source/header.wui")
}

pub const SAMPLE_MODULE_PATH: &str = VALIDATION_SAMPLE_MODULE_PATH;

fn unique_suffix() -> u128 {
    static UNIQUE_COUNTER: AtomicU64 = AtomicU64::new(0);
    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time should be after Unix epoch")
        .as_nanos();
    let counter = u128::from(UNIQUE_COUNTER.fetch_add(1, Ordering::Relaxed));
    let process = u128::from(std::process::id());
    (time << 32) ^ (process << 16) ^ counter
}
