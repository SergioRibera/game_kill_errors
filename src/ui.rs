use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_ui);
    }
}

fn setup_ui(mut cmd: Commands, asset_serve: Res<AssetServer>) {
    let font_regular = asset_serve.load("fonts/Lato-Regular.ttf");
    let font_light = asset_serve.load("fonts/Lato-Light.ttf");

    cmd.spawn(NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            position: UiRect::all(Val::Percent(50.)),
            gap: Size::height(Val::Px(10.)),
            ..default()
        },
        ..default()
    })
    .with_children(|cmd| {
        cmd.spawn(TextBundle {
            text: Text::from_section(
                "404",
                TextStyle {
                    font: font_regular.clone(),
                    font_size: 128.,
                    color: Color::rgba_u8(52, 52, 52, 255),
                },
            )
            .with_alignment(TextAlignment::Center),
            ..default()
        });
        cmd.spawn(TextBundle {
            text: Text::from_section(
                "Al parecer no encontramos lo que buscas",
                TextStyle {
                    font: font_light.clone(),
                    font_size: 32.,
                    color: Color::rgba_u8(52, 52, 52, 255),
                },
            )
            .with_alignment(TextAlignment::Center),
            ..default()
        });
        cmd.spawn(ButtonBundle {
            interaction: Interaction::Clicked,
            background_color: BackgroundColor(Color::WHITE.with_a(0.)),
            ..default()
        })
        .with_children(|cmd| {
            cmd.spawn(TextBundle {
                text: Text::from_section(
                    "Volver al Inicio",
                    TextStyle {
                        font: font_light.clone(),
                        font_size: 32.,
                        color: Color::rgba_u8(0, 133, 255, 255),
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..default()
            });
        });
    });

    cmd.spawn(TextBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            position: UiRect {
                bottom: Val::Px(50.),
                left: Val::Percent(50.),
                right: Val::Percent(50.),
                ..default()
            },
            ..default()
        },
        text: Text::from_section(
            "La paciencia es una gran virtud",
            TextStyle {
                font: font_regular.clone(),
                font_size: 32.,
                color: Color::rgba_u8(52, 52, 52, 45),
            },
        )
        .with_alignment(TextAlignment::Center),
        ..default()
    });
}
