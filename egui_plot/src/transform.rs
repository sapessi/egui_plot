use std::ops::RangeInclusive;

use egui::{Pos2, Rect, Vec2, Vec2b, pos2, remap};

use crate::Axis;

use super::PlotPoint;

/// 2D bounding box of f64 precision.
///
/// The range of data values we show.
#[derive(Clone, Copy, PartialEq, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PlotBounds {
    pub(crate) min: [f64; 2],
    pub(crate) max: [f64; 2],
}

impl PlotBounds {
    pub const NOTHING: Self = Self {
        min: [f64::INFINITY; 2],
        max: [-f64::INFINITY; 2],
    };

    #[inline]
    pub fn from_min_max(min: [f64; 2], max: [f64; 2]) -> Self {
        Self { min, max }
    }

    #[inline]
    pub fn min(&self) -> [f64; 2] {
        self.min
    }

    #[inline]
    pub fn max(&self) -> [f64; 2] {
        self.max
    }

    #[inline]
    pub fn new_symmetrical(half_extent: f64) -> Self {
        Self {
            min: [-half_extent; 2],
            max: [half_extent; 2],
        }
    }

    #[inline]
    pub fn is_finite(&self) -> bool {
        self.min[0].is_finite()
            && self.min[1].is_finite()
            && self.max[0].is_finite()
            && self.max[1].is_finite()
    }

    #[inline]
    pub fn is_finite_x(&self) -> bool {
        self.min[0].is_finite() && self.max[0].is_finite()
    }

    #[inline]
    pub fn is_finite_y(&self) -> bool {
        self.min[1].is_finite() && self.max[1].is_finite()
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        self.is_finite() && self.width() > 0.0 && self.height() > 0.0
    }

    #[inline]
    pub fn is_valid_x(&self) -> bool {
        self.is_finite_x() && self.width() > 0.0
    }

    #[inline]
    pub fn is_valid_y(&self) -> bool {
        self.is_finite_y() && self.height() > 0.0
    }

    #[inline]
    pub fn width(&self) -> f64 {
        self.max[0] - self.min[0]
    }

    #[inline]
    pub fn height(&self) -> f64 {
        self.max[1] - self.min[1]
    }

    #[inline]
    pub fn center(&self) -> PlotPoint {
        [
            (self.min[0] + self.max[0]) / 2.0,
            (self.min[1] + self.max[1]) / 2.0,
        ]
        .into()
    }

    /// Expand to include the given (x,y) value
    #[inline]
    pub fn extend_with(&mut self, value: &PlotPoint) {
        self.extend_with_x(value.x);
        self.extend_with_y(value.y);
    }

    /// Expand to include the given x coordinate
    #[inline]
    pub fn extend_with_x(&mut self, x: f64) {
        self.min[0] = self.min[0].min(x);
        self.max[0] = self.max[0].max(x);
    }

    /// Expand to include the given y coordinate
    #[inline]
    pub fn extend_with_y(&mut self, y: f64) {
        self.min[1] = self.min[1].min(y);
        self.max[1] = self.max[1].max(y);
    }

    #[inline]
    fn clamp_to_finite(&mut self) {
        for d in 0..2 {
            self.min[d] = self.min[d].clamp(f64::MIN, f64::MAX);
            if self.min[d].is_nan() {
                self.min[d] = 0.0;
            }

            self.max[d] = self.max[d].clamp(f64::MIN, f64::MAX);
            if self.max[d].is_nan() {
                self.max[d] = 0.0;
            }
        }
    }

    #[inline]
    pub fn expand_x(&mut self, pad: f64) {
        if pad.is_finite() {
            self.min[0] -= pad;
            self.max[0] += pad;
            self.clamp_to_finite();
        }
    }

    #[inline]
    pub fn expand_y(&mut self, pad: f64) {
        if pad.is_finite() {
            self.min[1] -= pad;
            self.max[1] += pad;
            self.clamp_to_finite();
        }
    }

