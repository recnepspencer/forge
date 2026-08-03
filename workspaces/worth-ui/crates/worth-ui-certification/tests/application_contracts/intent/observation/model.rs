use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ModelTarget {
    Front,
    Outer,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ModelEvent {
    Press {
        pointer: u64,
        capture: u64,
        target: ModelTarget,
    },
    Motion {
        pointer: u64,
        capture: u64,
    },
    Release {
        pointer: u64,
        capture: u64,
        target: ModelTarget,
    },
    FocusLoss,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum ModelStop {
    Capacity,
    CaptureChanged,
    FocusLost,
    NoActiveGesture,
    TargetChanged,
    Targeting,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct ModelVerdict {
    pub(super) pressed: usize,
    pub(super) semantics: usize,
    pub(super) stops: usize,
    pub(super) stop: Option<ModelStop>,
    pub(super) active: usize,
}

#[derive(Clone, Copy)]
struct ActivePress {
    capture: u64,
    target: ModelTarget,
}

pub(super) struct IndependentGestureModel {
    capacity: usize,
    active: BTreeMap<u64, ActivePress>,
}

impl IndependentGestureModel {
    pub(super) fn new(capacity: usize) -> Self {
        Self {
            capacity,
            active: BTreeMap::new(),
        }
    }

    pub(super) fn step(&mut self, event: ModelEvent) -> ModelVerdict {
        let mut verdict = match event {
            ModelEvent::Press {
                pointer,
                capture,
                target,
            } => self.press(pointer, capture, target),
            ModelEvent::Motion { pointer, capture } => self.motion(pointer, capture),
            ModelEvent::Release {
                pointer,
                capture,
                target,
            } => self.release(pointer, capture, target),
            ModelEvent::FocusLoss => {
                let stops = self.active.len();
                self.active.clear();
                ModelVerdict {
                    stops,
                    stop: (stops > 0).then_some(ModelStop::FocusLost),
                    ..Default::default()
                }
            }
        };
        verdict.active = self.active.len();
        verdict
    }

    pub(super) fn settle_all(&mut self) -> ModelVerdict {
        self.step(ModelEvent::FocusLoss)
    }

    fn press(&mut self, pointer: u64, capture: u64, target: ModelTarget) -> ModelVerdict {
        if target == ModelTarget::None {
            return ModelVerdict {
                stops: 1,
                stop: Some(ModelStop::Targeting),
                ..Default::default()
            };
        }
        if self.active.len() >= self.capacity {
            return ModelVerdict {
                stops: 1,
                stop: Some(ModelStop::Capacity),
                ..Default::default()
            };
        }
        self.active.insert(pointer, ActivePress { capture, target });
        ModelVerdict {
            pressed: 1,
            ..Default::default()
        }
    }

    fn motion(&mut self, pointer: u64, capture: u64) -> ModelVerdict {
        let capture_changed = self
            .active
            .get(&pointer)
            .is_some_and(|active| active.capture != capture);
        if capture_changed {
            self.active.remove(&pointer);
            return ModelVerdict {
                stops: 1,
                stop: Some(ModelStop::CaptureChanged),
                ..Default::default()
            };
        }
        ModelVerdict::default()
    }

    fn release(&mut self, pointer: u64, capture: u64, target: ModelTarget) -> ModelVerdict {
        let Some(active) = self.active.remove(&pointer) else {
            return ModelVerdict {
                stops: 1,
                stop: Some(ModelStop::NoActiveGesture),
                ..Default::default()
            };
        };
        if active.capture == capture && active.target == target && target != ModelTarget::None {
            ModelVerdict {
                semantics: 1,
                ..Default::default()
            }
        } else {
            ModelVerdict {
                stops: 1,
                stop: Some(ModelStop::TargetChanged),
                ..Default::default()
            }
        }
    }
}
