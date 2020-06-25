// Copyright 2019 The druid Authors.
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

//! An example of a custom drawing widget.

//use druid::kurbo::BezPath;
//use druid::piet::{FontBuilder, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::text::{TextBuffer, TextLayout};
use druid::widget::{prelude::*, Button, Flex, MainAxisAlignment, Scroll};
use druid::{AppLauncher, Data, WidgetExt, WindowDesc};

#[derive(Default)]
struct TextWidget {
    inner: Option<TextLayout>,
}

impl Widget<TextBuffer> for TextWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut TextBuffer, _env: &Env) {}

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &TextBuffer,
        _env: &Env,
    ) {
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &TextBuffer, data: &TextBuffer, env: &Env) {
        if !old_data.same(data) {
            if let Some(inner) = self.inner.as_mut() {
                eprintln!("updating buffer");
                inner.update_buffer(data.clone(), env);
            }
            ctx.request_layout();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &TextBuffer,
        env: &Env,
    ) -> Size {
        if let Some(inner) = self.inner.as_mut() {
            inner.update_width(bc.max().width);
        } else {
            self.inner = Some(TextLayout::new(
                data.clone(),
                ctx.text(),
                env,
                bc.max().width,
            ));
        }
        let size = self
            .inner
            .as_ref()
            .map(|layout| layout.size())
            .unwrap_or_default();
        dbg!(size);
        dbg!(bc.constrain(size))
    }

    //// The paint method gets called last, after an event flow.
    //// It goes event -> update -> layout -> paint, and each method can influence the next.
    //// Basically, anything that changes the appearance of a widget causes a paint.
    fn paint(&mut self, ctx: &mut PaintCtx, _data: &TextBuffer, env: &Env) {
        if let Some(inner) = self.inner.as_ref() {
            inner.draw(ctx, (0., 0.), env);
        }
    }
}

static ONE_TEXT: &str = "This is just a single little line, and that's just fine.";
static TWO_TEXT: &str = r#"Life in this society being, at best, an utter bore and no aspect of society being at all relevant to women, there remains to civic-minded, responsible, thrill-seeking females only to overthrow the government, eliminate the money system, institute complete automation and destroy the male sex.

It is now technically feasible to reproduce without the aid of males (or, for that matter, females) and to produce only females. We must begin immediately to do so. Retaining the male has not even the dubious purpose of reproduction. The male is a biological accident: the Y (male) gene is an incomplete X (female) gene, that is, it has an incomplete set of chromosomes. In other words, the male is an incomplete female, a walking abortion, aborted at the gene stage. To be male is to be deficient, emotionally limited; maleness is a deficiency disease and males are emotional cripples.

"#;
static THREE_TEXT: &str = r#"Thou still unravish'd bride of quietness,
       Thou foster-child of silence and slow time,
Sylvan historian, who canst thus express
       A flowery tale more sweetly than our rhyme:
What leaf-fring'd legend haunts about thy shape
       Of deities or mortals, or of both,
               In Tempe or the dales of Arcady?
       What men or gods are these? What maidens loth?
What mad pursuit? What struggle to escape?
               What pipes and timbrels? What wild ecstasy?

Heard melodies are sweet, but those unheard
       Are sweeter; therefore, ye soft pipes, play on;
Not to the sensual ear, but, more endear'd,
       Pipe to the spirit ditties of no tone:
Fair youth, beneath the trees, thou canst not leave
       Thy song, nor ever can those trees be bare;
               Bold Lover, never, never canst thou kiss,
Though winning near the goal yet, do not grieve;
       She cannot fade, though thou hast not thy bliss,
               For ever wilt thou love, and she be fair!

Ah, happy, happy boughs! that cannot shed
         Your leaves, nor ever bid the Spring adieu;
And, happy melodist, unwearied,
         For ever piping songs for ever new;
More happy love! more happy, happy love!
         For ever warm and still to be enjoy'd,
                For ever panting, and for ever young;
All breathing human passion far above,
         That leaves a heart high-sorrowful and cloy'd,
                A burning forehead, and a parching tongue.

Who are these coming to the sacrifice?
         To what green altar, O mysterious priest,
Lead'st thou that heifer lowing at the skies,
         And all her silken flanks with garlands drest?
What little town by river or sea shore,
         Or mountain-built with peaceful citadel,
                Is emptied of this folk, this pious morn?
And, little town, thy streets for evermore
         Will silent be; and not a soul to tell
                Why thou art desolate, can e'er return.

O Attic shape! Fair attitude! with brede
         Of marble men and maidens overwrought,
With forest branches and the trodden weed;
         Thou, silent form, dost tease us out of thought
As doth eternity: Cold Pastoral!
         When old age shall this generation waste,
                Thou shalt remain, in midst of other woe
Than ours, a friend to man, to whom thou say'st,
         "Beauty is truth, truth beauty,—that is all
                Ye know on earth, and all ye need to know."
"#;

pub fn main() {
    let window = WindowDesc::new(make_ui).title("Text test");
    AppLauncher::with_window(window)
        .use_simple_logger()
        .launch("Druid + Piet".into())
        .expect("launch failed");
}

fn make_ui() -> impl Widget<TextBuffer> {
    Flex::column()
        .with_child(
            Flex::row()
                .main_axis_alignment(MainAxisAlignment::SpaceEvenly)
                .with_child(Button::new("One").on_click(|_, data, _| *data = ONE_TEXT.into()))
                .with_child(Button::new("Two").on_click(|_, data, _| *data = TWO_TEXT.into()))
                .with_child(Button::new("Three").on_click(|_, data, _| *data = THREE_TEXT.into())),
        )
        .with_spacer(8.0)
        .with_flex_child(
            Scroll::new(TextWidget::default().padding(8.0))
                .vertical()
                .expand_width(),
            1.0,
        )
        .debug_paint_layout()
}
