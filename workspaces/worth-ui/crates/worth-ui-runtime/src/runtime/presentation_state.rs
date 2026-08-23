use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Arc;

#[cfg(test)]
#[path = "presentation_state_tests.rs"]
mod tests;

pub(crate) struct UiApplicationPresentationState {
    rows: HashMap<Box<str>, UiApplicationSemanticTextRow>,
    token_values: BTreeMap<crate::capability::ThemeTokenId, crate::capability::ThemeTokenValue>,
    resolved_targets: BTreeMap<crate::capability::ThemeTokenId, crate::capability::ThemeTokenId>,
    mutable_token_revisions: BTreeMap<crate::capability::ThemeTokenId, u64>,
}

struct UiApplicationSemanticTextRow {
    graph_node: Option<crate::graph::UiGraphNodeIdentity>,
    value: Option<Arc<str>>,
    contract: crate::capability::ComponentSemanticTextContract,
    semantic_revision: u64,
    presentation_revision: u64,
    projected_presentation_revision: Option<u64>,
}

pub(crate) struct UiApplicationPresentationProjection {
    content: crate::mounting::UiMountedSemanticContentInput,
    revisions: Box<[(Box<str>, u64)]>,
}

impl UiApplicationPresentationState {
    pub(crate) fn activate(capabilities: &crate::capability::CapabilitySnapshot) -> Self {
        let mut token_values = BTreeMap::new();
        let mut resolved_targets = BTreeMap::new();
        let mut mutable_token_revisions = BTreeMap::new();
        for entry in capabilities.theme_tokens().entries() {
            let target = entry.resolved_target_id().clone();
            let value = capabilities
                .theme_tokens()
                .get(&target)
                .and_then(crate::capability::ThemeTokenDescriptor::value)
                .expect("frozen theme-token alias target has a value")
                .clone();
            token_values.insert(entry.descriptor().id().clone(), value);
            resolved_targets.insert(entry.descriptor().id().clone(), target);
            if entry.descriptor().source().is_application_owned()
                && entry.descriptor().alias_target().is_none()
            {
                mutable_token_revisions.insert(entry.descriptor().id().clone(), 0);
            }
        }
        let rows = capabilities
            .components()
            .descriptors()
            .iter()
            .filter_map(|component| {
                component.semantic_text_contract().map(|contract| {
                    (
                        Box::<str>::from(format!("component:{}", component.id().as_str())),
                        UiApplicationSemanticTextRow {
                            graph_node: None,
                            value: None,
                            contract: contract.clone(),
                            semantic_revision: 0,
                            presentation_revision: 0,
                            projected_presentation_revision: None,
                        },
                    )
                })
            })
            .collect();
        Self {
            rows,
            token_values,
            resolved_targets,
            mutable_token_revisions,
        }
    }

    pub(crate) fn register_semantic_text(
        &mut self,
        authored_identity: Box<str>,
        graph_node: crate::graph::UiGraphNodeIdentity,
    ) -> Result<(), ()> {
        let Some(row) = self.rows.get_mut(authored_identity.as_ref()) else {
            return Ok(());
        };
        if row.graph_node.replace(graph_node).is_some() {
            return Err(());
        }
        Ok(())
    }

    pub(crate) fn admit_semantic_text(
        &mut self,
        changes: &[crate::facade::entry::UiNativeComponentSemanticTextChange],
    ) -> Result<(), ()> {
        let mut seen = HashSet::with_capacity(changes.len());
        let mut contracts = Vec::with_capacity(changes.len());
        for change in changes {
            if !seen.insert(change.authored_semantic_identity()) {
                return Err(());
            }
            let row = self
                .rows
                .get(change.authored_semantic_identity())
                .ok_or(())?;
            if row.semantic_revision != change.expected_revision() || row.graph_node.is_none() {
                return Err(());
            }
            let contract = match change.spans() {
                Some(spans) => self.validate_span_successor(row, change.text(), spans)?,
                None => row.contract.clone(),
            };
            contracts.push(contract);
        }
        for (change, contract) in changes.iter().zip(contracts) {
            let row = self
                .rows
                .get_mut(change.authored_semantic_identity())
                .expect("validated semantic-text row remains installed");
            row.semantic_revision = row.semantic_revision.checked_add(1).ok_or(())?;
            let next: Arc<str> = Arc::from(change.text());
            let changed = row.value.as_deref() != Some(next.as_ref()) || row.contract != contract;
            row.value = Some(next);
            row.contract = contract;
            if changed {
                row.presentation_revision = row.presentation_revision.checked_add(1).ok_or(())?;
            }
        }
        Ok(())
    }

