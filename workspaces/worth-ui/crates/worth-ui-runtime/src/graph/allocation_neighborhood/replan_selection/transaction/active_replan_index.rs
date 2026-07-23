impl super::UiGraphReplanAuthority {
    pub(in crate::graph::allocation_neighborhood) fn remove_admission(
        &mut self,
        admission: &super::UiGraphReplanAdmission,
    ) {
        for target in admission.targets() {
            let node = target.graph_node_identity();
            if let Some(existing) = self.targets_by_node.get(&node) {
                let rows = existing
                    .iter()
                    .filter(|row| row.neighborhood_identity() != target.neighborhood_identity())
                    .cloned()
                    .collect::<Vec<_>>();
                if rows.is_empty() {
                    self.targets_by_node.remove(&node);
                } else {
                    self.targets_by_node.insert(node, rows.into_boxed_slice());
                }
            }
            if let Some(key) = target.generation_key() {
                let digest = key.neighborhood_identity.identity_digest();
                if let Some(existing) = self.generations_by_digest.get(&digest) {
                    let keys = existing
                        .iter()
                        .filter(|candidate| *candidate != &key)
                        .cloned()
                        .collect::<Vec<_>>();
                    if keys.is_empty() {
                        self.generations_by_digest.remove(&digest);
                    } else {
                        self.generations_by_digest
                            .insert(digest, keys.into_boxed_slice());
                    }
                }
            }
        }
    }

    pub(in crate::graph::allocation_neighborhood) fn insert_admission(
        &mut self,
        admission: &super::UiGraphReplanAdmission,
    ) {
        for target in admission.targets() {
            let node = target.graph_node_identity();
            let mut rows = self
                .targets_by_node
                .get(&node)
                .map_or_else(Vec::new, |rows| rows.to_vec());
            rows.retain(|row| row.neighborhood_identity() != target.neighborhood_identity());
            rows.push(target.clone());
            rows.sort_by_key(super::replan_authority::causal_rank);
            self.targets_by_node.insert(node, rows.into_boxed_slice());
            if let Some(key) = target.generation_key() {
                let digest = key.neighborhood_identity.identity_digest();
                let mut keys = self
                    .generations_by_digest
                    .get(&digest)
                    .map_or_else(Vec::new, |keys| keys.to_vec());
                if !keys.contains(&key) {
                    keys.push(key);
                }
                self.generations_by_digest
                    .insert(digest, keys.into_boxed_slice());
            }
        }
    }

    pub(in crate::graph::allocation_neighborhood) fn rebuild_active_targets(&mut self) {
        let targets = self
            .active_neighborhoods
            .iter()
            .flat_map(|(_, entry)| entry.admission.targets())
            .cloned()
            .collect::<Vec<_>>();
        self.replace(targets.iter());
    }
}
