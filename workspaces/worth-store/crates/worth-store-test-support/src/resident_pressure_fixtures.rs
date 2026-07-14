#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LargeStorePressureClass {
    BarelyOverBudget,
    ModeratelyOverBudget,
    FarOverBudget,
    FragmentedPressure,
    ProtectedPressure,
    StreamingPressure,
}

impl LargeStorePressureClass {
    pub const ALL: [Self; 6] = [
        Self::BarelyOverBudget,
        Self::ModeratelyOverBudget,
        Self::FarOverBudget,
        Self::FragmentedPressure,
        Self::ProtectedPressure,
        Self::StreamingPressure,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BarelyOverBudget => "barely_over_budget",
            Self::ModeratelyOverBudget => "moderately_over_budget",
            Self::FarOverBudget => "far_over_budget",
            Self::FragmentedPressure => "fragmented_pressure",
            Self::ProtectedPressure => "protected_pressure",
            Self::StreamingPressure => "streaming_pressure",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LargeStorePressureFixture {
    class: LargeStorePressureClass,
    declared_store_bytes: u64,
    resident_budget_bytes: u64,
    page_count: u64,
    resident_frame_count: u64,
    fragment_count: u64,
    protected_page_count: u64,
    streaming_window_bytes: u64,
}

impl LargeStorePressureFixture {
    pub const fn for_class(class: LargeStorePressureClass) -> Self {
        match class {
            LargeStorePressureClass::BarelyOverBudget => Self::new(FixtureShape {
                class,
                declared_store_bytes: 10 * 1024,
                resident_budget_bytes: 8 * 1024,
                page_count: 10,
                resident_frame_count: 8,
                fragment_count: 1,
                protected_page_count: 0,
                streaming_window_bytes: 1024,
            }),
            LargeStorePressureClass::ModeratelyOverBudget => Self::new(FixtureShape {
                class,
                declared_store_bytes: 32 * 1024,
                resident_budget_bytes: 8 * 1024,
                page_count: 32,
                resident_frame_count: 8,
                fragment_count: 2,
                protected_page_count: 0,
                streaming_window_bytes: 1024,
            }),
            LargeStorePressureClass::FarOverBudget => Self::new(FixtureShape {
                class,
                declared_store_bytes: 256 * 1024,
                resident_budget_bytes: 8 * 1024,
                page_count: 256,
                resident_frame_count: 8,
                fragment_count: 4,
                protected_page_count: 0,
                streaming_window_bytes: 1024,
            }),
            LargeStorePressureClass::FragmentedPressure => Self::new(FixtureShape {
                class,
                declared_store_bytes: 64 * 1024,
                resident_budget_bytes: 8 * 1024,
                page_count: 64,
                resident_frame_count: 8,
                fragment_count: 16,
                protected_page_count: 0,
                streaming_window_bytes: 1024,
            }),
            LargeStorePressureClass::ProtectedPressure => Self::new(FixtureShape {
                class,
                declared_store_bytes: 64 * 1024,
                resident_budget_bytes: 8 * 1024,
                page_count: 64,
                resident_frame_count: 8,
                fragment_count: 4,
                protected_page_count: 8,
                streaming_window_bytes: 1024,
            }),
            LargeStorePressureClass::StreamingPressure => Self::new(FixtureShape {
                class,
                declared_store_bytes: 128 * 1024,
                resident_budget_bytes: 8 * 1024,
                page_count: 128,
                resident_frame_count: 8,
                fragment_count: 8,
                protected_page_count: 0,
                streaming_window_bytes: 2048,
            }),
        }
    }

    const fn new(shape: FixtureShape) -> Self {
        Self {
            class: shape.class,
            declared_store_bytes: shape.declared_store_bytes,
            resident_budget_bytes: shape.resident_budget_bytes,
            page_count: shape.page_count,
            resident_frame_count: shape.resident_frame_count,
            fragment_count: shape.fragment_count,
            protected_page_count: shape.protected_page_count,
            streaming_window_bytes: shape.streaming_window_bytes,
        }
    }

    pub const fn class(&self) -> LargeStorePressureClass {
        self.class
    }

    pub const fn declared_store_bytes(&self) -> u64 {
        self.declared_store_bytes
    }

    pub const fn resident_budget_bytes(&self) -> u64 {
        self.resident_budget_bytes
    }

    pub const fn page_count(&self) -> u64 {
        self.page_count
    }

    pub const fn resident_frame_count(&self) -> u64 {
        self.resident_frame_count
    }

    pub const fn fragment_count(&self) -> u64 {
        self.fragment_count
    }

    pub const fn protected_page_count(&self) -> u64 {
        self.protected_page_count
    }

    pub const fn streaming_window_bytes(&self) -> u64 {
        self.streaming_window_bytes
    }

    pub const fn allocation_envelope_bytes(&self) -> u64 {
        self.resident_budget_bytes + self.streaming_window_bytes
    }

    pub const fn persisted_exceeds_budget(&self) -> bool {
        self.declared_store_bytes > self.resident_budget_bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FixtureShape {
    class: LargeStorePressureClass,
    declared_store_bytes: u64,
    resident_budget_bytes: u64,
    page_count: u64,
    resident_frame_count: u64,
    fragment_count: u64,
    protected_page_count: u64,
    streaming_window_bytes: u64,
}
