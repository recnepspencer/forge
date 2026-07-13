use syn::UseTree;

pub(super) struct ExpandedUse {
    pub(super) target_module: Vec<String>,
    pub(super) target_name: String,
    pub(super) export_name: String,
}

pub(super) fn expand_use_targets(current_module: &[String], tree: &UseTree) -> Vec<ExpandedUse> {
    let prefix = match tree {
        UseTree::Path(path) if path.ident == "self" || path.ident == "super" => current_module,
        _ => &[],
    };
    expand_use_rec(prefix, tree)
}

fn expand_use_rec(prefix_module: &[String], tree: &UseTree) -> Vec<ExpandedUse> {
    match tree {
        UseTree::Path(path) => {
            let ident = path.ident.to_string();
            let next = match ident.as_str() {
                "self" => prefix_module.to_vec(),
                "super" => {
                    let mut parent = prefix_module.to_vec();
                    parent.pop();
                    parent
                }
                "crate" => Vec::new(),
                other => {
                    let mut nested = prefix_module.to_vec();
                    nested.push(other.to_owned());
                    nested
                }
            };
            expand_use_rec(&next, &path.tree)
        }
        UseTree::Name(name) if name.ident == "self" && !prefix_module.is_empty() => {
            let mut target_module = prefix_module.to_vec();
            let target_name = if target_module.len() == 1 {
                "*".to_owned()
            } else {
                target_module.pop().expect("non-empty prefix")
            };
            vec![ExpandedUse {
                target_module,
                target_name,
                export_name: prefix_module.last().expect("non-empty prefix").clone(),
            }]
        }
        UseTree::Name(name) => {
            let name = name.ident.to_string();
            vec![ExpandedUse {
                target_module: prefix_module.to_vec(),
                target_name: name.clone(),
                export_name: name,
            }]
        }
        UseTree::Rename(rename) => vec![ExpandedUse {
            target_module: prefix_module.to_vec(),
            target_name: rename.ident.to_string(),
            export_name: rename.rename.to_string(),
        }],
        UseTree::Glob(_) => vec![ExpandedUse {
            target_module: prefix_module.to_vec(),
            target_name: "*".to_owned(),
            export_name: "*".to_owned(),
        }],
        UseTree::Group(group) => group
            .items
            .iter()
            .flat_map(|item| expand_use_rec(prefix_module, item))
            .collect(),
    }
}
