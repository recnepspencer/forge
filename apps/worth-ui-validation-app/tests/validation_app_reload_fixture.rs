#![allow(dead_code)]

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use worth_ui_validation_app::reload::VALIDATION_SAMPLE_LIVE_VIEW_SOURCE;
use worth_ui_validation_app::sample_source::{
    VALIDATION_SAMPLE_APPEARANCE_SOURCE, VALIDATION_SAMPLE_COMMAND_PROJECTION_SOURCE,
    VALIDATION_SAMPLE_COMMAND_SOURCE, VALIDATION_SAMPLE_COMPONENT_SOURCE,
    VALIDATION_SAMPLE_DENSITY_SOURCE, VALIDATION_SAMPLE_SOURCE, VALIDATION_SAMPLE_THEME_SOURCE,
};
use worth_ui_validation_app::{
    default_reload_loop_config_from_authored_inputs, ValidationWorkbenchApp,
    ValidationWorkbenchLaunch,
};

pub struct ValidationAppReloadFixture {
    root: PathBuf,
    pub source_path: PathBuf,
    pub live_view_path: PathBuf,
    pub theme_path: PathBuf,
    pub command_path: PathBuf,
    pub command_projection_path: PathBuf,
    pub component_path: PathBuf,
    pub appearance_path: PathBuf,
    pub density_path: PathBuf,
}

impl ValidationAppReloadFixture {
    pub fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "worth-ui-validation-app-reload-{}",
            unique_suffix()
        ));
        let source_dir = root.join("source");
        let theme_dir = root.join("theme");
        fs::create_dir_all(&source_dir).expect("source fixture dir should be created");
        fs::create_dir_all(&theme_dir).expect("theme fixture dir should be created");
        let fixture = Self {
            source_path: source_dir.join("header.wui"),
            live_view_path: source_dir.join("live_view.worth"),
            theme_path: theme_dir.join("header.theme"),
            command_path: theme_dir.join("header.commands"),
            command_projection_path: theme_dir.join("header.projections"),
            component_path: theme_dir.join("header.components"),
            appearance_path: theme_dir.join("header.appearance"),
            density_path: theme_dir.join("header.density"),
            root,
        };
        fixture.seed_files();
        fixture
    }

    pub fn build_app(&self) -> ValidationWorkbenchApp {
        let launch = ValidationWorkbenchLaunch::new()
            .prepare_from_workspace_root(&self.root)
            .expect("validation app should prepare through Worth UI");
        let reload_loop_config =
            default_reload_loop_config_from_authored_inputs(Some(launch.authored_inputs()))
                .with_theme_path(&self.theme_path)
                .with_source_path(&self.source_path)
                .with_command_path(&self.command_path)
                .with_command_projection_path(&self.command_projection_path)
                .with_component_path(&self.component_path)
                .with_appearance_path(&self.appearance_path)
                .with_density_path(&self.density_path)
                .with_live_view_path(&self.live_view_path);
        let app = ValidationWorkbenchApp::new_with_reload_loop_config(launch, reload_loop_config);
        app
    }

    pub fn workspace_root(&self) -> &std::path::Path {
        &self.root
    }

    pub fn write_source(&self, source_text: &str) {
        fs::write(&self.source_path, source_text).expect("source fixture should be writable");
    }

    pub fn write_theme(&self, source_text: &str) {
        fs::write(&self.theme_path, source_text).expect("theme fixture should be writable");
    }

    pub fn write_command_projection(&self, source_text: &str) {
        fs::write(&self.command_projection_path, source_text)
            .expect("command projection fixture should be writable");
    }

    pub fn write_command(&self, source_text: &str) {
        fs::write(&self.command_path, source_text).expect("command fixture should be writable");
    }

    pub fn write_component(&self, source_text: &str) {
        fs::write(&self.component_path, source_text).expect("component fixture should be writable");
    }

    pub fn write_appearance(&self, source_text: &str) {
        fs::write(&self.appearance_path, source_text)
            .expect("appearance fixture should be writable");
    }

    pub fn write_density(&self, source_text: &str) {
        fs::write(&self.density_path, source_text).expect("density fixture should be writable");
    }

    fn seed_files(&self) {
        fs::write(&self.source_path, VALIDATION_SAMPLE_SOURCE)
            .expect("source fixture should be written");
        fs::write(&self.live_view_path, VALIDATION_SAMPLE_LIVE_VIEW_SOURCE)
            .expect("live view fixture should be written");
        fs::write(&self.theme_path, VALIDATION_SAMPLE_THEME_SOURCE)
            .expect("theme fixture should be written");
        fs::write(&self.command_path, VALIDATION_SAMPLE_COMMAND_SOURCE)
            .expect("command fixture should be written");
        fs::write(
            &self.command_projection_path,
            VALIDATION_SAMPLE_COMMAND_PROJECTION_SOURCE,
        )
        .expect("command projection fixture should be written");
        fs::write(&self.component_path, VALIDATION_SAMPLE_COMPONENT_SOURCE)
            .expect("component fixture should be written");
        fs::write(&self.appearance_path, VALIDATION_SAMPLE_APPEARANCE_SOURCE)
            .expect("appearance fixture should be written");
        fs::write(&self.density_path, VALIDATION_SAMPLE_DENSITY_SOURCE)
            .expect("density fixture should be written");
    }
}

impl Drop for ValidationAppReloadFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

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