    #[inline]
    pub fn merge_x(&mut self, other: &Self) {
        self.min[0] = self.min[0].min(other.min[0]);
        self.max[0] = self.max[0].max(other.max[0]);
    }

    #[inline]
    pub fn merge_y(&mut self, other: &Self) {
        self.min[1] = self.min[1].min(other.min[1]);
        self.max[1] = self.max[1].max(other.max[1]);
    }

    #[inline]
    pub fn set_x(&mut self, other: &Self) {
        self.min[0] = other.min[0];
        self.max[0] = other.max[0];
    }

    #[inline]
    pub fn set_x_center_width(&mut self, x: f64, width: f64) {
        self.min[0] = x - width / 2.0;
        self.max[0] = x + width / 2.0;
    }

    #[inline]
    pub fn set_y(&mut self, other: &Self) {
        self.min[1] = other.min[1];
        self.max[1] = other.max[1];
    }

    #[inline]
    pub fn set_y_center_height(&mut self, y: f64, height: f64) {
        self.min[1] = y - height / 2.0;
        self.max[1] = y + height / 2.0;
    }

    #[inline]
    pub fn merge(&mut self, other: &Self) {
        self.min[0] = self.min[0].min(other.min[0]);
        self.min[1] = self.min[1].min(other.min[1]);
        self.max[0] = self.max[0].max(other.max[0]);
        self.max[1] = self.max[1].max(other.max[1]);
    }

    #[inline]
    pub fn translate_x(&mut self, delta: f64) {
        if delta.is_finite() {
            self.min[0] += delta;
            self.max[0] += delta;
            self.clamp_to_finite();
        }
    }

    #[inline]
    pub fn translate_y(&mut self, delta: f64) {
        if delta.is_finite() {
            self.min[1] += delta;
            self.max[1] += delta;
            self.clamp_to_finite();
        }
    }

    #[inline]
    pub fn translate(&mut self, delta: (f64, f64)) {
        self.translate_x(delta.0);
        self.translate_y(delta.1);
    }

    #[inline]
    pub fn zoom(&mut self, zoom_factor: Vec2, center: PlotPoint) {
        self.min[0] = center.x + (self.min[0] - center.x) / (zoom_factor.x as f64);
        self.max[0] = center.x + (self.max[0] - center.x) / (zoom_factor.x as f64);
        self.min[1] = center.y + (self.min[1] - center.y) / (zoom_factor.y as f64);
        self.max[1] = center.y + (self.max[1] - center.y) / (zoom_factor.y as f64);
    }

    #[inline]
    pub fn add_relative_margin_x(&mut self, margin_fraction: Vec2) {
        let width = self.width().max(0.0);
        self.expand_x(margin_fraction.x as f64 * width);
    }

    #[inline]
    pub fn add_relative_margin_y(&mut self, margin_fraction: Vec2) {
        let height = self.height().max(0.0);
        self.expand_y(margin_fraction.y as f64 * height);
    }

    #[inline]
    pub fn range_x(&self) -> RangeInclusive<f64> {
        self.min[0]..=self.max[0]
    }

    #[inline]
    pub fn range_y(&self) -> RangeInclusive<f64> {
        self.min[1]..=self.max[1]
    }

    #[inline]
    pub fn make_x_symmetrical(&mut self) {
        let x_abs = self.min[0].abs().max(self.max[0].abs());
        self.min[0] = -x_abs;
        self.max[0] = x_abs;
    }

    #[inline]
    pub fn make_y_symmetrical(&mut self) {
        let y_abs = self.min[1].abs().max(self.max[1].abs());
        self.min[1] = -y_abs;
        self.max[1] = y_abs;
    }
}

/// Contains the screen rectangle and the plot bounds and provides methods to transform between them.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug)]
pub struct PlotTransform {
    /// The screen rectangle.
    frame: Rect,

