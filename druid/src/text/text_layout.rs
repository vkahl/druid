// Copyright 2020 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! A type for representing text that is displayed on the screen.

use super::TextBuffer;
use crate::kurbo::Vec2;
use crate::piet::{
    FontBuilder as _, PietText, PietTextLayout, Text as _, TextLayout as _, TextLayoutBuilder as _,
};
use crate::{theme, Env, PaintCtx, Point, RenderContext, Size};

pub struct TextLayout {
    text: PietText,
    buffer: TextBuffer,
    // this is optional so that you can create a `TextLayout` before you get passed contexts etc
    layout: LayoutWrapper,
    /// The width for the purpose of line breaks; that is, the width of the view,
    /// not necessarily the width of the current text.
    width: f64,
}

/// A helper type for interacting with piet.
///
/// The goal for this type is to experiment with what sort of underlying API changes
/// we might want to make in piet, which for the time being we can implement on
/// this type, using the existing piet API.
#[derive(Clone, Default)]
struct LayoutWrapper {
    inner: Option<PietTextLayout>,
}

impl TextLayout {
    pub fn new(
        buffer: TextBuffer,
        mut text: PietText,
        env: &Env,
        width: impl Into<Option<f64>>,
    ) -> Self {
        let width = width.into().unwrap_or(f64::INFINITY);
        let layout = LayoutWrapper::for_buffer(&buffer, &mut text, env, width);
        TextLayout {
            text,
            buffer,
            layout,
            width,
        }
    }

    pub fn update_buffer(&mut self, buffer: TextBuffer, env: &Env) {
        self.layout = LayoutWrapper::for_buffer(&buffer, &mut self.text, env, self.width);
        self.buffer = buffer;
    }

    pub fn update_width(&mut self, width: impl Into<Option<f64>>) {
        self.width = width.into().unwrap_or(f64::INFINITY);
        self.layout.update_width(self.width);
    }

    // point is relative to the typographic origin
    pub fn draw(&self, ctx: &mut PaintCtx, point: impl Into<Point>, env: &Env) {
        let y_off = self.layout.first_baseline();
        if let Some(layout) = &self.layout.inner {
            let point = point.into() + Vec2::new(0., y_off);
            let color = env.get(theme::LABEL_COLOR);
            ctx.draw_text(layout, point, &color);
        }
    }

    pub fn size(&self) -> Size {
        self.layout.size()
    }
}

impl LayoutWrapper {
    fn for_buffer(buffer: &TextBuffer, text: &mut PietText, env: &Env, width: f64) -> Self {
        //FIXME: figure out how to resolve these from `TextBuffer`
        let font_name = env.get(theme::FONT_NAME);
        let font_size = env.get(theme::TEXT_SIZE_NORMAL);
        //FIXME: there needs to be a way to get a fallback font.
        let font = text.new_font_by_name(font_name, font_size).build().unwrap();

        let inner = text
            .new_text_layout(&font, buffer.slice(..).as_ref(), width)
            .build()
            .ok();

        LayoutWrapper { inner }
    }

    fn update_width(&mut self, new_width: f64) {
        if let Some(inner) = &mut self.inner {
            inner.update_width(new_width).unwrap();
        }
    }

    /// The size of this layout, in display points.
    fn size(&self) -> Size {
        self.inner
            .as_ref()
            .map(|layout| {
                let height = layout
                    .line_metric(layout.line_count() - 1)
                    .map(|metric| metric.cumulative_height)
                    .unwrap_or(0.);
                Size::new(layout.width(), height)
            })
            .unwrap_or_default()
    }

    /// The position of the baseline relative to the origin of the typographic
    /// bounds.
    ///
    /// This is equal to the length of the ascender of the first line, + leading.
    fn first_baseline(&self) -> f64 {
        self.inner
            .as_ref()
            .and_then(|layout| layout.line_metric(0).map(|metric| metric.baseline))
            .unwrap_or_default()
    }

    /// Given a point on the screen, determine the corresponding position in the buffer.
    ///
    /// This position is guaranteed to always be a utf8 boundary.
    fn offset_for_point(&self, point: Point) -> usize {
        self.inner
            .as_ref()
            .map(|layout| layout.hit_test_point(point).metrics.text_position)
            .unwrap_or_default()
    }
}
