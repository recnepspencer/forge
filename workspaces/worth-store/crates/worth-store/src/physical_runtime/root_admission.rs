use std::{collections::HashSet, sync::Mutex};

use super::{AdmissionError, DeclaredStoreRoot};

static LIVE_DECLARED_ROOTS: Mutex<Option<HashSet<DeclaredStoreRoot>>> = Mutex::new(None);

pub(crate) struct RootAdmission {
    declared_root: DeclaredStoreRoot,
    released: bool,
}

impl RootAdmission {
    pub(crate) fn admit(declared_root: DeclaredStoreRoot) -> Result<Self, AdmissionError> {
        let mut registry = LIVE_DECLARED_ROOTS
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let live_roots = registry.get_or_insert_with(HashSet::new);

        if !live_roots.insert(declared_root.clone()) {
            return Err(AdmissionError::DeclaredRootAlreadyAdmitted(declared_root));
        }

        Ok(Self {
            declared_root,
            released: false,
        })
    }

    pub(crate) fn declared_root(&self) -> &DeclaredStoreRoot {
        &self.declared_root
    }

    pub(crate) fn release_after<Output>(mut self, transition: impl FnOnce() -> Output) -> Output {
        let mut registry = LIVE_DECLARED_ROOTS
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let output = transition();
        remove_live_root(&mut registry, &self.declared_root);
        self.released = true;
        output
    }
}

impl Drop for RootAdmission {
    fn drop(&mut self) {
        if self.released {
            return;
        }
        let mut registry = LIVE_DECLARED_ROOTS
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        remove_live_root(&mut registry, &self.declared_root);
        self.released = true;
    }
}

fn remove_live_root(
    registry: &mut Option<HashSet<DeclaredStoreRoot>>,
    declared_root: &DeclaredStoreRoot,
) {
    let live_roots = registry
        .as_mut()
        .expect("a live root owner must have a process-local registry");
    assert!(
        live_roots.remove(declared_root),
        "a live root owner must release exactly its registered declaration"
    );
    if live_roots.is_empty() {
        *registry = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{sync::mpsc, time::Duration};

    #[test]
    fn terminal_transition_holds_root_registry_until_release_is_publishable() {
        let declared_root = DeclaredStoreRoot::from_validated_path(std::env::temp_dir().join(
            format!("worth-store-c3-linearized-release-{}", std::process::id()),
        ));
        let admitted = RootAdmission::admit(declared_root.clone()).unwrap();
        let (transition_entered_sender, transition_entered_receiver) = mpsc::sync_channel(1);
        let (finish_transition_sender, finish_transition_receiver) = mpsc::sync_channel(1);
        let releasing = std::thread::spawn(move || {
            admitted.release_after(|| {
                transition_entered_sender.send(()).unwrap();
                finish_transition_receiver.recv().unwrap();
            });
        });
        transition_entered_receiver
            .recv_timeout(Duration::from_secs(5))
            .expect("release transition must begin");
        assert!(matches!(
            LIVE_DECLARED_ROOTS.try_lock(),
            Err(std::sync::TryLockError::WouldBlock)
        ));

        let contender_root = declared_root.clone();
        let (contender_sender, contender_receiver) = mpsc::sync_channel(1);
        let contender = std::thread::spawn(move || {
            assert!(contender_sender
                .send(RootAdmission::admit(contender_root))
                .is_ok());
        });
        assert!(matches!(
            contender_receiver.try_recv(),
            Err(mpsc::TryRecvError::Empty)
        ));

        finish_transition_sender.send(()).unwrap();
        releasing.join().unwrap();
        let readmitted = contender_receiver
            .recv_timeout(Duration::from_secs(5))
            .expect("contender must complete after release")
            .expect("terminal release must make the root reusable");
        contender.join().unwrap();
        drop(readmitted);
    }
}
