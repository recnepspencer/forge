pub struct WorthUiVisualInspectionAuthority {
    session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    policy: worth_ui_inspection::UiVisualInspectionPolicy,
}

pub struct UiVisualGeometryGrant {
    session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    scope: UiVisualGrantScope,
}

pub struct UiVisualPixelCaptureGrant {
    session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    scope: UiVisualGrantScope,
}

pub struct UiVisualOverlayGrant {
    session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    scope: UiVisualGrantScope,
}

pub struct UiVisualSnapshotComparisonGrant {
    session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
    scope: UiVisualGrantScope,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiVisualGrantScope {
    disclosure: worth_ui_inspection::UiVisualInspectionDisclosure,
    surfaces: UiVisualGrantSurfaceScope,
    lifetime: UiVisualGrantLifetime,
    maximum_snapshot_count: u8,
    maximum_capture_bytes: u64,
    maximum_retained_pixel_bytes: u64,
    maximum_retained_structural_bytes_per_receipt: u64,
    maximum_retained_structural_bytes_per_session: u64,
    maximum_visible_region_records: u32,
    maximum_hit_test_region_records: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualGrantSurfaceScope {
    RegisteredApplicationSurfaces,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiVisualGrantLifetime {
    ActiveSession,
}

impl WorthUiVisualInspectionAuthority {
    pub(crate) const fn seal(
        session: crate::lifecycle::WorthUiActiveApplicationSessionIdentity,
        policy: worth_ui_inspection::UiVisualInspectionPolicy,
    ) -> Self {
        Self { session, policy }
    }

    pub fn issue_geometry_grant(&self) -> UiVisualGeometryGrant {
        UiVisualGeometryGrant {
            session: self.session,
            scope: UiVisualGrantScope::from_policy(self.policy),
        }
    }

    pub fn issue_pixel_grant(&self) -> UiVisualPixelCaptureGrant {
        UiVisualPixelCaptureGrant {
            session: self.session,
            scope: UiVisualGrantScope::from_policy(self.policy),
        }
    }

    pub fn issue_overlay_grant(&self) -> UiVisualOverlayGrant {
        UiVisualOverlayGrant {
            session: self.session,
            scope: UiVisualGrantScope::from_policy(self.policy),
        }
    }

    pub fn issue_comparison_grant(&self) -> UiVisualSnapshotComparisonGrant {
        UiVisualSnapshotComparisonGrant {
            session: self.session,
            scope: UiVisualGrantScope::from_policy(self.policy),
        }
    }

    pub const fn policy(&self) -> worth_ui_inspection::UiVisualInspectionPolicy {
        self.policy
    }
}

impl UiVisualGeometryGrant {
    pub(crate) const fn session(
        &self,
    ) -> crate::lifecycle::WorthUiActiveApplicationSessionIdentity {
        self.session
    }

    pub const fn scope(&self) -> UiVisualGrantScope {
        self.scope
    }
}

impl UiVisualPixelCaptureGrant {
    pub(crate) const fn session(
        &self,
    ) -> crate::lifecycle::WorthUiActiveApplicationSessionIdentity {
        self.session
    }

    pub const fn scope(&self) -> UiVisualGrantScope {
        self.scope
    }
}

impl UiVisualOverlayGrant {
    pub(crate) const fn session(
        &self,
    ) -> crate::lifecycle::WorthUiActiveApplicationSessionIdentity {
        self.session
    }

    pub const fn session_identity(
        &self,
    ) -> crate::lifecycle::WorthUiActiveApplicationSessionIdentity {
        self.session
    }

    pub const fn scope(&self) -> UiVisualGrantScope {
        self.scope
    }
}

impl UiVisualSnapshotComparisonGrant {
    pub(crate) const fn session(
        &self,
    ) -> crate::lifecycle::WorthUiActiveApplicationSessionIdentity {
        self.session
    }

    pub const fn scope(&self) -> UiVisualGrantScope {
        self.scope
    }
}

impl UiVisualGrantScope {
    const fn from_policy(policy: worth_ui_inspection::UiVisualInspectionPolicy) -> Self {
        Self {
            disclosure: policy.disclosure(),
            surfaces: UiVisualGrantSurfaceScope::RegisteredApplicationSurfaces,
            lifetime: UiVisualGrantLifetime::ActiveSession,
            maximum_snapshot_count: policy.maximum_snapshot_count(),
            maximum_capture_bytes: policy.maximum_capture_bytes(),
            maximum_retained_pixel_bytes: policy.maximum_retained_pixel_bytes(),
            maximum_retained_structural_bytes_per_receipt: policy
                .maximum_retained_structural_bytes_per_receipt(),
            maximum_retained_structural_bytes_per_session: policy
                .maximum_retained_structural_bytes_per_session(),
            maximum_visible_region_records: policy.maximum_visible_region_records(),
            maximum_hit_test_region_records: policy.maximum_hit_test_region_records(),
        }
    }

    pub const fn audience(self) -> worth_ui_inspection::UiVisualInspectionAudience {
        self.disclosure.audience()
    }

    pub const fn disclosure(self) -> worth_ui_inspection::UiVisualInspectionDisclosure {
        self.disclosure
    }

    pub const fn surfaces(self) -> UiVisualGrantSurfaceScope {
        self.surfaces
    }

    pub const fn lifetime(self) -> UiVisualGrantLifetime {
        self.lifetime
    }

    pub const fn maximum_snapshot_count(self) -> u8 {
        self.maximum_snapshot_count
    }

    pub const fn maximum_capture_bytes(self) -> u64 {
        self.maximum_capture_bytes
    }

    pub const fn maximum_retained_pixel_bytes(self) -> u64 {
        self.maximum_retained_pixel_bytes
    }

    pub const fn maximum_retained_structural_bytes_per_receipt(self) -> u64 {
        self.maximum_retained_structural_bytes_per_receipt
    }

    pub const fn maximum_retained_structural_bytes_per_session(self) -> u64 {
        self.maximum_retained_structural_bytes_per_session
    }

    pub const fn maximum_visible_region_records(self) -> u32 {
        self.maximum_visible_region_records
    }

    pub const fn maximum_hit_test_region_records(self) -> u32 {
        self.maximum_hit_test_region_records
    }
}
