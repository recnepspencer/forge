use forge_foundational::facade::AspectValue;
use forge_foundational::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::runtime::ForgeQueryWorkspace;
use forge_query::facade::ForgeQueryAspectTouch;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HostileGraphFixtureSummary {
    user_count: usize,
    active_user_count: usize,
    relation_edge_count: usize,
    branching_factor: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct HostileFrontierGraphSpec {
    active_user_count: usize,
    inactive_decoy_count: usize,
    branching_factor: usize,
}

impl HostileFrontierGraphSpec {
    fn broad_branching_with_inactive_decoys() -> Self {
        Self {
            active_user_count: 48,
            inactive_decoy_count: 8,
            branching_factor: 6,
        }
    }

    fn total_user_count(&self) -> usize {
        self.active_user_count + self.inactive_decoy_count
    }
}

impl HostileGraphFixtureSummary {
    pub fn user_count(&self) -> usize {
        self.user_count
    }

    pub fn active_user_count(&self) -> usize {
        self.active_user_count
    }

    pub fn relation_edge_count(&self) -> usize {
        self.relation_edge_count
    }

    pub fn branching_factor(&self) -> usize {
        self.branching_factor
    }
}

pub fn seed_hostile_frontier_graph(
    workspace: &mut ForgeQueryWorkspace,
    prefix: &str,
) -> HostileGraphFixtureSummary {
    let spec = HostileFrontierGraphSpec::broad_branching_with_inactive_decoys();
    for index in 0..spec.active_user_count {
        workspace
            .insert("user", |user| {
                user.aspect(touch("identity.id"), text(user_id(prefix, index)))
                    .aspect(touch("status.value"), text("active"))
                    .aspect(
                        touch("profile.display_name"),
                        text(format!("User {index:03}")),
                    )
            })
            .expect("hostile active user should insert");
    }
    for index in spec.active_user_count..spec.total_user_count() {
        workspace
            .insert("user", |user| {
                user.aspect(touch("identity.id"), text(user_id(prefix, index)))
                    .aspect(touch("status.value"), text("inactive"))
                    .aspect(
                        touch("profile.display_name"),
                        text(format!("Decoy {index:03}")),
                    )
            })
            .expect("hostile decoy user should insert");
    }
    let relation_edge_count = seed_relation_edges(
        workspace,
        prefix,
        "manager",
        spec.active_user_count,
        spec.branching_factor,
    ) + seed_relation_edges(
        workspace,
        prefix,
        "mentor",
        spec.active_user_count,
        spec.branching_factor,
    );
    HostileGraphFixtureSummary {
        user_count: spec.total_user_count(),
        active_user_count: spec.active_user_count,
        relation_edge_count,
        branching_factor: spec.branching_factor,
    }
}

fn seed_relation_edges(
    workspace: &mut ForgeQueryWorkspace,
    prefix: &str,
    relation: &str,
    active_user_count: usize,
    branching_factor: usize,
) -> usize {
    let mut edge_count = 0;
    for source_index in 0..active_user_count {
        for branch in 1..=branching_factor {
            let target_index = (source_index + branch) % active_user_count;
            workspace
                .insert(relation, |edge| {
                    edge.aspect(
                        touch("identity.id"),
                        text(format!("{prefix}-{relation}-{source_index}-{target_index}")),
                    )
                    .aspect(touch("source.id"), text(user_id(prefix, source_index)))
                    .aspect(touch("target.id"), text(user_id(prefix, target_index)))
                })
                .expect("hostile relation edge should insert");
            edge_count += 1;
        }
    }
    edge_count
}

fn user_id(prefix: &str, index: usize) -> String {
    format!("{prefix}-{index}")
}

fn touch(aspect_path: &str) -> ForgeQueryAspectTouch {
    let mut segments = aspect_path.split('.');
    let aspect = segments
        .next()
        .and_then(|segment| AspectKey::new(segment.to_string()))
        .expect("fixture aspect path aspect should admit");
    let fields = segments
        .map(|segment| {
            FieldKey::new(segment.to_string()).expect("fixture aspect path field should admit")
        })
        .collect::<Vec<_>>();
    if fields.is_empty() {
        ForgeQueryAspectTouch::aspect(aspect)
    } else {
        ForgeQueryAspectTouch::field_path(
            aspect,
            CanonicalFieldPath::new(fields).expect("fixture aspect path should have fields"),
        )
    }
}

fn text(value: impl Into<String>) -> AspectValue {
    AspectValue::String(value.into().into())
}