    /// The plot bounds.
    bounds: PlotBounds,

    /// Whether to always center the x-range or y-range of the bounds.
    centered: Vec2b,
}

impl PlotTransform {
    pub fn new(frame: Rect, bounds: PlotBounds, center_axis: impl Into<Vec2b>) -> Self {
        debug_assert!(
            0.0 <= frame.width() && 0.0 <= frame.height(),
            "Bad plot frame: {frame:?}"
        );
        let center_axis = center_axis.into();

        // Since the current Y bounds an affect the final X bounds and vice versa, we need to keep
        // the original version of the `bounds` before we start modifying it.
        let mut new_bounds = bounds;

        // Sanitize bounds.
        //
        // When a given bound axis is "thin" (e.g. width or height is 0) but finite, we center the
        // bounds around that value. If the other axis is "fat", we reuse its extent for the thin
        // axis, and default to +/- 1.0 otherwise.
        if !bounds.is_finite_x() {
            new_bounds.set_x(&PlotBounds::new_symmetrical(1.0));
        } else if bounds.width() <= 0.0 {
            new_bounds.set_x_center_width(
                bounds.center().x,
                if bounds.is_valid_y() {
                    bounds.height()
                } else {
                    1.0
                },
            );
        };

        if !bounds.is_finite_y() {
            new_bounds.set_y(&PlotBounds::new_symmetrical(1.0));
        } else if bounds.height() <= 0.0 {
            new_bounds.set_y_center_height(
                bounds.center().y,
                if bounds.is_valid_x() {
                    bounds.width()
                } else {
                    1.0
                },
            );
        };

        // Scale axes so that the origin is in the center.
        if center_axis.x {
            new_bounds.make_x_symmetrical();
        };
        if center_axis.y {
            new_bounds.make_y_symmetrical();
        };

        debug_assert!(
            new_bounds.is_valid(),
            "Bad final plot bounds: {new_bounds:?}"
        );

        Self {
            frame,
            bounds: new_bounds,
            centered: center_axis,
        }
    }

    /// ui-space rectangle.
    #[inline]
    pub fn frame(&self) -> &Rect {
        &self.frame
    }

    /// Plot-space bounds.
    #[inline]
    pub fn bounds(&self) -> &PlotBounds {
        &self.bounds
    }

    #[inline]
    pub fn set_bounds(&mut self, bounds: PlotBounds) {
        self.bounds = bounds;
    }

    pub fn translate_bounds(&mut self, mut delta_pos: (f64, f64)) {
        if self.centered.x {
            delta_pos.0 = 0.;
        }
        if self.centered.y {
            delta_pos.1 = 0.;
        }
        delta_pos.0 *= self.dvalue_dpos()[0];
        delta_pos.1 *= self.dvalue_dpos()[1];
        self.bounds.translate((delta_pos.0, delta_pos.1));
    }

    /// Zoom by a relative factor with the given screen position as center.
    pub fn zoom(&mut self, zoom_factor: Vec2, center: Pos2) {
        let center = self.value_from_position(center);

        let mut new_bounds = self.bounds;
        new_bounds.zoom(zoom_factor, center);

        if new_bounds.is_valid() {
            self.bounds = new_bounds;
        }
    }

    pub fn position_from_point_x(&self, value: f64) -> f32 {
        remap(
            value,
            self.bounds.min[0]..=self.bounds.max[0],
            (self.frame.left() as f64)..=(self.frame.right() as f64),
        ) as f32
    }

    pub fn position_from_point_y(&self, value: f64) -> f32 {
        remap(
            value,
            self.bounds.min[1]..=self.bounds.max[1],
            (self.frame.bottom() as f64)..=(self.frame.top() as f64), // negated y axis!
        ) as f32
    }

