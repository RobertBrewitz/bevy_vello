use std::borrow::Cow;

use crate::text::context::{get_global_font_context, LOCAL_LAYOUT_CONTEXT};

use super::{
    context::LOCAL_FONT_CONTEXT,
    vello_text::{VelloFontAxes, VelloTextSection},
    VelloTextAnchor,
};
use bevy::{prelude::*, reflect::TypePath, render::render_asset::RenderAsset};
use parley::{
    AlignmentOptions, FontSettings, FontStyle, FontWeight, FontWidth, InlineBox,
    PositionedLayoutItem, RangedBuilder, StyleProperty,
};
use vello::{
    kurbo::Affine,
    peniko::{Brush, Fill},
    Scene,
};

#[derive(Asset, TypePath, Debug, Clone)]
pub struct VelloFont {
    pub family_name: String,
    pub bytes: Vec<u8>,
}

impl RenderAsset for VelloFont {
    type SourceAsset = VelloFont;

    type Param = ();

    fn prepare_asset(
        source_asset: Self::SourceAsset,
        _param: &mut bevy::ecs::system::SystemParamItem<Self::Param>,
    ) -> Result<Self, bevy::render::render_asset::PrepareAssetError<Self::SourceAsset>> {
        Ok(source_asset)
    }
}

impl VelloFont {
    pub fn new(font_data: Vec<u8>) -> Self {
        Self {
            bytes: font_data,
            family_name: "".to_string(),
        }
    }

    pub fn sizeof(&self, text_section: &VelloTextSection) -> Vec2 {
        LOCAL_FONT_CONTEXT.with_borrow_mut(|font_context| {
            if font_context.is_none() {
                *font_context = Some(get_global_font_context().clone());
            }

            let font_context = font_context.as_mut().unwrap();

            LOCAL_LAYOUT_CONTEXT.with_borrow_mut(|layout_context| {
                // TODO: fix scale magic number
                // TODO: store and reuse the builder?
                let mut builder =
                    layout_context.ranged_builder(font_context, &text_section.value, 1.0);
                builder.push_default(StyleProperty::FontStack(parley::FontStack::Single(
                    parley::FontFamily::Named(Cow::Borrowed(&self.family_name)),
                )));
                // apply_font_styles(&mut builder, text_section);
                // apply_variable_axes(&mut builder, &text_section.style.font_axes);
                if let Some(weight) = text_section.style.font_axes.font_weight {
                    println!("apply weight: {}", weight);
                    builder.push_default(StyleProperty::FontWeight(FontWeight::new(weight)));
                }
                builder.push_inline_box(InlineBox {
                    id: 0,
                    index: 5,
                    width: 50.0,
                    height: 50.0,
                });
                let mut layout = builder.build(&text_section.value);
                layout.break_all_lines(None);
                Vec2::new(layout.width(), layout.height())
            })
        })
    }

