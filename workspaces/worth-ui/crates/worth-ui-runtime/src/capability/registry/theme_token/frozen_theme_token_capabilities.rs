use std::collections::{BTreeMap, BTreeSet};

use crate::capability::ThemeTokenId;

use super::{
    FrozenThemeTokenEntry, ThemeTokenAcceptedRegistrationProof, ThemeTokenDescriptor, ThemeTokenKey,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrozenThemeTokenCapabilities {
    entries: Vec<FrozenThemeTokenEntry>,
}

impl FrozenThemeTokenCapabilities {
    pub(crate) fn empty() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub(crate) fn from_accepted_descriptors(
        mut descriptors: Vec<ThemeTokenDescriptor>,
        accepted_theme_tokens: &ThemeTokenAcceptedRegistrationProof,
    ) -> Self {
        descriptors.retain(|descriptor| accepted_theme_tokens.admits(descriptor));
        descriptors.sort_by(|left, right| left.id().cmp(right.id()));
        let terminal_alias_targets = terminal_alias_targets_by_id(&descriptors);
        let entries = descriptors
            .into_iter()
            .map(|descriptor| frozen_theme_token_entry(descriptor, &terminal_alias_targets))
            .collect();
        Self { entries }
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn entries(&self) -> &[FrozenThemeTokenEntry] {
        &self.entries
    }

    pub fn get(&self, id: &ThemeTokenId) -> Option<&ThemeTokenDescriptor> {
        self.entries
            .binary_search_by(|entry| entry.descriptor().id().cmp(id))
            .ok()
            .map(|index| self.entries[index].descriptor())
    }

    pub fn get_entry(&self, id: &ThemeTokenId) -> Option<&FrozenThemeTokenEntry> {
        self.entries
            .binary_search_by(|entry| entry.descriptor().id().cmp(id))
            .ok()
            .map(|index| &self.entries[index])
    }

    pub(crate) fn digest_basis(&self) -> u64 {
        self.entries
            .iter()
            .fold(0x8422_5d1d_b77a_11e3, |basis, entry| {
                fold_theme_token_key(basis, entry.key())
            })
    }
}

fn frozen_theme_token_entry(
    descriptor: ThemeTokenDescriptor,
    terminal_alias_targets: &BTreeMap<ThemeTokenId, ThemeTokenId>,
) -> FrozenThemeTokenEntry {
    let key = ThemeTokenKey::from_descriptor(&descriptor);
    let resolved_target_id = terminal_alias_targets
        .get(descriptor.id())
        .cloned()
        .unwrap_or_else(|| descriptor.id().clone());
    FrozenThemeTokenEntry::new(descriptor, key, resolved_target_id)
}

fn terminal_alias_targets_by_id(
    descriptors: &[ThemeTokenDescriptor],
) -> BTreeMap<ThemeTokenId, ThemeTokenId> {
    let direct_alias_targets = descriptors
        .iter()
        .filter_map(|descriptor| {
            descriptor
                .alias_target()
                .map(|target_id| (descriptor.id().clone(), target_id.clone()))
        })
        .collect::<BTreeMap<_, _>>();

    let mut terminal_alias_targets = BTreeMap::new();
    for descriptor in descriptors {
        if direct_alias_targets.contains_key(descriptor.id()) {
            let terminal_target = terminal_alias_target(
                descriptor.id(),
                &direct_alias_targets,
                &mut terminal_alias_targets,
                &mut BTreeSet::new(),
            );
            terminal_alias_targets.insert(descriptor.id().clone(), terminal_target);
        }
    }
    terminal_alias_targets
}

fn terminal_alias_target(
    token_id: &ThemeTokenId,
    direct_alias_targets: &BTreeMap<ThemeTokenId, ThemeTokenId>,
    terminal_alias_targets: &mut BTreeMap<ThemeTokenId, ThemeTokenId>,
    resolving_ids: &mut BTreeSet<ThemeTokenId>,
) -> ThemeTokenId {
    if let Some(terminal_target) = terminal_alias_targets.get(token_id) {
        return terminal_target.clone();
    }

    if !resolving_ids.insert(token_id.clone()) {
        return token_id.clone();
    }

    let terminal_target = direct_alias_targets
        .get(token_id)
        .map(|direct_target| {
            terminal_alias_target(
                direct_target,
                direct_alias_targets,
                terminal_alias_targets,
                resolving_ids,
            )
        })
        .unwrap_or_else(|| token_id.clone());
    resolving_ids.remove(token_id);
    terminal_alias_targets.insert(token_id.clone(), terminal_target.clone());
    terminal_target
}

fn fold_theme_token_key(accumulator: u64, key: &ThemeTokenKey) -> u64 {
    fold_bytes(accumulator, key.projection_basis().as_bytes())
}

fn fold_bytes(mut accumulator: u64, bytes: &[u8]) -> u64 {
    for byte in bytes {
        accumulator ^= u64::from(*byte);
        accumulator = accumulator.wrapping_mul(0x0000_0100_0000_01b3);
    }
    accumulator
}
