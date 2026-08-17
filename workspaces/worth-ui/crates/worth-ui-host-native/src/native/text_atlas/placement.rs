//! Deterministic bounded page placement for native text atlases.

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct UiAtlasRect {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl UiAtlasRect {
    pub(crate) const fn area(self) -> u64 {
        self.width as u64 * self.height as u64
    }

    const fn contains(self, width: u32, height: u32) -> bool {
        width <= self.width && height <= self.height
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiAtlasPage {
    width: u32,
    height: u32,
    free: Vec<UiAtlasRect>,
}

impl UiAtlasPage {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            free: vec![UiAtlasRect {
                x: 0,
                y: 0,
                width,
                height,
            }],
        }
    }

    pub(crate) fn allocate(&mut self, width: u32, height: u32) -> Option<UiAtlasRect> {
        let index = self
            .free
            .iter()
            .enumerate()
            .filter(|(_, block)| block.contains(width, height))
            .min_by_key(|(_, block)| (block.area(), block.y, block.x, block.width, block.height))
            .map(|(index, _)| index)?;
        let block = self.free.swap_remove(index);
        let placed = UiAtlasRect {
            x: block.x,
            y: block.y,
            width,
            height,
        };
        if block.width > width {
            self.free.push(UiAtlasRect {
                x: block.x + width,
                y: block.y,
                width: block.width - width,
                height,
            });
        }
        if block.height > height {
            self.free.push(UiAtlasRect {
                x: block.x,
                y: block.y + height,
                width: block.width,
                height: block.height - height,
            });
        }
        self.normalize_free_blocks();
        Some(placed)
    }

    pub(crate) fn release(&mut self, rect: UiAtlasRect) {
        self.free.push(rect);
        self.normalize_free_blocks();
    }

    #[cfg(test)]
    pub(crate) fn free_area(&self) -> u64 {
        self.free.iter().map(|rect| rect.area()).sum()
    }

    fn normalize_free_blocks(&mut self) {
        self.free.sort_unstable();
        self.free.dedup();
        let mut changed = true;
        while changed {
            changed = false;
            'outer: for left in 0..self.free.len() {
                for right in (left + 1)..self.free.len() {
                    if let Some(merged) = merge_rects(self.free[left], self.free[right]) {
                        self.free[left] = merged;
                        self.free.swap_remove(right);
                        self.free.sort_unstable();
                        changed = true;
                        break 'outer;
                    }
                }
            }
        }
    }
}

fn merge_rects(left: UiAtlasRect, right: UiAtlasRect) -> Option<UiAtlasRect> {
    if left.y == right.y && left.height == right.height {
        if left.x + left.width == right.x {
            return Some(UiAtlasRect {
                x: left.x,
                y: left.y,
                width: left.width + right.width,
                height: left.height,
            });
        }
        if right.x + right.width == left.x {
            return Some(UiAtlasRect {
                x: right.x,
                y: right.y,
                width: left.width + right.width,
                height: left.height,
            });
        }
    }
    if left.x == right.x && left.width == right.width {
        if left.y + left.height == right.y {
            return Some(UiAtlasRect {
                x: left.x,
                y: left.y,
                width: left.width,
                height: left.height + right.height,
            });
        }
        if right.y + right.height == left.y {
            return Some(UiAtlasRect {
                x: right.x,
                y: right.y,
                width: left.width,
                height: left.height + right.height,
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{UiAtlasPage, UiAtlasRect};

    #[test]
    fn placement_is_stable_and_release_coalesces_exact_siblings() {
        let mut page = UiAtlasPage::new(8, 8);
        let first = page.allocate(4, 4).unwrap();
        let second = page.allocate(4, 4).unwrap();
        assert_eq!(
            first,
            UiAtlasRect {
                x: 0,
                y: 0,
                width: 4,
                height: 4
            }
        );
        assert_eq!(
            second,
            UiAtlasRect {
                x: 4,
                y: 0,
                width: 4,
                height: 4
            }
        );
        page.release(first);
        page.release(second);
        assert_eq!(page.free_area(), 64);
        assert_eq!(
            page.allocate(8, 8),
            Some(UiAtlasRect {
                x: 0,
                y: 0,
                width: 8,
                height: 8
            })
        );
    }
}
