use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use worth_ui::facade::ComponentId;
use worth_ui_validation_app::reload::ValidationSourcePackage;
use worth_ui_validation_app::reload::{ValidationReloadLoop, ValidationReloadLoopConfig};
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

pub fn component_id(raw_text: &str) -> ComponentId {
    ComponentId::new(raw_text).expect("valid component id")
}

pub struct ComponentReloadLoopFixture {
    pub component_path: PathBuf,
    theme_path: PathBuf,
}

impl ComponentReloadLoopFixture {
    pub fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "worth-ui-component-reload-loop-{}",
            unique_suffix()
        ));
        fs::create_dir_all(&root).expect("fixture root should be created");
        let component_path = root.join("header.components");
        let theme_path = root.join("header.theme");
        fs::write(
            &component_path,
            "\
component_id = validation.component.header.dropdown
prop_schema = validation.header.dropdown.props
child_policy = no_children
state_ownership = runtime_owned
focus = focusable
execution_lane = interactive",
        )
        .expect("component fixture should be written");
        fs::write(&theme_path, "# Worth UI validation header theme.\n")
            .expect("theme fixture should be written");
        Self {
            component_path,
            theme_path,
        }
    }

    pub fn start_loop(&self) -> ValidationReloadLoop {
        ValidationReloadLoop::start(
            ValidationReloadLoopConfig::new(&self.theme_path)
                .with_component_path(&self.component_path),
        )
        .expect("reload loop should start from readable fixture files")
    }

    pub fn write_component(&self, source_text: &str) {
        fs::write(&self.component_path, source_text).expect("component fixture should be writable");
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
