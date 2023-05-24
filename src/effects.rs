use bevy::prelude::*;
use bevy_sprite3d::*;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EffectTypeEvent>()
            .register_type::<Effect>()
            .insert_resource(ImageAssets::default())
            // initially load assets
            .add_startup_system(
                |asset_server: Res<AssetServer>,
                 mut assets: ResMut<ImageAssets>,
                 mut texture_atlases: ResMut<Assets<TextureAtlas>>| {
                    assets.image_bug_1 = asset_server.load("effects/hit.png");
                    assets.image_bug_2 = asset_server.load("effects/smoke.png");

                    assets.atlas_bug_1 = texture_atlases.add(TextureAtlas::from_grid(
                        assets.image_bug_1.clone(),
                        Vec2::new(32.0, 32.0),
                        3,
                        1,
                        None,
                        None,
                    ));
                    assets.atlas_bug_2 = texture_atlases.add(TextureAtlas::from_grid(
                        assets.image_bug_2.clone(),
                        Vec2::new(32.0, 32.0),
                        5,
                        1,
                        None,
                        None,
                    ));
                },
            )
            .add_system(spawn_particles)
            .add_system(animate_sprite);
    }
}

#[derive(Resource, Default)]
struct ImageAssets {
    image_bug_1: Handle<Image>,
    atlas_bug_1: Handle<TextureAtlas>,
    image_bug_2: Handle<Image>,
    atlas_bug_2: Handle<TextureAtlas>,
}

#[derive(Component, Reflect)]
#[reflect]
struct Effect {
    timer: Timer,
}

pub enum EffectTypeEvent {
    Click { pos: Vec3 },
    Dead { pos: Vec3 },
}

fn spawn_particles(
    mut cmd: Commands,
    assets: Res<ImageAssets>,
    mut sprite_params: Sprite3dParams,
    mut effects: EventReader<EffectTypeEvent>,
) {
    for effect in effects.iter() {
        let (pos, atlas, scale) = match effect {
            EffectTypeEvent::Click { pos } => (*pos, assets.atlas_bug_1.clone(), 2.),
            EffectTypeEvent::Dead { pos } => (*pos, assets.atlas_bug_2.clone(), 3.),
        };

        let pos = Vec3::new(pos.x, pos.y, pos.z + 1.1);

        cmd.spawn((
            AtlasSprite3d {
                atlas,
                pixels_per_metre: 32.,
                partial_alpha: true,
                unlit: true,
                index: 0,
                transform: Transform::from_translation(pos).with_scale(Vec3::splat(scale)),
                ..default()
            }
            .bundle(&mut sprite_params),
            Effect {
                timer: Timer::from_seconds(0.1, TimerMode::Once),
            },
        ));
    }
}

fn animate_sprite(
    mut cmd: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Effect, &mut AtlasSprite3dComponent)>,
) {
    for (entity, mut effect, mut sprite) in query.iter_mut() {
        if effect.timer.tick(time.delta()).just_finished() {
            if sprite.index == sprite.atlas.len() - 1 {
                cmd.entity(entity).despawn_recursive();
                continue;
            }
            sprite.index += 1;
            effect.timer.reset();
        }
    }
}
