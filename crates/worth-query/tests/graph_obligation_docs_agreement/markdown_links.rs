use std::path::{Path, PathBuf};

pub fn assert_markdown_links_resolve(docs_root: &Path, relative_path: &str, contents: &str) {
    let source_path = docs_root.join(relative_path);
    let source_dir = source_path.parent().expect("doc has parent directory");
    for link in local_markdown_links(contents) {
        let link_without_fragment = link.split('#').next().unwrap_or_default();
        if link_without_fragment.is_empty() {
            continue;
        }
        let target = normalize_doc_path(source_dir.join(link_without_fragment));
        assert!(
            target.exists(),
            "markdown link `{link}` in `{relative_path}` must resolve to `{}`",
            target.display()
        );
    }
}

fn local_markdown_links(contents: &str) -> impl Iterator<Item = String> + '_ {
    markdown_links(contents).into_iter().filter(|link| {
        !link.starts_with('#')
            && !link.starts_with("http:")
            && !link.starts_with("https:")
            && !link.starts_with("mailto:")
    })
}

fn markdown_links(contents: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut cursor = contents;
    while let Some(label_start) = cursor.find('[') {
        cursor = &cursor[label_start + 1..];
        let Some(label_end) = cursor.find(']') else {
            break;
        };
        cursor = &cursor[label_end + 1..];
        if !cursor.starts_with('(') {
            continue;
        }
        cursor = &cursor[1..];
        let Some(link_end) = cursor.find(')') else {
            break;
        };
        links.push(cursor[..link_end].to_string());
        cursor = &cursor[link_end + 1..];
    }
    links
}

fn normalize_doc_path(path: PathBuf) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                normalized.pop();
            }
            std::path::Component::CurDir => {}
            other => normalized.push(other.as_os_str()),
        }
    }
    normalized
}
