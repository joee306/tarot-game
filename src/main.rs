use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::prelude::*;

mod controller;
mod player_animation;

use controller::*;
use player_animation::PlayerAnimationPlugin;
//use test_sprite::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(EditorPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        // Internal game plugins
        .add_plugins(ControllerPlugin)
        .add_plugins(PlayerAnimationPlugin)
        .add_systems(Startup, spawn_world)
        .run();
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Collider::cuboid(3.0, 0.1, 3.0),
            RigidBody::Fixed,
            TransformBundle::from(Transform {
                translation: Vec3::new(-5.0, 1.0, -5.0),
                rotation: Quat::from_rotation_z(-25.0_f32.to_radians()),
                ..default()
            }),
        ))
        .with_children(|child| {
            child.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(6.0, 6.0)))),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    rotation: Quat::from_rotation_x(-90.0_f32.to_radians()),
                    ..default()
                },
                material: materials.add(Color::rgb(0.4, 0.7, 0.5).into()),
                ..default()
            });
        });

    commands.spawn((World, TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))).with_children(|parent| {
        for x in 0..50 {
            for y in 0..50 {
                parent.spawn((
                    Collider::cuboid(0.5, 0.5, 0.5),
                    RigidBody::Fixed,
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                        material: materials.add(Color::rgb(0.3, 0.7, 0.4).into()),
                        transform: Transform::from_xyz(
                            1.0 * x as f32 - 25.0,
                            0.2 * rand::thread_rng().gen_range(0..4) as f32,
                            1.0 * y as f32 - 25.0,
                        ),
                        ..default()
                    },
                ));
            }
        }
    });
}

#[derive(Component)]
struct World;
