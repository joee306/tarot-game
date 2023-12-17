use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_sprite3d::*;

mod components;

use crate::controller::{self, Player};
pub use components::*;

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugins(Sprite3dPlugin)
            .add_loading_state(
                LoadingState::new(GameState::Loading).continue_to_state(GameState::Main),
            )
            .add_collection_to_loading_state::<_, PlayerSpites>(GameState::Loading)
            .add_systems(OnEnter(GameState::Main), spawn_player)
            .add_systems(
                Update,
                (
                    update_animation.run_if(in_state(GameState::Main)),
                    update_direction.run_if(in_state(GameState::Main)),
                ),
            );
    }
}

fn spawn_player(
    assets: Res<PlayerSpites>,
    mut commands: Commands,
    mut sprite_params: Sprite3dParams,
    query: Query<Entity, With<Player>>,
) {
    for entity in query.iter() {
        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                AtlasSprite3d {
                    atlas: assets.player_walking_side.clone(),
                    pixels_per_metre: 128.,
                    alpha_mode: AlphaMode::Blend,
                    unlit: true,
                    index: 0,
                    transform: Transform::from_xyz(0.0, 0.0, 0.0)
                        .with_rotation(Quat::from_rotation_y(45.0_f32.to_radians())),

                    // pivot: Some(Vec2::new(0.5, 0.5)),
                    ..default()
                }
                .bundle(&mut sprite_params),
                AnimationTimer {
                    timer: Timer::from_seconds(0.125, TimerMode::Repeating),
                    frame_count: 12,
                },
            ));
        });
    }
}

fn update_direction(
    assets: Res<PlayerSpites>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut player_query: Query<(&Children, &controller::Direction), With<Player>>,
    mut transform_query: Query<
        (&mut Transform, &Handle<StandardMaterial>),
        With<AtlasSprite3dComponent>,
    >,
) {
    use controller::Direction as Dir;
    for (children, direction) in player_query.iter_mut() {
        // get child's transform with atlas sprite
        for child in children {
            if let Ok((mut transform, material_handle)) = transform_query.get_mut(*child) {
                let mut change = |sprite| {
                    let material = materials.get_mut(material_handle).unwrap();
                    material.base_color_texture = Some(sprite);
                };
                match direction {
                    Dir::Left => {
                        transform.rotation = Quat::from_rotation_y((180.0_f32 + 45.).to_radians());
                        change(assets.player_walking_side_img.clone());
                    }
                    Dir::Right => {
                        transform.rotation = Quat::from_rotation_y(45.0_f32.to_radians());
                        change(assets.player_walking_side_img.clone());
                    }
                    Dir::Down => {
                        change(assets.player_walking_down.clone());
                    }
                    Dir::Up => {
                        change(assets.player_walking_up.clone());
                    }
                }
            }
        }
    }
}

fn update_animation(
    mut sprites: Query<(&mut AtlasSprite3dComponent, &mut AnimationTimer)>,
    time: Res<Time>,
) {
    for (mut texture, mut animation) in sprites.iter_mut() {
        animation.timer.tick(time.delta());
        if animation.timer.just_finished() {
            texture.index = (texture.index + 1) % animation.frame_count;
        }
    }
}