    pub(crate) fn admit_theme_values(
        &mut self,
        changes: &[crate::facade::entry::UiNativeThemeTokenValueChange],
    ) -> Result<(), ()> {
        let mut seen = HashSet::with_capacity(changes.len());
        for change in changes {
            if !seen.insert(change.token()) {
                return Err(());
            }
            let revision = self.mutable_token_revisions.get(change.token()).ok_or(())?;
            if *revision != change.expected_revision() {
                return Err(());
            }
        }
        for change in changes {
            let revision = self
                .mutable_token_revisions
                .get_mut(change.token())
                .expect("validated mutable theme token remains installed");
            *revision = revision.checked_add(1).ok_or(())?;
            let target = change.token();
            let affected = self
                .resolved_targets
                .iter()
                .filter_map(|(token, resolved)| (resolved == target).then_some(token.clone()))
                .collect::<Vec<_>>();
            let changed = affected
                .iter()
                .any(|token| self.token_values.get(token) != Some(change.value()));
            for token in &affected {
                self.token_values
                    .insert(token.clone(), change.value().clone());
            }
            if changed {
                self.mark_token_consumers_pending(target)?;
            }
        }
        Ok(())
    }

    fn validate_span_successor(
        &self,
        row: &UiApplicationSemanticTextRow,
        text: &str,
        spans: &[crate::capability::ComponentSemanticTextSpanContract],
    ) -> Result<crate::capability::ComponentSemanticTextContract, ()> {
        let exact_end = u32::try_from(text.len()).map_err(|_| ())?;
        if spans.last().map(|span| span.original_range().end()) != Some(exact_end)
            || spans.iter().any(|span| {
                !text.is_char_boundary(span.original_range().start() as usize)
                    || !text.is_char_boundary(span.original_range().end() as usize)
                    || !self.token_values.contains_key(span.foreground_token())
            })
        {
            return Err(());
        }
        crate::capability::ComponentSemanticTextContract::spanned(
            row.contract.theme_token().clone(),
            row.contract.layer_semantic_order(),
            spans.iter().cloned(),
        )
        .map_err(|_| ())
    }

    fn mark_token_consumers_pending(
        &mut self,
        target: &crate::capability::ThemeTokenId,
    ) -> Result<(), ()> {
        for row in self.rows.values_mut() {
            let consumes = row
                .contract
                .foreground_tokens()
                .any(|token| self.resolved_targets.get(token) == Some(target));
            if consumes {
                row.presentation_revision = row.presentation_revision.checked_add(1).ok_or(())?;
            }
        }
        Ok(())
    }

    pub(crate) fn project(
        &self,
    ) -> Result<UiApplicationPresentationProjection, crate::mounting::UiMountedFramePreparationDenial>
    {
        self.project_rows(|row| {
            row.projected_presentation_revision != Some(row.presentation_revision)
        })
    }

    pub(crate) fn project_complete(
        &self,
    ) -> Result<UiApplicationPresentationProjection, crate::mounting::UiMountedFramePreparationDenial>
    {
        self.project_rows(|_| true)
    }

    fn project_rows(
        &self,
        include: impl Fn(&UiApplicationSemanticTextRow) -> bool,
    ) -> Result<UiApplicationPresentationProjection, crate::mounting::UiMountedFramePreparationDenial>
    {
        let mut content = crate::mounting::UiMountedSemanticContentInput::empty();
        let mut revisions = Vec::new();
        for (identity, row) in &self.rows {
            if !include(row) {
                continue;
            }
            let (Some(value), Some(graph_node)) = (&row.value, row.graph_node) else {
                continue;
            };
            let token_values = row
                .contract
                .foreground_tokens()
                .map(|token| {
                    self.token_values
                        .get(token)
                        .cloned()
                        .map(|value| (token.clone(), value))
                        .ok_or_else(unknown_graph_node)
                })
                .collect::<Result<BTreeMap<_, _>, _>>()?;
            content
                .insert_scalar_with_formatting(
                    graph_node,
                    crate::mounting::UiMountedSemanticTextValueDirective::Replace(Arc::clone(
                        value,
                    )),
                    Arc::from("active-application-presentation"),
                    Some(
                        crate::mounting::UiMountedSemanticTextFormattingDirective::new(
                            row.contract.clone(),
                            token_values,
                        ),
                    ),
                )
                .map_err(|_| unknown_graph_node())?;
            revisions.push((identity.clone(), row.presentation_revision));
        }
        Ok(UiApplicationPresentationProjection {
            content,
            revisions: revisions.into_boxed_slice(),
        })
    }

    pub(crate) fn commit(&mut self, projection: &UiApplicationPresentationProjection) {
        for (identity, revision) in &projection.revisions {
            if let Some(row) = self.rows.get_mut(identity.as_ref()) {
                if row.presentation_revision == *revision {
                    row.projected_presentation_revision = Some(*revision);
                }
            }
        }
    }
}

impl UiApplicationPresentationProjection {
    pub(crate) fn content(&self) -> crate::mounting::UiMountedSemanticContentInput {
        self.content.clone()
    }
}

fn unknown_graph_node() -> crate::mounting::UiMountedFramePreparationDenial {
    crate::mounting::UiMountedFramePreparationDenial::Projection(
        crate::mounting::UiMountedProjectionDenial::UnknownGraphNode,
    )
}
