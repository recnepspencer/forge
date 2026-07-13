//! Orientation construction for configured Query audience facade crates.

use super::model::CrateOrientation;
use crate::authority_inputs::{
    CommittedFacadeSnapshot, QueryAudienceContractSpec, QueryAudienceFacadeSpec,
};
use std::path::Path;

pub(crate) fn framework_audience_orientations(
    root: &Path,
    contract: &QueryAudienceContractSpec,
    machine_constitution: &str,
    facade_snapshot: &CommittedFacadeSnapshot,
) -> Result<Vec<CrateOrientation>, String> {
    contract
        .audiences
        .iter()
        .map(|audience| {
            let relative_path = format!("crates/{}", audience.package);
            let crate_root = root.join(&relative_path);
            if !crate_root.is_dir() {
                return Err(format!(
                    "configured Query audience facade missing at {relative_path}"
                ));
            }
            let facade_exports = facade_snapshot.exports_for(&audience.package)?;
            let machine_fences = vec![
                format!(
                    "Framework Query audience facade (`{}`); legal consuming bands: {}.",
                    audience.label,
                    render_bands(&audience.allowed_bands)
                ),
                format!(
                    "May depend only on engine package `{}`; must not depend on other audience facades.",
                    contract.engine_package
                ),
                format!(
                    "Leaf re-export surface only; guidance: {}.",
                    audience.guidance
                ),
            ];

            Ok(CrateOrientation {
                crate_name: audience.package.clone(),
                relative_path,
                constitutional_class: "framework/query-audience".to_owned(),
                domain: audience.label.clone(),
                exemplar_role: format!(
                    "Query {} audience facade over `{}`",
                    audience.label, contract.engine_package
                ),
                deferred_routes: Vec::new(),
                allowed_target_bands: Vec::new(),
                facade_exports,
                owned_modules: Vec::new(),
                machine_fences,
                skeleton_fence: "Framework audience facade: re-export-only; no seed-skeleton allowlist.".to_owned(),
                machine_constitution: machine_constitution.to_owned(),
            })
        })
        .collect()
}

pub(crate) fn query_machine_fences_for_band(
    band: &str,
    contract: &QueryAudienceContractSpec,
) -> Vec<String> {
    let mut fences = vec![format!(
        "Must not depend on Query engine `{}` directly; consume only through configured audience facades.",
        contract.engine_package
    )];

    let legal: Vec<&QueryAudienceFacadeSpec> = contract
        .audiences
        .iter()
        .filter(|audience| audience.allowed_bands.iter().any(|allowed| allowed == band))
        .collect();

    if legal.is_empty() {
        fences.push(
            "No Query audience facade is legal for this band; derived and other ordinary bands have no Query audience in this milestone."
                .to_owned(),
        );
    } else {
        fences.push(format!(
            "Legal Query audience facades for this band: {}.",
            legal
                .iter()
                .map(|audience| format!("`{}` ({})", audience.package, audience.guidance))
                .collect::<Vec<_>>()
                .join("; ")
        ));
    }

    for audience in &contract.audiences {
        if !audience.allowed_bands.iter().any(|allowed| allowed == band) {
            fences.push(format!(
                "Must not depend on Query audience facade `{}` (allowed bands: {}).",
                audience.package,
                render_bands(&audience.allowed_bands)
            ));
        }
    }

    fences
}

fn render_bands(bands: &[String]) -> String {
    if bands.is_empty() {
        return "none".to_owned();
    }
    bands.join(", ")
}
