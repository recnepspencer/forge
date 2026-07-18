use std::path::Path;

use serde::Serialize;

use super::UiProofRunFailure;

#[derive(Serialize)]
pub(super) struct UiEnvironmentManifestContract {
    profiles: toml::Table,
}

impl UiEnvironmentManifestContract {
    pub(super) fn load(
        store_root: &Path,
        required_profile: &str,
    ) -> Result<Self, UiProofRunFailure> {
        let manifest_path = store_root.join("Cargo.toml");
        let source = std::fs::read_to_string(&manifest_path).map_err(|error| {
            UiProofRunFailure::EnvironmentObservation(format!(
                "read {}: {error}",
                manifest_path.display()
            ))
        })?;
        let manifest = source.parse::<toml::Table>().map_err(|error| {
            UiProofRunFailure::EnvironmentObservation(format!(
                "parse {}: {error}",
                manifest_path.display()
            ))
        })?;
        let declared = required_table(&manifest, "profile", "Store manifest")?;
        let profiles = profile_inheritance_closure(declared, required_profile)?;
        Ok(Self { profiles })
    }

    pub(super) fn render(
        &self,
        package_identity: &str,
        dependency_manifest: &str,
    ) -> Result<String, UiProofRunFailure> {
        let dependency_document = dependency_manifest
            .parse::<toml::Table>()
            .map_err(|error| {
                UiProofRunFailure::InvalidDeclaration(format!(
                    "UI dependency manifest is invalid TOML: {error}"
                ))
            })?;
        if dependency_document.len() != 1 || !dependency_document.contains_key("dependencies") {
            return Err(UiProofRunFailure::InvalidDeclaration(
                "UI dependency manifest may contain only [dependencies]".to_owned(),
            ));
        }
        let dependencies = required_table(
            &dependency_document,
            "dependencies",
            "UI dependency manifest",
        )?
        .clone();

        let mut package = toml::Table::new();
        package.insert(
            "name".to_owned(),
            toml::Value::String(format!("store_ui_{}", &package_identity[..16])),
        );
        package.insert(
            "version".to_owned(),
            toml::Value::String("0.0.0".to_owned()),
        );
        package.insert("edition".to_owned(), toml::Value::String("2021".to_owned()));

        let mut workspace = toml::Table::new();
        workspace.insert("resolver".to_owned(), toml::Value::String("2".to_owned()));

        let mut document = toml::Table::new();
        document.insert("package".to_owned(), toml::Value::Table(package));
        document.insert("workspace".to_owned(), toml::Value::Table(workspace));
        document.insert("dependencies".to_owned(), toml::Value::Table(dependencies));
        document.insert(
            "profile".to_owned(),
            toml::Value::Table(self.profiles.clone()),
        );
        toml::to_string(&document).map_err(|error| {
            UiProofRunFailure::InvalidDeclaration(format!(
                "could not render UI environment manifest: {error}"
            ))
        })
    }
}

fn profile_inheritance_closure(
    declared: &toml::Table,
    required_profile: &str,
) -> Result<toml::Table, UiProofRunFailure> {
    let mut selected = toml::Table::new();
    let mut current = required_profile.to_owned();
    loop {
        if selected.contains_key(&current) {
            return Err(UiProofRunFailure::InvalidDeclaration(format!(
                "Cargo profile inheritance contains a cycle at {current:?}"
            )));
        }
        let profile = declared.get(&current).cloned().ok_or_else(|| {
            UiProofRunFailure::InvalidDeclaration(format!(
                "UI profile {current:?} is not declared by the Store workspace"
            ))
        })?;
        let inherited = profile
            .as_table()
            .and_then(|table| table.get("inherits"))
            .and_then(toml::Value::as_str)
            .map(str::to_owned);
        selected.insert(current, profile);
        let Some(parent) = inherited else {
            break;
        };
        current = parent;
    }
    Ok(selected)
}

fn required_table<'a>(
    parent: &'a toml::Table,
    key: &str,
    owner: &str,
) -> Result<&'a toml::Table, UiProofRunFailure> {
    parent
        .get(key)
        .and_then(toml::Value::as_table)
        .ok_or_else(|| {
            UiProofRunFailure::EnvironmentObservation(format!(
                "{owner} omits required table {key:?}"
            ))
        })
}
