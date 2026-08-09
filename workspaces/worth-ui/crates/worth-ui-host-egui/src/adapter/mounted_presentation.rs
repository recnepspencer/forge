use std::collections::HashMap;

use worth_ui_host_contract::{
    UiHostPresentationCostReport, UiHostSurfacePresentationDenial, UiMountedCompletedEffects,
    UiMountedEffectFamily, UiMountedFrameConsumptionView, UiMountedPaintCommand,
    UiMountedPaintCommandChange, UiMountedPaintCommandIdentity,
    UiMountedPresentationAuxiliaryState, UiMountedPresentationWorkView,
};

#[derive(Clone)]
pub(super) struct UiEguiPreparedMountedPresentation {
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    commands: HashMap<UiMountedPaintCommandIdentity, UiMountedPaintCommand>,
    auxiliary: UiMountedPresentationAuxiliaryState,
    native_paint: super::native_paint::UiEguiPreparedNativePaint,
    identity_overlay: super::identity_overlay::UiEguiPreparedIdentityOverlay,
    native_regions: super::native_regions::UiEguiRetainedNativeRegions,
}

pub(super) struct UiEguiPresentationCandidate {
    pub(super) presentation: UiEguiPreparedMountedPresentation,
    execution: UiEguiPresentationExecution,
}

#[derive(Clone, Copy)]
enum UiEguiPresentationExecution {
    Initial,
    Delta {
        native_paint: bool,
        identity_overlay: bool,
    },
    Unchanged,
}

impl UiEguiPresentationCandidate {
    pub(super) fn prepare(
        context: &egui::Context,
        view: &UiMountedFrameConsumptionView<'_>,
        current: Option<&UiEguiPreparedMountedPresentation>,
    ) -> Result<Self, UiHostSurfacePresentationDenial> {
        match view.presentation_work() {
            UiMountedPresentationWorkView::Initial(initial) => {
                if current.is_some() {
                    return Err(UiHostSurfacePresentationDenial::MalformedProjection);
                }
                deny_unsupported(initial.projection())?;
                let commands = command_map(initial.commands());
                Ok(Self {
                    presentation: UiEguiPreparedMountedPresentation {
                        frame: view.frame(),
                        commands: commands.clone(),
                        auxiliary: initial.auxiliary().clone(),
                        native_paint:
                            super::native_paint::UiEguiPreparedNativePaint::prepare_initial(
                                view, initial,
                            )?,
                        identity_overlay:
                            super::identity_overlay::UiEguiPreparedIdentityOverlay::prepare(
                                context,
                                initial.projection(),
                            )?,
                        native_regions:
                            super::native_regions::UiEguiRetainedNativeRegions::prepare_projection(
                                initial.projection(),
                                &commands,
                                initial.order(),
                            )?,
                    },
                    execution: UiEguiPresentationExecution::Initial,
                })
            }
            UiMountedPresentationWorkView::Delta(delta) => {
                let current = current
                    .filter(|current| delta.affinity().predecessor() == Some(current.frame))
                    .ok_or(UiHostSurfacePresentationDenial::MalformedProjection)?;
                let mut commands = current.commands.clone();
                apply_raw_changes(&mut commands, delta.changes())?;
                let auxiliary = delta
                    .auxiliary()
                    .cloned()
                    .unwrap_or_else(|| current.auxiliary.clone());
                let changed_projection = delta
                    .auxiliary()
                    .map(|_| auxiliary.reconstruct(&commands))
                    .transpose()
                    .map_err(|_| UiHostSurfacePresentationDenial::MalformedProjection)?;
                if let Some(projection) = changed_projection.as_ref() {
                    deny_unsupported(projection)?;
                }
                let native_paint = current.native_paint.apply_delta(view, delta)?;
                let identity_overlay = match changed_projection.as_ref() {
                    Some(projection) => {
                        super::identity_overlay::UiEguiPreparedIdentityOverlay::prepare(
                            context, projection,
                        )?
                    }
                    None => current.identity_overlay.clone(),
                };
                let native_regions = match changed_projection.as_ref() {
                    Some(projection) => {
                        super::native_regions::UiEguiRetainedNativeRegions::prepare_projection(
                            projection,
                            &commands,
                            native_paint.order(),
                        )?
                    }
                    None => current
                        .native_regions
                        .apply_delta(delta, native_paint.order()),
                };
                let native_paint_changed = !delta.changes().is_empty()
                    || !delta.order().is_empty()
                    || !delta.damage().is_empty();
                let identity_overlay_changed = changed_projection.is_some()
                    && (!current.identity_overlay.is_empty() || !identity_overlay.is_empty());
                Ok(Self {
                    presentation: UiEguiPreparedMountedPresentation {
                        frame: view.frame(),
                        commands: commands.clone(),
                        auxiliary,
                        native_paint,
                        identity_overlay,
                        native_regions,
                    },
                    execution: UiEguiPresentationExecution::Delta {
                        native_paint: native_paint_changed,
                        identity_overlay: identity_overlay_changed,
                    },
                })
            }
            UiMountedPresentationWorkView::Unchanged(unchanged) => {
                let current = current
                    .filter(|current| unchanged.affinity().predecessor() == Some(current.frame))
                    .ok_or(UiHostSurfacePresentationDenial::MalformedProjection)?;
                let mut presentation = current.clone();
                presentation.frame = view.frame();
                Ok(Self {
                    presentation,
                    execution: UiEguiPresentationExecution::Unchanged,
                })
            }
        }
    }

