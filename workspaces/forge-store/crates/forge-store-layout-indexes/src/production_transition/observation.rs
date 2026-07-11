use std::cell::RefCell;

use super::S8LayoutProductionTransition;

thread_local! {
    static ISSUED: RefCell<Option<Vec<S8LayoutProductionTransition>>> = const {
        RefCell::new(None)
    };
}

pub(super) fn record(transition: S8LayoutProductionTransition) {
    ISSUED.with(|issued| {
        if let Some(observed) = issued.borrow_mut().as_mut() {
            if !observed.contains(&transition) {
                observed.push(transition);
            }
        }
    });
}

pub(crate) fn capture_issued_transitions(
    exercise: impl FnOnce(),
) -> Vec<S8LayoutProductionTransition> {
    ISSUED.with(|issued| {
        assert!(
            issued.borrow().is_none(),
            "owner transition capture is nested"
        );
        *issued.borrow_mut() = Some(Vec::new());
    });
    exercise();
    ISSUED.with(|issued| {
        issued
            .borrow_mut()
            .take()
            .expect("owner transition capture must remain active")
    })
}
