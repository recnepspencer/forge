use crate::consumer_kit::prohibition_registry::hard_prohibition_registry;
use crate::ForgeQueryProhibitedSeam;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBoundaryAuditSeededSource {
    seam: ForgeQueryProhibitedSeam,
    label: String,
    source_path: String,
    source: String,
}

pub fn hard_prohibition_seeded_consumer_sources(
    crate_name: impl Into<String>,
) -> Vec<ForgeQueryBoundaryAuditSeededSource> {
    let crate_name = crate_name.into();
    hard_prohibition_registry()
        .rows()
        .iter()
        .flat_map(|row| {
            [
                ForgeQueryBoundaryAuditSeededSource::new(
                    row.seam(),
                    format!("{crate_name}.seeded.{}.method", row.seam_key()),
                    format!("src/seeded/{}_method.rs", row.seam_key().replace('-', "_")),
                    seeded_method_source(row.public_symbol()),
                ),
                ForgeQueryBoundaryAuditSeededSource::new(
                    row.seam(),
                    format!("{crate_name}.seeded.{}.associated-path", row.seam_key()),
                    format!(
                        "src/seeded/{}_associated_path.rs",
                        row.seam_key().replace('-', "_")
                    ),
                    seeded_associated_path_source(row.public_symbol()),
                ),
            ]
        })
        .collect()
}

impl ForgeQueryBoundaryAuditSeededSource {
    fn new(
        seam: ForgeQueryProhibitedSeam,
        label: String,
        source_path: String,
        source: String,
    ) -> Self {
        Self {
            seam,
            label,
            source_path,
            source,
        }
    }

    pub fn seam(&self) -> ForgeQueryProhibitedSeam {
        self.seam
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

fn seeded_method_source(public_symbol: &str) -> String {
    let method = public_symbol
        .rsplit("::")
        .next()
        .expect("registry public symbol should include method segment");
    format!(
        "fn seeded_method_bypass(workspace: &mut ForgeQueryWorkspace) {{\n    workspace.{method}(command);\n}}\n"
    )
}

fn seeded_associated_path_source(public_symbol: &str) -> String {
    format!("fn seeded_associated_path_bypass() {{\n    {public_symbol}(command);\n}}\n")
}
