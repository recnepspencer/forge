#[derive(Default)]
pub(crate) struct ImplContext {
    current_owner: Option<String>,
    brace_depth: i32,
}

impl ImplContext {
    pub(crate) fn current_owner(&self) -> Option<&str> {
        self.current_owner.as_deref()
    }

    pub(crate) fn observe_opening(&mut self, line: &str) {
        if self.current_owner.is_none() {
            if let Some(owner) = impl_owner_name(line.trim_start()) {
                self.current_owner = Some(owner);
            }
        }
    }

    pub(crate) fn observe_closing(&mut self, line: &str) {
        if self.current_owner.is_none() {
            return;
        }
        self.brace_depth += brace_delta(line);
        if self.brace_depth <= 0 {
            self.current_owner = None;
            self.brace_depth = 0;
        }
    }
}

pub(crate) fn declared_visible_identifier(line: &str, impl_owner: Option<&str>) -> Option<String> {
    if !is_visible_declaration(line.trim_start()) {
        return None;
    }
    declared_identifier(line.trim_start(), impl_owner, true)
}

pub(crate) fn declared_private_identifier(line: &str, impl_owner: Option<&str>) -> Option<String> {
    if is_visible_declaration(line.trim_start()) {
        return None;
    }
    declared_identifier(line.trim_start(), impl_owner, false)
}

fn declared_identifier(line: &str, impl_owner: Option<&str>, allow_mod: bool) -> Option<String> {
    let rest = strip_declaration_prefixes(line);
    let keywords = if allow_mod {
        ["fn ", "struct ", "enum ", "mod "].as_slice()
    } else {
        ["fn ", "struct ", "enum "].as_slice()
    };
    for keyword in keywords {
        if let Some(name) = rest.strip_prefix(keyword) {
            let identifier = name
                .split(|ch: char| !(ch == '_' || ch.is_ascii_alphanumeric()))
                .next()
                .filter(|value| !value.is_empty())?;
            if *keyword == "fn " {
                if let Some(impl_owner) = impl_owner {
                    return Some(format!("{impl_owner}::{identifier}"));
                }
            }
            return Some(identifier.to_string());
        }
    }
    None
}

fn is_visible_declaration(line: &str) -> bool {
    line.starts_with("pub ")
        || line.starts_with("pub(")
        || line.starts_with("pub(crate)")
        || line.starts_with("pub(super)")
        || line.starts_with("pub(in ")
}

fn strip_declaration_prefixes(mut rest: &str) -> &str {
    loop {
        let next = rest
            .strip_prefix("pub(crate) ")
            .or_else(|| rest.strip_prefix("pub(super) "))
            .or_else(|| rest.strip_prefix("pub "))
            .or_else(|| strip_pub_in(rest))
            .or_else(|| rest.strip_prefix("async "))
            .or_else(|| rest.strip_prefix("const "))
            .or_else(|| rest.strip_prefix("unsafe "))
            .or_else(|| rest.strip_prefix("default "));
        match next {
            Some(value) => rest = value.trim_start(),
            None => return rest,
        }
    }
}

fn strip_pub_in(rest: &str) -> Option<&str> {
    let rest = rest.strip_prefix("pub(in ")?;
    let closing = rest.find(") ")?;
    Some(&rest[closing + 2..])
}

fn impl_owner_name(line: &str) -> Option<String> {
    let rest = impl_header_body(line)?;
    if let Some((trait_name, target_name)) = trait_impl_names(rest) {
        return Some(format!("{trait_name} for {target_name}"));
    }
    inherent_impl_name(rest).map(str::to_string)
}

fn inherent_impl_name(rest: &str) -> Option<&str> {
    let candidate = rest
        .split(|ch: char| ch.is_whitespace() || ch == '{' || ch == '<')
        .next()?;
    if candidate.is_empty() || candidate.contains("::") {
        return None;
    }
    Some(candidate)
}

fn trait_impl_names(rest: &str) -> Option<(&str, &str)> {
    let normalized = rest.trim_end_matches('{').trim();
    let (trait_name, target_name) = normalized.split_once(" for ")?;
    Some((trait_name.trim(), target_name.trim()))
}

fn impl_header_body(line: &str) -> Option<&str> {
    if let Some(rest) = line.strip_prefix("impl ") {
        return Some(rest);
    }
    let rest = line.strip_prefix("impl<")?;
    let (_, body) = rest.split_once("> ")?;
    Some(body)
}

fn brace_delta(line: &str) -> i32 {
    line.chars().fold(0, |depth, ch| match ch {
        '{' => depth + 1,
        '}' => depth - 1,
        _ => depth,
    })
}
