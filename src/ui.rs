use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_tweening::{Animator, Delay, EaseFunction, Tween, TweenCompleted};

use crate::{
    game::ScoreText,
    lens::{GameTextColorLens, InstanceLens},
    locale::LocaleAsset,
    LocaleLangs, OpenLinkResource, HOME_URL, TIME_WAIT_TO_START,
};

#[derive(Component)]
struct RemovableUI;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            |mut cmd: Commands, asset_server: Res<AssetServer>| {
                cmd.insert_resource(UiResources {
                    languages: vec![
                        asset_server.load("locale/en-EN.locale"), // english dictionary
                        asset_server.load("locale/es-ES.locale"), //spanish dictionary
                    ],
                });
            },
        )
        .add_systems(
            Update,
            (setup_ui, button_system, remove_screen.run_if(run_if_anim)),
        );
    }
}

#[derive(Resource)]
struct UiResources {
    languages: Vec<Handle<LocaleAsset>>,
}

fn setup_ui(
    mut cmd: Commands,
    ui_res: Res<UiResources>,
    asset_serve: Res<AssetServer>,
    lang: Res<LocaleLangs>,
    languages: Res<Assets<LocaleAsset>>,
    mut runned: Local<bool>,
) {
    if *runned {
        return;
    }
    let font_regular = asset_serve.load("fonts/Lato-Regular.ttf");
    let font_light = asset_serve.load("fonts/Lato-Light.ttf");

    let Some(lang) = languages.get(&ui_res.languages[*lang as usize]) else { return; };
    *runned = true;

    // 404 text
    cmd.spawn(NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            column_gap: Val::Px(10.),
            ..default()
        },
        ..default()
    })
    .insert(RemovableUI)
    .with_children(|cmd| {
        cmd.spawn((
            TextBundle {
                text: Text::from_sections([
                    TextSection::new(
                        "404\n",
                        TextStyle {
                            font: font_regular.clone(),
                            font_size: 128.,
                            color: Color::rgba_u8(52, 52, 52, 255),
                        },
                    ),
                    TextSection::new(
                        lang.get_default("message1", "Al parecer no encontramos lo que buscas"),
                        TextStyle {
                            font: font_light.clone(),
                            font_size: 32.,
                            color: Color::rgba_u8(52, 52, 52, 255),
                        },
                    ),
                ])
                .with_alignment(TextAlignment::Center),
                ..default()
            },
            Animator::new(
                Delay::new(Duration::from_secs(*TIME_WAIT_TO_START)).then(
                    Tween::new(
                        EaseFunction::QuadraticInOut,
                        Duration::from_secs(5),
                        GameTextColorLens::create(
                            Color::rgba_u8(52, 52, 52, 255),
                            Color::rgba_u8(52, 52, 52, 0),
                        ),
                    )
                    .with_completed_event(1),
                ),
            ),
        ));

        // Come back to Home button
        cmd.spawn(ButtonBundle {
            background_color: BackgroundColor(Color::WHITE.with_a(0.)),
            ..default()
        })
        .with_children(|cmd| {
            cmd.spawn((
                TextBundle {
                    text: Text::from_section(
                        lang.get_default("button", "Volver al Inicio"),
                        TextStyle {
                            font: font_light.clone(),
                            font_size: 32.,
                            color: Color::rgba_u8(0, 133, 255, 255),
                        },
                    )
                    .with_alignment(TextAlignment::Center),
                    ..default()
                },
                Animator::new(
                    Delay::new(Duration::from_secs(*TIME_WAIT_TO_START)).then(
                        Tween::new(
                            EaseFunction::QuadraticInOut,
                            Duration::from_secs(5),
                            GameTextColorLens::create(
                                Color::rgba_u8(0, 133, 255, 255),
                                Color::rgba_u8(0, 133, 255, 0),
                            ),
                        )
                        .with_completed_event(1),
                    ),
                ),
            ));
        });
    });

    // Transparent warning message
    cmd.spawn(NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexEnd,
            bottom: Val::Px(50.),
            width: Val::Vw(100.),
            // height: Val::Vw(100.),
            // size: Size::all(Val::Percent(100.)),
            ..default()
        },
        ..default()
    })
    .insert(RemovableUI)
    .with_children(|cmd| {
        cmd.spawn((
            TextBundle {
                text: Text::from_section(
                    lang.get_default("message2", "La paciencia es una gran virtud"),
                    TextStyle {
                        font: font_regular.clone(),
                        font_size: 32.,
                        color: Color::rgba_u8(52, 52, 52, 45),
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..default()
            },
            Animator::new(
                Delay::new(Duration::from_secs(*TIME_WAIT_TO_START)).then(
                    Tween::new(
                        EaseFunction::QuadraticInOut,
                        Duration::from_secs(5),
                        GameTextColorLens::create(
                            Color::rgba_u8(52, 52, 52, 45),
                            Color::rgba_u8(52, 52, 52, 0),
                        ),
                    )
                    .with_completed_event(1),
                ),
            ),
        ));
    });

    // Score Text
    cmd.spawn(NodeBundle {
        style: Style {
            display: Display::Flex,
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            left: Val::Vw(50.),
            top: Val::Vh(50.),
            right: Val::Vw(50.),
            bottom: Val::Vh(50.),
            ..default()
        },
        ..default()
    })
    .with_children(|cmd| {
        cmd.spawn((
            TextBundle {
                text: Text::from_section(
                    "0",
                    TextStyle {
                        font: font_light.clone(),
                        font_size: 128.,
                        color: Color::rgba_u8(52, 52, 52, 0),
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..default()
            },
            ScoreText,
            Animator::new(
                Delay::new(Duration::from_secs(*TIME_WAIT_TO_START + 3)).then(
                    Tween::new(
                        EaseFunction::QuadraticInOut,
                        Duration::from_secs(5),
                        GameTextColorLens::create(
                            Color::rgba_u8(52, 52, 52, 0),
                            Color::rgba_u8(52, 52, 52, 255),
                        ),
                    )
                    .with_completed_event(2),
                ),
            ),
        ));
    });
}

fn button_system(
    mut window: Query<&mut Window, With<PrimaryWindow>>,
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    url_callback: Res<OpenLinkResource>,
) {
    let mut window = window.single_mut();
    for (interaction, _children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                window.cursor.icon = CursorIcon::Hand;
                url_callback.0(HOME_URL);
            }
            Interaction::Hovered => window.cursor.icon = CursorIcon::Hand,
            Interaction::None => window.cursor.icon = CursorIcon::Default,
        }
    }
}

fn run_if_anim(
    anim_reader: EventReader<TweenCompleted>,
    texts: Query<Entity, (With<Node>, With<RemovableUI>)>,
) -> bool {
    !anim_reader.is_empty() && !texts.is_empty()
}

fn remove_screen(mut cmd: Commands, texts: Query<Entity, (With<Node>, With<RemovableUI>)>) {
    for text in texts.iter() {
        cmd.entity(text).despawn_recursive();
    }
}
