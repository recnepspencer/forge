/// Mechanical egui event families visible at the production raw-input seam.
///
/// This is reachability evidence only. It carries no target, interaction,
/// intent, admission, or execution authority.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiEguiRawInputReachability {
    event_count: usize,
    pointer_button_events: usize,
    keyboard_events: usize,
    text_events: usize,
    ime_preedit_events: usize,
    ime_commit_events: usize,
    ime_cancel_events: usize,
}

impl UiEguiRawInputReachability {
    pub(super) fn inspect(raw_input: &egui::RawInput) -> Self {
        let mut reachability = Self {
            event_count: raw_input.events.len(),
            ..Self::default()
        };
        for event in &raw_input.events {
            reachability.observe(event);
        }
        reachability
    }

    pub const fn event_count(self) -> usize {
        self.event_count
    }

    pub const fn pointer_button_events(self) -> usize {
        self.pointer_button_events
    }

    pub const fn keyboard_events(self) -> usize {
        self.keyboard_events
    }

    pub const fn text_events(self) -> usize {
        self.text_events
    }

    pub const fn ime_preedit_events(self) -> usize {
        self.ime_preedit_events
    }

    pub const fn ime_commit_events(self) -> usize {
        self.ime_commit_events
    }

    pub const fn ime_cancel_events(self) -> usize {
        self.ime_cancel_events
    }

    fn observe(&mut self, event: &egui::Event) {
        match event {
            egui::Event::PointerButton { .. } => self.pointer_button_events += 1,
            egui::Event::Key { .. } => self.keyboard_events += 1,
            egui::Event::Text(_) => self.text_events += 1,
            egui::Event::Ime(egui::ImeEvent::Preedit(_)) => self.ime_preedit_events += 1,
            egui::Event::Ime(egui::ImeEvent::Commit(_)) => self.ime_commit_events += 1,
            egui::Event::Ime(egui::ImeEvent::Disabled) => self.ime_cancel_events += 1,
            _ => {}
        }
    }
}

/// Raw input reached the adapter, but no production translator is installed.
///
/// Phase 2 adds translated outcomes only with the concrete translator and
/// capability proof, forcing exhaustive consumers to update at that cutover.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use]
pub enum UiEguiRawInputIngressOutcome {
    Unsupported(UiEguiRawInputReachability),
}
