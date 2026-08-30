use worth_ui_host_contract::{
    UiMountedCanonicalBox, UiMountedCanonicalBoxInput, UiMountedCoordinateSpace,
    UiMountedLogicalDamage, UiMountedPresentationOpacity, UiMountedPresentationSampleChange,
    UiMountedPresentationSampleInput, UiMountedPresentationTransform,
};

use super::{production_cost, LocalWorkCost, RetainedTraversalCost, UiMountedPresentationState};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiMountedMotionSampleWorkDenial {
    PresentationBasisMismatch,
    UnknownTargetCommands,
    InvalidGeometry,
    InvalidOpacity,
}

impl UiMountedPresentationState {
    pub(crate) fn issue_motion_sample(
        &self,
        sampling: &crate::mounting::presentation::motion_sampling::UiPresentationMotionSamplingReceipt,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
        lease: &crate::mounting::presentation::UiMountedPresentationLease,
    ) -> Result<
        crate::mounting::presentation::UiMountedPresentationWork,
        UiMountedMotionSampleWorkDenial,
    > {
        if self.frame != presentation.frame()
            || self.requirement.host_surface() != presentation.host_surface()
            || self.requirement.binding() != presentation.binding()
        {
            return Err(UiMountedMotionSampleWorkDenial::PresentationBasisMismatch);
        }
        let mut changes = Vec::new();
        let mut damage = Vec::new();
        for sample in sampling.samples() {
            let portal_group = self.portal_motion_group(sample.target());
            let portal_clip = portal_group
                .as_ref()
                .and_then(|group| group.viewport_clip());
            let identities = portal_group.map_or_else(
                || {
                    self.command_identities_for_instance(sample.target().mounted_instance())
                        .collect::<Vec<_>>()
                },
                |group| group.commands().collect::<Vec<_>>(),
            );
            if identities.is_empty() {
                return Err(UiMountedMotionSampleWorkDenial::UnknownTargetCommands);
            }
            let opacity = UiMountedPresentationOpacity::from_runtime_sampling(sample.opacity())
                .map_err(|_| UiMountedMotionSampleWorkDenial::InvalidOpacity)?;
            for identity in identities {
                let command = self
                    .command_option(identity)
                    .ok_or(UiMountedMotionSampleWorkDenial::UnknownTargetCommands)?;
                let transform =
                    sample_transform(*sample, command.clip_bounds().coordinate_space())?;
                changes.push(UiMountedPresentationSampleChange::from_runtime_sampling(
                    identity, transform, opacity,
                ));
                if portal_clip.is_none() {
                    append_clipped_damage(&mut damage, *sample, command.clip_bounds())?;
                }
            }
            if let Some(portal_clip) = portal_clip {
                append_clipped_damage(&mut damage, *sample, portal_clip)?;
            }
        }
        if changes.is_empty() || damage.is_empty() {
            return Err(UiMountedMotionSampleWorkDenial::UnknownTargetCommands);
        }
        Ok(lease.issue_sample(UiMountedPresentationSampleInput {
            frame: self.frame,
            surface: self.requirement.semantic_surface(),
            binding: self.requirement.binding(),
            content: self.content,
            baseline: self.requirement.baseline(),
            production_cost: production_cost(
                LocalWorkCost {
                    source_instances: sampling.samples().len(),
                    commands_considered: changes.len(),
                    command_index_lookups: sampling.samples().len(),
                    order_lookups: 0,
                },
                RetainedTraversalCost::default(),
                0,
            ),
            changes,
            damage,
        }))
    }

    pub(crate) const fn motion_sample_requirement(
        &self,
    ) -> worth_ui_host_contract::UiMountedSurfaceBindingRequirement {
        self.requirement
    }
}

fn append_clipped_damage(
    damage: &mut Vec<UiMountedLogicalDamage>,
    sample: crate::mounting::presentation::motion_sampling::UiPresentationMotionSampleReceipt,
    clip: UiMountedCanonicalBox,
) -> Result<(), UiMountedMotionSampleWorkDenial> {
    let sampled_clip = clip_geometry(clip)?;
    damage.extend(
        sample
            .damage()
            .clipped_to(sampled_clip)
            .into_iter()
            .flatten()
            .map(|region| {
                logical_damage(region.components(), clip.coordinate_space())
                    .map_err(|_| UiMountedMotionSampleWorkDenial::InvalidGeometry)
            })
            .collect::<Result<Vec<_>, _>>()?,
    );
    Ok(())
}

fn sample_transform(
    sample: crate::mounting::presentation::motion_sampling::UiPresentationMotionSampleReceipt,
    coordinate_space: UiMountedCoordinateSpace,
) -> Result<Option<UiMountedPresentationTransform>, UiMountedMotionSampleWorkDenial> {
    match (sample.base_geometry(), sample.geometry()) {
        (Some(source), Some(sampled)) => {
            let source = canonical_box(source.components(), coordinate_space)
                .map_err(|_| UiMountedMotionSampleWorkDenial::InvalidGeometry)?;
            let sampled = canonical_box(sampled.components(), coordinate_space)
                .map_err(|_| UiMountedMotionSampleWorkDenial::InvalidGeometry)?;
            UiMountedPresentationTransform::from_runtime_sampling(source, sampled)
                .map(Some)
                .map_err(|_| UiMountedMotionSampleWorkDenial::InvalidGeometry)
        }
        (None, None) => Ok(None),
        _ => Err(UiMountedMotionSampleWorkDenial::InvalidGeometry),
    }
}

fn clip_geometry(
    bounds: UiMountedCanonicalBox,
) -> Result<
    crate::mounting::presentation::motion_sampling::UiPresentationSampledClipGeometry,
    UiMountedMotionSampleWorkDenial,
> {
    crate::mounting::presentation::motion_sampling::UiPresentationSampledClipGeometry::from_presented_components([
        bounds.x(), bounds.y(), bounds.width(), bounds.height(),
    ])
    .map_err(|_| UiMountedMotionSampleWorkDenial::InvalidGeometry)
}

fn logical_damage(
    components: [f32; 4],
    coordinate_space: UiMountedCoordinateSpace,
) -> Result<UiMountedLogicalDamage, worth_ui_host_contract::UiMountedGeometryDenial> {
    canonical_box(components, coordinate_space).map(UiMountedLogicalDamage::from_runtime_mounting)
}

fn canonical_box(
    components: [f32; 4],
    coordinate_space: UiMountedCoordinateSpace,
) -> Result<UiMountedCanonicalBox, worth_ui_host_contract::UiMountedGeometryDenial> {
    UiMountedCanonicalBox::canonicalize(UiMountedCanonicalBoxInput {
        x: components[0],
        y: components[1],
        width: components[2],
        height: components[3],
        coordinate_space,
    })
}
