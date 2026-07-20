use std::collections::BTreeSet;

use super::super::*;

impl Drop for WorthQueryRuntime {
    fn drop(&mut self) {
        let registered_names = self
            .live_subscriptions
            .keys()
            .chain(self.materialized_read_views.keys())
            .map(|target| target.view_name().to_string())
            .collect::<BTreeSet<_>>();
        for name in registered_names {
            let _ = self.backend.close_live_view(&name);
        }
    }
}
