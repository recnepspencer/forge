use worth_ui_host_contract::{UiHostObservationPresentationBasis, UiMountedHitTestMechanic};

use super::UiPresentedFrameBasisRelation;

mod portal_motion;

/// Exact mounting-owned evidence made available to interaction targeting.
pub(crate) struct UiPresentedHitTestBasis {
    presentation: UiHostObservationPresentationBasis,
    relation: UiPresentedFrameBasisRelation,
    rows: Box<[UiPresentedHitTestRow]>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct UiPresentedHitTestRow {
    mounted: UiMountedHitTestMechanic,
    bounds: worth_ui_host_contract::UiMountedCanonicalBox,
    clip_bounds: worth_ui_host_contract::UiMountedCanonicalBox,
    portal_motion_target: Option<crate::runtime::motion::UiMotionTargetIdentity>,
    owns_presented_portal: bool,
}

impl UiPresentedHitTestBasis {
    pub(crate) fn new(
        presentation: UiHostObservationPresentationBasis,
        relation: UiPresentedFrameBasisRelation,
        rows: Box<[crate::mounting::UiMountedHitTestPresentation]>,
    ) -> Self {
        Self {
            presentation,
            relation,
            rows: rows
                .into_vec()
                .into_iter()
                .map(UiPresentedHitTestRow::from_mounted)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        }
    }

    pub(crate) const fn presentation(&self) -> UiHostObservationPresentationBasis {
        self.presentation
    }

    pub(crate) const fn relation(&self) -> UiPresentedFrameBasisRelation {
        self.relation
    }

    pub(crate) fn rows(&self) -> &[UiPresentedHitTestRow] {
        &self.rows
    }

    pub(crate) fn apply_motion_samples(
        &mut self,
        sampler: &crate::mounting::presentation::motion_sampling::UiMountedMotionSampler,
    ) {
        self.rows = self
            .rows
            .iter()
            .filter_map(|row| row.with_current_motion(sampler, self.presentation))
            .collect::<Vec<_>>()
            .into_boxed_slice();
    }
}

impl UiPresentedHitTestRow {
    fn from_mounted(presentation: crate::mounting::UiMountedHitTestPresentation) -> Self {
        let mounted = presentation.mechanic();
        let portal_target = presentation.portal().map(portal_motion_target);
        Self {
            bounds: mounted.bounds(),
            clip_bounds: mounted.clip_bounds(),
            mounted,
            portal_motion_target: portal_target,
            owns_presented_portal: presentation.owns_presented_portal(),
        }
    }

    fn with_current_motion(
        self,
        sampler: &crate::mounting::presentation::motion_sampling::UiMountedMotionSampler,
        presentation: UiHostObservationPresentationBasis,
    ) -> Option<Self> {
        let sample = self.portal_motion_target.map_or_else(
            || {
                sampler
                    .current_sample_for(self.mounted.mounted_instance(), presentation)
                    .filter(|_| !self.owns_presented_portal)
            },
            |target| sampler.current_sample_for_target(target, presentation),
        );
        let Some(sample) = sample else {
            return Some(self);
        };
        if !sample.hit_test_visible() {
            return None;
        }
        let sampled = sample.geometry()?;
        let (bounds, clip_bounds) = if self.portal_motion_target.is_some() {
            let source = sample.base_geometry()?;
            (
                portal_motion::transform_presented_box(self.bounds, source, sampled),
                portal_motion::transform_presented_box(self.clip_bounds, source, sampled),
            )
        } else {
            (
                canonicalize_sampled_geometry(sampled, self.bounds.coordinate_space()),
                self.clip_bounds,
            )
        };
        Some(Self {
            bounds,
            clip_bounds,
            ..self
        })
    }

    pub(crate) const fn mounted(self) -> UiMountedHitTestMechanic {
        self.mounted
    }
    pub(crate) const fn bounds(self) -> worth_ui_host_contract::UiMountedCanonicalBox {
        self.bounds
    }
    pub(crate) const fn clip_bounds(self) -> worth_ui_host_contract::UiMountedCanonicalBox {
        self.clip_bounds
    }
    pub(crate) const fn order(self) -> worth_ui_host_contract::UiMountedHitTestOrder {
        self.mounted.order()
    }
    pub(crate) const fn mounted_instance(
        self,
    ) -> worth_ui_host_contract::UiMountedInstanceIdentity {
        self.mounted.mounted_instance()
    }
    pub(crate) const fn node_receipt(self) -> worth_ui_host_contract::UiMountedNodeReceiptIdentity {
        self.mounted.node_receipt()
    }
}

fn portal_motion_target(
    portal: worth_ui_host_contract::UiMountedPortalOverlayMechanic,
) -> crate::runtime::motion::UiMotionTargetIdentity {
    crate::runtime::motion::UiMotionTargetIdentity::from_family_owner(
        portal.surface(),
        portal.owner(),
        portal.portal_identity(),
    )
}

fn canonicalize_sampled_geometry(
    geometry: crate::mounting::presentation::motion_sampling::UiPresentationSampledGeometry,
    coordinate_space: worth_ui_host_contract::UiMountedCoordinateSpace,
) -> worth_ui_host_contract::UiMountedCanonicalBox {
    let components = geometry.components();
    worth_ui_host_contract::UiMountedCanonicalBox::canonicalize(
        worth_ui_host_contract::UiMountedCanonicalBoxInput {
            x: components[0],
            y: components[1],
            width: components[2],
            height: components[3],
            coordinate_space,
        },
    )
    .expect("validated presentation-sampled geometry remains canonical at hit-test projection")
}

#[cfg(test)]
pub(crate) fn motion_sampling_hit_test_mechanic_for_test(
    presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    target: crate::runtime::motion::UiMotionTargetIdentity,
    components: [f32; 4],
) -> worth_ui_host_contract::UiMountedHitTestMechanic {
    let bounds = worth_ui_host_contract::UiMountedCanonicalBox::canonicalize(
        worth_ui_host_contract::UiMountedCanonicalBoxInput {
            x: components[0],
            y: components[1],
            width: components[2],
            height: components[3],
            coordinate_space: worth_ui_host_contract::UiMountedCoordinateSpace::Viewport,
        },
    )
    .expect("test hit-test geometry is canonical");
    let receipt =
        worth_ui_host_contract::UiMountedNodeReceiptIssuer::mint_for(presentation.frame())
            .expect("test frame identity is non-zero")
            .receipt_for(target.mounted_instance());
    worth_ui_host_contract::UiMountedHitTestMechanic::complete_from_runtime_mounting(
        worth_ui_host_contract::UiMountedHitTestCompletionInput {
            frame: presentation.frame(),
            surface: target.semantic_surface(),
            binding: presentation.binding(),
            mounted_instance: target.mounted_instance(),
            node_receipt: receipt,
            bounds,
            clip_bounds: bounds,
            order: worth_ui_host_contract::UiMountedHitTestOrder::from_runtime_plan(1),
        },
    )
    .expect("test hit-test mechanic is coherent")
}