    pub(super) fn completed_effects(&self) -> UiMountedCompletedEffects {
        let mut effects = Vec::new();
        match self.execution {
            UiEguiPresentationExecution::Initial => {
                if !self.presentation.native_paint.is_empty() {
                    effects.push(UiMountedEffectFamily::NativePaint);
                }
                if !self.presentation.identity_overlay.is_empty() {
                    effects.push(UiMountedEffectFamily::IdentityOverlay);
                }
            }
            UiEguiPresentationExecution::Delta {
                native_paint,
                identity_overlay,
            } => {
                if native_paint {
                    effects.push(UiMountedEffectFamily::NativePaint);
                }
                if identity_overlay {
                    effects.push(UiMountedEffectFamily::IdentityOverlay);
                }
            }
            UiEguiPresentationExecution::Unchanged => {}
        }
        UiMountedCompletedEffects::new(effects)
    }

    pub(super) fn cost(
        &self,
        view: &UiMountedFrameConsumptionView<'_>,
    ) -> Result<UiHostPresentationCostReport, UiHostSurfacePresentationDenial> {
        super::presentation_cost::for_work(view.presentation_work())
    }

    pub(super) fn paint(&self, context: &egui::Context) {
        match self.execution {
            UiEguiPresentationExecution::Initial => {
                self.presentation.native_paint.paint(context);
                self.presentation.identity_overlay.paint(context);
            }
            UiEguiPresentationExecution::Delta {
                native_paint,
                identity_overlay,
            } => {
                if native_paint {
                    self.presentation.native_paint.paint(context);
                }
                if identity_overlay {
                    self.presentation.identity_overlay.paint(context);
                }
            }
            UiEguiPresentationExecution::Unchanged => {}
        }
    }

    pub(super) fn realized_regions(&self) -> Vec<worth_ui_host_contract::UiHostRealizedRegion> {
        self.presentation.native_regions.realized()
    }
}

impl UiEguiPreparedMountedPresentation {
    pub(super) fn paint(&self, context: &egui::Context) {
        self.native_paint.paint(context);
        self.identity_overlay.paint(context);
    }
}

fn command_map(
    commands: &[UiMountedPaintCommand],
) -> HashMap<UiMountedPaintCommandIdentity, UiMountedPaintCommand> {
    commands
        .iter()
        .cloned()
        .map(|command| (command.identity(), command))
        .collect()
}

fn apply_raw_changes(
    commands: &mut HashMap<UiMountedPaintCommandIdentity, UiMountedPaintCommand>,
    changes: &[UiMountedPaintCommandChange],
) -> Result<(), UiHostSurfacePresentationDenial> {
    for change in changes {
        match change {
            UiMountedPaintCommandChange::Insert(command) => {
                if commands
                    .insert(command.identity(), command.clone())
                    .is_some()
                {
                    return Err(UiHostSurfacePresentationDenial::MalformedProjection);
                }
            }
            UiMountedPaintCommandChange::Replace(command) => {
                if !commands.contains_key(&command.identity()) {
                    return Err(UiHostSurfacePresentationDenial::MalformedProjection);
                }
                commands.insert(command.identity(), command.clone());
            }
            UiMountedPaintCommandChange::Remove(identity) => {
                if commands.remove(identity).is_none() {
                    return Err(UiHostSurfacePresentationDenial::MalformedProjection);
                }
            }
        }
    }
    Ok(())
}

fn deny_unsupported(
    projection: &worth_ui_host_contract::UiMountedProjectionView,
) -> Result<(), UiHostSurfacePresentationDenial> {
    if let Some(effect) = super::mounted_effect_support::unsupported_projection_effect(projection) {
        return Err(UiHostSurfacePresentationDenial::UnsupportedEffect(effect));
    }
    Ok(())
}