    /// Screen/ui position from point on plot.
    pub fn position_from_point(&self, value: &PlotPoint) -> Pos2 {
        pos2(
            self.position_from_point_x(value.x),
            self.position_from_point_y(value.y),
        )
    }

    /// Plot point from screen/ui position.
    pub fn value_from_position(&self, pos: Pos2) -> PlotPoint {
        let x = remap(
            pos.x as f64,
            (self.frame.left() as f64)..=(self.frame.right() as f64),
            self.bounds.range_x(),
        );
        let y = remap(
            pos.y as f64,
            (self.frame.bottom() as f64)..=(self.frame.top() as f64), // negated y axis!
            self.bounds.range_y(),
        );
        PlotPoint::new(x, y)
    }

    /// Transform a rectangle of plot values to a screen-coordinate rectangle.
    ///
    /// This typically means that the rect is mirrored vertically (top becomes bottom and vice versa),
    /// since the plot's coordinate system has +Y up, while egui has +Y down.
    pub fn rect_from_values(&self, value1: &PlotPoint, value2: &PlotPoint) -> Rect {
        let pos1 = self.position_from_point(value1);
        let pos2 = self.position_from_point(value2);

        let mut rect = Rect::NOTHING;
        rect.extend_with(pos1);
        rect.extend_with(pos2);
        rect
    }

    /// delta position / delta value = how many ui points per step in the X axis in "plot space"
    pub fn dpos_dvalue_x(&self) -> f64 {
        self.frame.width() as f64 / self.bounds.width()
    }

    /// delta position / delta value = how many ui points per step in the Y axis in "plot space"
    pub fn dpos_dvalue_y(&self) -> f64 {
        -self.frame.height() as f64 / self.bounds.height() // negated y axis!
    }

    /// delta position / delta value = how many ui points per step in "plot space"
    pub fn dpos_dvalue(&self) -> [f64; 2] {
        [self.dpos_dvalue_x(), self.dpos_dvalue_y()]
    }

    /// delta value / delta position = how much ground do we cover in "plot space" per ui point?
    pub fn dvalue_dpos(&self) -> [f64; 2] {
        [1.0 / self.dpos_dvalue_x(), 1.0 / self.dpos_dvalue_y()]
    }

    /// scale.x/scale.y ratio.
    ///
    /// If 1.0, it means the scale factor is the same in both axes.
    fn aspect(&self) -> f64 {
        let rw = self.frame.width() as f64;
        let rh = self.frame.height() as f64;
        (self.bounds.width() / rw) / (self.bounds.height() / rh)
    }

    /// Sets the aspect ratio by expanding the x- or y-axis.
    ///
    /// This never contracts, so we don't miss out on any data.
    pub(crate) fn set_aspect_by_expanding(&mut self, aspect: f64) {
        let current_aspect = self.aspect();

        let epsilon = 1e-5;
        if (current_aspect - aspect).abs() < epsilon {
            // Don't make any changes when the aspect is already almost correct.
            return;
        }

        if current_aspect < aspect {
            self.bounds
                .expand_x((aspect / current_aspect - 1.0) * self.bounds.width() * 0.5);
        } else {
            self.bounds
                .expand_y((current_aspect / aspect - 1.0) * self.bounds.height() * 0.5);
        }
    }

    /// Sets the aspect ratio by changing either the X or Y axis (callers choice).
    pub(crate) fn set_aspect_by_changing_axis(&mut self, aspect: f64, axis: Axis) {
        let current_aspect = self.aspect();

        let epsilon = 1e-5;
        if (current_aspect - aspect).abs() < epsilon {
            // Don't make any changes when the aspect is already almost correct.
            return;
        }

        match axis {
            Axis::X => {
                self.bounds
                    .expand_x((aspect / current_aspect - 1.0) * self.bounds.width() * 0.5);
            }
            Axis::Y => {
                self.bounds
                    .expand_y((current_aspect / aspect - 1.0) * self.bounds.height() * 0.5);
            }
        }
    }
}
