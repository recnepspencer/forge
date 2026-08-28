#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiScrollRevealAlignment {
    Nearest,
    Start,
    Center,
    End,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiScrollRevealInterval {
    start_subpixels: i64,
    end_subpixels: i64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiScrollRevealTarget {
    inline: UiScrollRevealInterval,
    block: UiScrollRevealInterval,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiScrollViewportExtent {
    inline_subpixels: i64,
    block_subpixels: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiScrollProgrammaticRevealRequest {
    chain: Vec<super::UiScrollChainEntry>,
    target: UiScrollRevealTarget,
    viewport: UiScrollViewportExtent,
    alignment: UiScrollRevealAlignment,
}

impl UiScrollRevealInterval {
    pub(crate) const fn new(start_subpixels: i64, end_subpixels: i64) -> Option<Self> {
        if start_subpixels < 0 || end_subpixels < start_subpixels {
            None
        } else {
            Some(Self {
                start_subpixels,
                end_subpixels,
            })
        }
    }
}

impl UiScrollRevealTarget {
    pub(crate) const fn new(inline: UiScrollRevealInterval, block: UiScrollRevealInterval) -> Self {
        Self { inline, block }
    }
}

impl UiScrollViewportExtent {
    pub(crate) const fn new(inline_subpixels: i64, block_subpixels: i64) -> Option<Self> {
        if inline_subpixels <= 0 || block_subpixels <= 0 {
            None
        } else {
            Some(Self {
                inline_subpixels,
                block_subpixels,
            })
        }
    }
}

impl UiScrollProgrammaticRevealRequest {
    pub(crate) fn new(
        chain: Vec<super::UiScrollChainEntry>,
        target: UiScrollRevealTarget,
        viewport: UiScrollViewportExtent,
        alignment: UiScrollRevealAlignment,
    ) -> Result<Self, super::UiScrollRouteDenial> {
        super::UiScrollDeltaRequest::new(
            chain.clone(),
            super::UiScrollDelta::new(0, 0),
            super::UiScrollDeltaCause::ProgrammaticReveal,
        )?;
        Ok(Self {
            chain,
            target,
            viewport,
            alignment,
        })
    }

    pub(super) fn chain(&self) -> &[super::UiScrollChainEntry] {
        &self.chain
    }
}

impl super::UiScrollRuntimeState {
    pub(crate) fn reveal(
        &mut self,
        request: UiScrollProgrammaticRevealRequest,
    ) -> Result<super::UiScrollRouteReceipt, super::UiScrollRouteDenial> {
        let first = request.chain()[0];
        let (current, bounds, axes) = self.owner_geometry(first.owner(), first.incarnation())?;
        let desired = desired_offset(
            current,
            bounds,
            axes,
            request.target,
            request.viewport,
            request.alignment,
        );
        let delta = super::UiScrollDelta::new(
            desired.inline_subpixels() - current.inline_subpixels(),
            desired.block_subpixels() - current.block_subpixels(),
        );
        self.route(super::UiScrollDeltaRequest::new(
            request.chain,
            delta,
            super::UiScrollDeltaCause::ProgrammaticReveal,
        )?)
    }
}

fn desired_offset(
    current: super::UiScrollOffset,
    bounds: super::UiScrollBounds,
    axes: super::UiScrollAxes,
    target: UiScrollRevealTarget,
    viewport: UiScrollViewportExtent,
    alignment: UiScrollRevealAlignment,
) -> super::UiScrollOffset {
    let inline = if axes.accepts_inline() {
        aligned_axis(
            current.inline_subpixels(),
            target.inline,
            viewport.inline_subpixels,
            alignment,
        )
    } else {
        current.inline_subpixels()
    };
    let block = if axes.accepts_block() {
        aligned_axis(
            current.block_subpixels(),
            target.block,
            viewport.block_subpixels,
            alignment,
        )
    } else {
        current.block_subpixels()
    };
    bounds.clamp(super::UiScrollOffset::new(inline.max(0), block.max(0)).unwrap())
}

fn aligned_axis(
    current: i64,
    target: UiScrollRevealInterval,
    viewport: i64,
    alignment: UiScrollRevealAlignment,
) -> i64 {
    match alignment {
        UiScrollRevealAlignment::Start => target.start_subpixels,
        UiScrollRevealAlignment::End => target.end_subpixels.saturating_sub(viewport),
        UiScrollRevealAlignment::Center => {
            let center = i128::from(target.start_subpixels) + i128::from(target.end_subpixels);
            clamp_i128(center / 2 - i128::from(viewport) / 2)
        }
        UiScrollRevealAlignment::Nearest => {
            let viewport_end = current.saturating_add(viewport);
            if target.start_subpixels < current {
                target.start_subpixels
            } else if target.end_subpixels > viewport_end {
                target.end_subpixels.saturating_sub(viewport)
            } else {
                current
            }
        }
    }
}

fn clamp_i128(value: i128) -> i64 {
    value.clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64
}
