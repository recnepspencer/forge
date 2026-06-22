#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthTouchedGraphOrdinaryPublicFacadeExport {
    facade_source_path: String,
    authority_source_path: String,
    exported_surface: String,
}

impl WorthTouchedGraphOrdinaryPublicFacadeExport {
    fn new(facade_source_path: &str, authority_source_path: &str, exported_surface: &str) -> Self {
        Self {
            facade_source_path: facade_source_path.to_string(),
            authority_source_path: authority_source_path.to_string(),
            exported_surface: exported_surface.to_string(),
        }
    }

    pub(crate) fn source_path(&self) -> &str {
        &self.facade_source_path
    }

    pub(crate) fn authority_source_path(&self) -> &str {
        &self.authority_source_path
    }

    pub(crate) fn exported_surface(&self) -> &str {
        &self.exported_surface
    }
}

const VALIDATION_FACADE_SOURCE_PATH: &str = "crates/worth-topo/src/validation/mod.rs";
const VALIDATION_FACADE_SOURCE: &str = include_str!("../../../worth-topo/src/validation/mod.rs");
const TOPOLOGY_FACADE_SOURCE_PATH: &str = "crates/worth-topo/src/facade.rs";
const TOPOLOGY_FACADE_SOURCE: &str = include_str!("../../../worth-topo/src/facade.rs");
const SPATIAL_WORKLOAD_VOCABULARY_FACADE_SOURCE_PATH: &str =
    "crates/worth-spatial/src/facade/workload_vocabulary/mod.rs";
const SPATIAL_WORKLOAD_VOCABULARY_FACADE_SOURCE: &str =
    include_str!("../../../worth-spatial/src/facade/workload_vocabulary/mod.rs");
const SPATIAL_RETAINED_REPLAY_WORKLOAD_FACADE_SOURCE_PATH: &str =
    "crates/worth-spatial/src/facade/retained_replay_workload/mod.rs";
const SPATIAL_RETAINED_REPLAY_WORKLOAD_FACADE_SOURCE: &str =
    include_str!("../../../worth-spatial/src/facade/retained_replay_workload/mod.rs");
const SPATIAL_USER_RESPONSE_FACADE_SOURCE_PATH: &str =
    "crates/worth-spatial/src/facade/user_response/mod.rs";
const SPATIAL_USER_RESPONSE_FACADE_SOURCE: &str =
    include_str!("../../../worth-spatial/src/facade/user_response/mod.rs");

pub(crate) fn current_worth_touched_graph_ordinary_public_facade_exports(
) -> Vec<WorthTouchedGraphOrdinaryPublicFacadeExport> {
    let mut exports = [
        (VALIDATION_FACADE_SOURCE_PATH, VALIDATION_FACADE_SOURCE),
        (TOPOLOGY_FACADE_SOURCE_PATH, TOPOLOGY_FACADE_SOURCE),
        (
            SPATIAL_WORKLOAD_VOCABULARY_FACADE_SOURCE_PATH,
            SPATIAL_WORKLOAD_VOCABULARY_FACADE_SOURCE,
        ),
        (
            SPATIAL_RETAINED_REPLAY_WORKLOAD_FACADE_SOURCE_PATH,
            SPATIAL_RETAINED_REPLAY_WORKLOAD_FACADE_SOURCE,
        ),
        (
            SPATIAL_USER_RESPONSE_FACADE_SOURCE_PATH,
            SPATIAL_USER_RESPONSE_FACADE_SOURCE,
        ),
    ]
    .into_iter()
    .flat_map(|(source_path, source)| {
        ordinary_public_facade_exports_from_source(source_path, source)
    })
    .collect::<Vec<_>>();
    exports.sort_by(|left, right| {
        left.authority_source_path
            .cmp(&right.authority_source_path)
            .then(left.exported_surface.cmp(&right.exported_surface))
    });
    exports
}

pub(crate) fn ordinary_public_facade_exports_from_source(
    source_path: &str,
    source: &str,
) -> Vec<WorthTouchedGraphOrdinaryPublicFacadeExport> {
    let mut exports = Vec::new();
    let mut active_public_use_authority_source_path: Option<String> = None;

    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(authority_source_path) = &active_public_use_authority_source_path {
            if let Some(end) = trimmed.strip_suffix("};") {
                push_public_use_block_surfaces(
                    &mut exports,
                    source_path,
                    authority_source_path,
                    end,
                );
                active_public_use_authority_source_path = None;
            } else {
                push_public_use_block_surfaces(
                    &mut exports,
                    source_path,
                    authority_source_path,
                    trimmed,
                );
            }
            continue;
        }

        if let Some(authority_source_path) =
            public_use_block_authority_source_path(source_path, trimmed)
        {
            active_public_use_authority_source_path = Some(authority_source_path);
            continue;
        }
        if let Some((authority_source_path, surface)) =
            public_use_single_authority_and_surface(source_path, trimmed)
        {
            exports.push(export(source_path, &authority_source_path, surface));
            continue;
        }
        if let Some(surface) = public_module_surface(trimmed) {
            exports.push(export(source_path, source_path, surface));
        }
    }

    exports.sort_by(|left, right| left.exported_surface.cmp(&right.exported_surface));
    exports
}

fn public_use_block_authority_source_path(source_path: &str, line: &str) -> Option<String> {
    line.strip_prefix("pub use ")
        .and_then(|export| export.strip_suffix("::{"))
        .filter(|module| !module.contains("(crate)"))
        .map(|module| authority_source_path_from_public_use_module(source_path, module))
}

fn public_use_single_authority_and_surface<'a>(
    source_path: &str,
    line: &'a str,
) -> Option<(String, &'a str)> {
    let export = line.strip_prefix("pub use ")?.strip_suffix(';')?;
    if export.contains("(crate)") || export.contains("::{") {
        return None;
    }
    let (module, surface) = export.rsplit_once("::")?;
    Some((
        authority_source_path_from_public_use_module(source_path, module),
        surface,
    ))
}

fn public_module_surface(line: &str) -> Option<&str> {
    line.strip_prefix("pub mod ")?.strip_suffix(';')
}

fn push_public_use_block_surfaces(
    exports: &mut Vec<WorthTouchedGraphOrdinaryPublicFacadeExport>,
    facade_source_path: &str,
    authority_source_path: &str,
    line: &str,
) {
    for surface in line
        .split(',')
        .map(str::trim)
        .filter(|surface| !surface.is_empty())
    {
        exports.push(export(facade_source_path, authority_source_path, surface));
    }
}

fn export(
    facade_source_path: &str,
    authority_source_path: &str,
    exported_surface: &str,
) -> WorthTouchedGraphOrdinaryPublicFacadeExport {
    WorthTouchedGraphOrdinaryPublicFacadeExport::new(
        facade_source_path,
        authority_source_path,
        exported_surface,
    )
}

fn authority_source_path_from_public_use_module(source_path: &str, module: &str) -> String {
    if let Some(module_path) = module.strip_prefix("crate::") {
        let crate_root = source_path.split("/src/").next().unwrap_or(source_path);
        format!("{crate_root}/src/{}", module_path.replace("::", "/"))
    } else {
        source_path.to_string()
    }
}
