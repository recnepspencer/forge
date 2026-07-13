use crate::orientation::CrateOrientation;
use std::fs;
use std::path::Path;

const TEMPLATE_PATH: &str = "templates/agent_context.md.tmpl";

pub(crate) fn render_context(orientation: &CrateOrientation) -> Result<String, String> {
    let template_root = Path::new(env!("CARGO_MANIFEST_DIR")).join(TEMPLATE_PATH);
    let template = fs::read_to_string(&template_root)
        .map_err(|e| format!("read {}: {e}", template_root.display()))?;
    Ok(apply_template(template, orientation))
}

fn apply_template(template: String, orientation: &CrateOrientation) -> String {
    let replacements = [
        ("{{crate_name}}", orientation.crate_name.as_str()),
        ("{{relative_path}}", orientation.relative_path.as_str()),
        (
            "{{machine_constitution}}",
            orientation.machine_constitution.as_str(),
        ),
        (
            "{{constitutional_class}}",
            orientation.constitutional_class.as_str(),
        ),
        ("{{domain}}", orientation.domain.as_str()),
        ("{{exemplar_role}}", orientation.exemplar_role.as_str()),
        (
            "{{deferred_routes}}",
            &render_lines(&orientation.deferred_routes),
        ),
        (
            "{{allowed_target_bands}}",
            &render_list(&orientation.allowed_target_bands),
        ),
        (
            "{{facade_exports}}",
            &render_list(&orientation.facade_exports),
        ),
        (
            "{{owned_modules}}",
            &render_list(&orientation.owned_modules),
        ),
        (
            "{{machine_fences}}",
            &render_lines(&orientation.machine_fences),
        ),
        ("{{skeleton_fence}}", orientation.skeleton_fence.as_str()),
    ];

    replacements
        .into_iter()
        .fold(template, |current, (key, value)| {
            current.replace(key, value)
        })
}

fn render_list(items: &[String]) -> String {
    if items.is_empty() {
        return "none".to_owned();
    }
    items.join(", ")
}

fn render_lines(items: &[String]) -> String {
    items
        .iter()
        .map(|item| format!("- {item}"))
        .collect::<Vec<_>>()
        .join("\n")
}
