use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct UiCommandRoutingPlan {
    candidates: Vec<super::candidate::UiCommandRouteCandidate>,
    first_stroke_index: BTreeMap<
        (
            crate::capability::UiCommandShortcutStroke,
            super::context::UiCommandRoutingContextKey,
        ),
        Vec<usize>,
    >,
    platform: crate::capability::UiCommandShortcutPlatform,
}

impl UiCommandRoutingPlan {
    pub(super) fn compile(commands: &crate::capability::FrozenCommandCapabilities) -> Self {
        let candidates = commands
            .descriptors()
            .iter()
            .filter_map(|descriptor| {
                Some(super::candidate::UiCommandRouteCandidate::new(
                    descriptor.id().clone(),
                    descriptor.default_shortcut(),
                    descriptor.route()?,
                ))
            })
            .collect::<Vec<_>>();
        Self::from_candidates(
            candidates,
            crate::capability::UiCommandShortcutPlatform::current_target(),
        )
    }

    pub(super) fn first_stroke_candidates(
        &self,
        stroke: super::input_stroke::UiCommandInputStroke,
        platform: crate::capability::UiCommandShortcutPlatform,
        context: &super::UiCommandRoutingContext,
    ) -> Vec<&super::candidate::UiCommandRouteCandidate> {
        let mut indexes = std::collections::BTreeSet::new();
        for key in context.active_keys() {
            for observed in [Some(stroke.logical()), stroke.physical()]
                .into_iter()
                .flatten()
            {
                if let Some(matched) = self
                    .first_stroke_index
                    .get(&(observed.resolved_for(platform), key))
                {
                    indexes.extend(matched.iter().copied());
                }
            }
        }
        indexes
            .into_iter()
            .map(|index| &self.candidates[index])
            .collect()
    }

    pub(super) fn len(&self) -> usize {
        self.candidates.len()
    }

    #[cfg(test)]
    pub(super) const fn platform(&self) -> crate::capability::UiCommandShortcutPlatform {
        self.platform
    }

    fn from_candidates(
        candidates: Vec<super::candidate::UiCommandRouteCandidate>,
        platform: crate::capability::UiCommandShortcutPlatform,
    ) -> Self {
        let mut plan = Self {
            candidates,
            first_stroke_index: BTreeMap::new(),
            platform,
        };
        plan.rebuild_indexes(platform);
        plan
    }

    fn rebuild_indexes(&mut self, platform: crate::capability::UiCommandShortcutPlatform) {
        self.first_stroke_index.clear();
        for (index, candidate) in self.candidates.iter().enumerate() {
            if let Some(shortcut) = candidate.shortcut() {
                let first = shortcut.strokes()[0].resolved_for(platform);
                if let Some(key) =
                    super::context::UiCommandRoutingContextKey::for_route(candidate.route())
                {
                    self.first_stroke_index
                        .entry((first, key))
                        .or_default()
                        .push(index);
                }
            }
        }
    }
}

impl Default for UiCommandRoutingPlan {
    fn default() -> Self {
        Self {
            candidates: Vec::new(),
            first_stroke_index: BTreeMap::new(),
            platform: crate::capability::UiCommandShortcutPlatform::current_target(),
        }
    }
}

#[cfg(test)]
impl UiCommandRoutingPlan {
    pub(super) fn for_test(candidates: Vec<super::candidate::UiCommandRouteCandidate>) -> Self {
        Self::from_candidates(
            candidates,
            crate::capability::UiCommandShortcutPlatform::current_target(),
        )
    }

    pub(super) fn for_test_platform(
        candidates: Vec<super::candidate::UiCommandRouteCandidate>,
        platform: crate::capability::UiCommandShortcutPlatform,
    ) -> Self {
        Self::from_candidates(candidates, platform)
    }
}
