use std::f32::consts::PI;

use bevy::{
    asset::{embedded_asset, AssetMetaCheck},
    prelude::*,
};
use bevy_vello::{prelude::*, text::VelloTextAnchor, VelloPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(AssetPlugin {
        meta_check: AssetMetaCheck::Never,
        ..default()
    }))
    .add_plugins(VelloPlugin::default())
    .add_systems(
        Startup,
        (setup_camera, setup_screenspace_text, setup_worldspace_text),
    )
    .add_systems(Update, (toggle_animations, animate_axes, gizmos).chain());
    embedded_asset!(app, "assets/Rubik-VariableFont_wght.ttf");
    app.run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, VelloView));
}

fn setup_worldspace_text(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands
        .spawn(VelloTextSection {
            value: "Default font\nand multi-line support.".to_string(),
            ..default()
        })
        .insert(VelloTextAnchor::Center)
        .insert(AnimationToggles::default());

    commands.spawn(VelloTextBundle {
        text: VelloTextSection {
            value: "RobotoFlex-VariableFont".to_string(),
            style: VelloTextStyle {
                font: asset_server.load("embedded://text/assets/Rubik-VariableFont_wght.ttf"),
                font_size: 48.0,
                ..default()
            },
        },
        text_anchor: VelloTextAnchor::Center,
        transform: Transform::from_xyz(0.0, 100.0, 0.0)
            .with_rotation(Quat::from_rotation_z(PI / 12.0)),
        ..default()
    });
}

#[derive(Debug, Default, Component)]
struct AnimationToggles {
    pub weight: bool,
    pub width: bool,
    pub slant: bool,
    pub grade: bool,
    pub thick_stroke: bool,
    pub thin_stroke: bool,
    pub counter_width: bool,
    pub uppercase_height: bool,
    pub lowercase_height: bool,
    pub ascender_height: bool,
    pub descender_depth: bool,
    pub figure_height: bool,
}

fn toggle_animations(
    mut query: Query<&mut AnimationToggles>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyQ) {
        for mut toggles in query.iter_mut() {
            toggles.weight = !toggles.weight;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyW) {
        for mut toggles in query.iter_mut() {
            toggles.width = !toggles.width;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyE) {
        for mut toggles in query.iter_mut() {
            toggles.slant = !toggles.slant;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyR) {
        for mut toggles in query.iter_mut() {
            toggles.grade = !toggles.grade;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        for mut toggles in query.iter_mut() {
            toggles.thick_stroke = !toggles.thick_stroke;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) {
        for mut toggles in query.iter_mut() {
            toggles.thin_stroke = !toggles.thin_stroke;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyD) {
        for mut toggles in query.iter_mut() {
            toggles.counter_width = !toggles.counter_width;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyF) {
        for mut toggles in query.iter_mut() {
            toggles.uppercase_height = !toggles.uppercase_height;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyZ) {
        for mut toggles in query.iter_mut() {
            toggles.lowercase_height = !toggles.lowercase_height;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyX) {
        for mut toggles in query.iter_mut() {
            toggles.ascender_height = !toggles.ascender_height;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyC) {
        for mut toggles in query.iter_mut() {
            toggles.descender_depth = !toggles.descender_depth;
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyV) {
        for mut toggles in query.iter_mut() {
            toggles.figure_height = !toggles.figure_height;
        }
    }
}

fn animate_axes(time: Res<Time>, mut query: Query<(&mut VelloTextSection, &AnimationToggles)>) {
    let sin_time = time.elapsed_secs().sin().mul_add(0.5, 0.5);

    // https://fonts.google.com/specimen/Roboto+Flex/tester?query=variable
    let font_weight = sin_time.remap(0., 1., 100., 1000.);
    let font_width = sin_time.remap(0., 1., 25., 151.);
    let slant = sin_time.remap(0., 1., -10., 0.);
    let grade = sin_time.remap(0., 1., -200., 150.);
    let thick_stroke = sin_time.remap(0., 1., 27., 175.);
    let thin_stroke = sin_time.remap(0., 1., 25., 135.);
    let counter_width = sin_time.remap(0., 1., 323., 603.);
    let uppercase_height = sin_time.remap(0., 1., 528., 760.);
    let lowercase_height = sin_time.remap(0., 1., 416., 570.);
    let ascender_height = sin_time.remap(0., 1., 649., 854.);
    let descender_depth = sin_time.remap(0., 1., -98., -305.);
    let figure_height = sin_time.remap(0., 1., 560., 788.);

    for (mut text_section, animation_toggles) in query.iter_mut() {
        if animation_toggles.weight {
            println!("weight: {}", font_weight);
            text_section.style.font_axes.font_weight = Some(font_weight);
        }

        if animation_toggles.width {
            text_section.style.font_axes.font_width = Some(font_width);
        }

        if animation_toggles.slant {
            text_section.style.font_axes.slant = Some(slant);
        }

        if animation_toggles.grade {
            text_section.style.font_axes.grade = Some(grade as i32);
        }

        if animation_toggles.thick_stroke {
            text_section.style.font_axes.thick_stroke = Some(thick_stroke as i32);
        }

        if animation_toggles.thin_stroke {
            text_section.style.font_axes.thin_stroke = Some(thin_stroke as i32);
        }

        if animation_toggles.counter_width {
            text_section.style.font_axes.counter_width = Some(counter_width as i32);
        }

        if animation_toggles.uppercase_height {
            text_section.style.font_axes.uppercase_height = Some(uppercase_height as u32);
        }

        if animation_toggles.lowercase_height {
            text_section.style.font_axes.lowercase_height = Some(lowercase_height as u32);
        }

        if animation_toggles.ascender_height {
            text_section.style.font_axes.ascender_height = Some(ascender_height as u32);
        }

        if animation_toggles.descender_depth {
            text_section.style.font_axes.descender_depth = Some(descender_depth as i32);
        }

        if animation_toggles.figure_height {
            text_section.style.font_axes.figure_height = Some(figure_height as u32);
        }
    }
}

fn setup_screenspace_text(mut commands: Commands) {
    // Bevy text
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(100.0),
                left: Val::Px(100.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
        ))
        .insert(Text::new("Use bevy's Text for UI text!"))
        .insert(TextFont {
            font_size: 24.,
            ..default()
        })
        .insert(TextLayout::new_with_justify(JustifyText::Left));
}

fn gizmos(
    texts: Query<(&VelloTextSection, &GlobalTransform)>,
    assets: Res<Assets<VelloFont>>,
    mut gizmos: Gizmos,
) {
    for (text, gtransform) in texts.iter() {
        let Some(font) = assets.get(text.style.font.id()) else {
            continue;
        };

        let bb_size = font.sizeof(text);

        gizmos.rect_2d(
            Isometry2d::new(
                gtransform.translation().xy(),
                Rot2::radians(gtransform.rotation().to_scaled_axis().z),
            ),
            bb_size * gtransform.scale().xy(),
            Color::WHITE,
        );
    }
}