    pub(crate) fn render(
        &self,
        scene: &mut Scene,
        mut transform: Affine,
        text_section: &VelloTextSection,
        text_anchor: VelloTextAnchor,
    ) {
        LOCAL_FONT_CONTEXT.with_borrow_mut(|font_context| {
            if font_context.is_none() {
                *font_context = Some(get_global_font_context().clone());
            }

            let font_context = font_context.as_mut().unwrap();

            LOCAL_LAYOUT_CONTEXT.with_borrow_mut(|layout_context| {
                // TODO: fix scale magic number
                // TODO: store and reuse the builder?
                let mut builder =
                    layout_context.ranged_builder(font_context, &text_section.value, 1.0);

                println!("family_name: {}", self.family_name);
                builder.push_default(StyleProperty::FontStack(parley::FontStack::Single(
                    parley::FontFamily::Named(Cow::Borrowed(&self.family_name)),
                )));

                // apply_font_styles(&mut builder, text_section);
                // apply_variable_axes(&mut builder, &text_section.style.font_axes);
                if let Some(weight) = text_section.style.font_axes.font_weight {
                    println!("apply weight: {}", weight);
                    builder.push_default(StyleProperty::FontWeight(FontWeight::new(weight)));
                }

                let mut layout = builder.build(&text_section.value);

                layout.break_all_lines(None);
                layout.align(None, parley::Alignment::Middle, AlignmentOptions::default());

                for line in layout.lines() {
                    for item in line.items() {
                        let PositionedLayoutItem::GlyphRun(glyph_run) = item else {
                            continue;
                        };

                        let mut x = glyph_run.offset();
                        let y = glyph_run.baseline();
                        let run = glyph_run.run();
                        let font = run.font();
                        let font_size = run.font_size();
                        let synthesis = run.synthesis();
                        let glyph_xform = synthesis
                            .skew()
                            .map(|angle| Affine::skew(angle.to_radians().tan() as f64, 0.0));

                        scene
                            .draw_glyphs(font)
                            .brush(&text_section.style.brush)
                            .hint(true)
                            .transform(transform)
                            .glyph_transform(glyph_xform)
                            .font_size(font_size)
                            .normalized_coords(run.normalized_coords())
                            .draw(
                                Fill::NonZero,
                                glyph_run.glyphs().map(|glyph| {
                                    let gx = x + glyph.x;
                                    let gy = y - glyph.y;
                                    x += glyph.advance;
                                    vello::Glyph {
                                        id: glyph.id as _,
                                        x: gx,
                                        y: gy,
                                    }
                                }),
                            );
                    }
                }

                let width = layout.width() as f64;
                let height = layout.height() as f64;

                match text_anchor {
                    VelloTextAnchor::TopLeft => {
                        transform *= vello::kurbo::Affine::translate((0.0, height));
                    }
                    VelloTextAnchor::Left => {
                        transform *= vello::kurbo::Affine::translate((0.0, height / 2.0));
                    }
                    VelloTextAnchor::BottomLeft => {
                        transform *= vello::kurbo::Affine::translate((0.0, 0.0));
                    }
                    VelloTextAnchor::Top => {
                        transform *= vello::kurbo::Affine::translate((-width / 2.0, height));
                    }
                    VelloTextAnchor::Center => {
                        transform *= vello::kurbo::Affine::translate((-width / 2.0, height / 2.0));
                    }
                    VelloTextAnchor::Bottom => {
                        transform *= vello::kurbo::Affine::translate((-width / 2.0, 0.0));
                    }
                    VelloTextAnchor::TopRight => {
                        transform *= vello::kurbo::Affine::translate((-width, height));
                    }
                    VelloTextAnchor::Right => {
                        transform *= vello::kurbo::Affine::translate((-width, height / 2.0));
                    }
                    VelloTextAnchor::BottomRight => {
                        transform *= vello::kurbo::Affine::translate((-width, 0.0));
                    }
                }
            });
        })
    }
}

// Applies the font styles to the text
//
// font - font asset
// line_height - line height
fn apply_font_styles(builder: &mut RangedBuilder<'_, Brush>, text: &VelloTextSection) {
    builder.push_default(StyleProperty::FontSize(text.style.font_size));

    if let Some(line_height) = text.style.line_height {
        builder.push_default(StyleProperty::LineHeight(line_height));
    }
}

// Applies the variable axes to the text
//
// wght - font weight
// wdth - font width
// opsz - optical size
// ital - italic
// slnt - slant
// grad - grade
// xopq - thick stroke
// yopq - thin stroke
// ytuc - uppercase height
// ytlc - lowercase height
// ytas - ascender height
// ytde - descender depth
// ytfi - figure height
fn apply_variable_axes(builder: &mut RangedBuilder<'_, Brush>, text: &VelloFontAxes) {
    if let Some(weight) = text.font_weight {
        println!("apply weight: {}", weight);
        builder.push_default(StyleProperty::FontWeight(FontWeight::new(weight)));
    }

    if let Some(width) = text.font_width {
        // sets the width for the entire text
        builder.push_default(StyleProperty::FontWidth(FontWidth::from_ratio(width)));
    }

    if let Some(optical_size) = text.optical_size {
        // sets the optical size for the entire text
        builder.push_default(StyleProperty::FontFeatures(FontSettings::Source(
            Cow::Borrowed(&format!("'opsz' {}", optical_size).to_string()),
        )));
    }

    // FontVariations(FontSettings<'a, FontVariation>),
    // FontFeatures(FontSettings<'a, FontFeature>),

    if text.italic {
        builder.push_default(StyleProperty::FontStyle(FontStyle::Italic));
    } else if text.slant.is_some() {
        // sets the slant for the entire text
        builder.push_default(StyleProperty::FontStyle(FontStyle::Oblique(text.slant)));
    }
}
