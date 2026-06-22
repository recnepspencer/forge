#[derive(Clone, Debug, Default)]
pub(super) struct GraphReadBypassScanState {
    relation_row_variables: Vec<String>,
    adjacency_variables: Vec<String>,
    frontier_variables: Vec<String>,
    visited_variables: Vec<String>,
    pending_assignment_variable: Option<String>,
}

impl GraphReadBypassScanState {
    pub(super) fn observe_line(&mut self, line: &str) {
        let pending_assignment_variable = self.pending_assignment_variable.take();
        if line.contains("surfaces.relations()")
            || line.contains(".relations()")
            || line.contains("relation_rows:")
        {
            if let Some(variable) = pending_assignment_variable.as_deref() {
                push_unique(&mut self.relation_row_variables, variable);
            } else {
                self.push_named_variable(line, "relation_rows");
            }
        }
        if code_has_any(line, ["BTreeMap", "HashMap"]) {
            if let Some(variable) = pending_assignment_variable.as_deref() {
                push_unique(&mut self.adjacency_variables, variable);
            } else if line.contains("adjacency") {
                self.push_named_variable(line, "adjacency");
            }
        }
        if line.contains("frontier") && code_has_any(line, ["VecDeque", "Vec::", "from_iter"]) {
            self.push_named_variable(line, "frontier");
        }
        if line.contains("visited") && code_has_any(line, ["BTreeSet", "HashSet", "Set::"]) {
            self.push_named_variable(line, "visited");
        }
        if line.trim_end().ends_with('=') {
            self.pending_assignment_variable =
                variable_name_before_assignment(line).map(ToString::to_string);
        }
    }

    pub(super) fn line_mentions_relation_rows(&self, line: &str) -> bool {
        code_has_any(
            line,
            ["relation_rows", "relations()", "read(surfaces.relations"],
        ) || self
            .relation_row_variables
            .iter()
            .any(|variable| code_has_word(line, variable))
    }

    pub(super) fn line_mentions_adjacency(&self, line: &str) -> bool {
        code_has_any(line, ["adjacency"])
            || self
                .adjacency_variables
                .iter()
                .any(|variable| code_has_word(line, variable))
    }

    pub(super) fn line_mentions_frontier(&self, line: &str) -> bool {
        code_has_any(line, ["frontier"])
            || self
                .frontier_variables
                .iter()
                .any(|variable| code_has_word(line, variable))
    }

    pub(super) fn line_mentions_visited(&self, line: &str) -> bool {
        code_has_any(line, ["visited"])
            || self
                .visited_variables
                .iter()
                .any(|variable| code_has_word(line, variable))
    }

    fn push_named_variable(&mut self, line: &str, fallback: &str) {
        let variable = variable_name_before_assignment(line).unwrap_or(fallback);
        match fallback {
            "relation_rows" => push_unique(&mut self.relation_row_variables, variable),
            "adjacency" => push_unique(&mut self.adjacency_variables, variable),
            "frontier" => push_unique(&mut self.frontier_variables, variable),
            "visited" => push_unique(&mut self.visited_variables, variable),
            _ => {}
        }
    }
}

fn push_unique(variables: &mut Vec<String>, variable: &str) {
    if !variables.iter().any(|existing| existing == variable) {
        variables.push(variable.to_string());
    }
}

fn variable_name_before_assignment(line: &str) -> Option<&str> {
    let before_equals = line.split('=').next()?.trim();
    let before_colon = before_equals.split(':').next().unwrap_or(before_equals);
    before_colon
        .split_whitespace()
        .last()
        .filter(|candidate| !candidate.is_empty())
}

fn code_has_word(line: &str, word: &str) -> bool {
    line.match_indices(word).any(|(index, _)| {
        let before = line[..index].chars().next_back();
        let after = line[index + word.len()..].chars().next();
        !is_identifier_char(before) && !is_identifier_char(after)
    })
}

fn is_identifier_char(ch: Option<char>) -> bool {
    ch.is_some_and(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

fn code_has_any<const N: usize>(line: &str, needles: [&str; N]) -> bool {
    needles.into_iter().any(|needle| line.contains(needle))
}
