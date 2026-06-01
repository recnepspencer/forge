use super::surface::{
    forge_query_platform_entry_closeout_surface, ForgeQueryPlatformEntryCloseoutSurface,
};
use crate::identity::hash_parts;

const OUTPUT_MANIFEST: &[&str] = &[
    "public_surface_digest",
    "compile_fail_boundary_digest",
    "parity_digest",
    "hostile_digest",
    "docs_coverage_digest",
    "milestone_closeout_digest",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPlatformEntryCloseoutOutput {
    name: &'static str,
    digest: String,
}

impl ForgeQueryPlatformEntryCloseoutOutput {
    fn new(name: &'static str, digest: String) -> Self {
        Self { name, digest }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPlatformEntryCloseoutBundle {
    output_manifest: Vec<&'static str>,
    surface: ForgeQueryPlatformEntryCloseoutSurface,
    outputs: Vec<ForgeQueryPlatformEntryCloseoutOutput>,
    milestone_closeout_digest: String,
}

impl ForgeQueryPlatformEntryCloseoutBundle {
    fn new(surface: ForgeQueryPlatformEntryCloseoutSurface) -> Self {
        let outputs = vec![
            ForgeQueryPlatformEntryCloseoutOutput::new(
                "public_surface_digest",
                surface.public_surface_digest().to_string(),
            ),
            ForgeQueryPlatformEntryCloseoutOutput::new(
                "compile_fail_boundary_digest",
                surface.compile_fail_boundary_digest().to_string(),
            ),
            ForgeQueryPlatformEntryCloseoutOutput::new(
                "parity_digest",
                surface.parity_digest().to_string(),
            ),
            ForgeQueryPlatformEntryCloseoutOutput::new(
                "hostile_digest",
                surface.hostile_digest().to_string(),
            ),
            ForgeQueryPlatformEntryCloseoutOutput::new(
                "docs_coverage_digest",
                surface.docs_coverage_digest().to_string(),
            ),
        ];
        let milestone_closeout_digest = hash_parts(
            &outputs
                .iter()
                .map(|output| format!("{}:{}", output.name(), output.digest()))
                .collect::<Vec<_>>(),
        );
        let mut outputs = outputs;
        outputs.push(ForgeQueryPlatformEntryCloseoutOutput::new(
            "milestone_closeout_digest",
            milestone_closeout_digest.clone(),
        ));

        let actual_names = outputs.iter().map(|row| row.name()).collect::<Vec<_>>();
        assert_eq!(actual_names, OUTPUT_MANIFEST);

        Self {
            output_manifest: OUTPUT_MANIFEST.to_vec(),
            surface,
            outputs,
            milestone_closeout_digest,
        }
    }

    pub fn output_manifest(&self) -> &[&'static str] {
        &self.output_manifest
    }

    pub fn surface(&self) -> &ForgeQueryPlatformEntryCloseoutSurface {
        &self.surface
    }

    pub fn outputs(&self) -> &[ForgeQueryPlatformEntryCloseoutOutput] {
        &self.outputs
    }

    pub fn output_digest(&self, key: &str) -> Option<&str> {
        self.outputs
            .iter()
            .find(|output| output.name() == key)
            .map(ForgeQueryPlatformEntryCloseoutOutput::digest)
    }

    pub fn milestone_closeout_digest(&self) -> &str {
        &self.milestone_closeout_digest
    }
}

pub fn certify_platform_entry_closeout() -> ForgeQueryPlatformEntryCloseoutBundle {
    ForgeQueryPlatformEntryCloseoutBundle::new(forge_query_platform_entry_closeout_surface())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundle_emits_the_required_closeout_outputs() {
        let bundle = certify_platform_entry_closeout();

        assert_eq!(bundle.output_manifest(), OUTPUT_MANIFEST);
        for key in OUTPUT_MANIFEST {
            assert!(bundle
                .output_digest(key)
                .is_some_and(|digest| !digest.is_empty()));
        }
        assert!(!bundle.milestone_closeout_digest().is_empty());
    }
}
