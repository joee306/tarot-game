use crate::components::{Direction, *};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 3.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BinderFollwer(Transform::from_xyz(0., 3.0, 14.0)),
    ));

    //spawn a body that is the player
    commands.spawn((
        RigidBody::Dynamic,
        Collider::cuboid(0.5, 1.0, 0.5),
        Controller::default(),
        Binder,
        IsFalling::default(),
        LockedAxes::ROTATION_LOCKED,
        Velocity::zero(),
        Direction::Left,
        Friction {
            coefficient: 20.0,
            combine_rule: CoefficientCombineRule::Average,
        },
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(2.3, 2.3)))),
            transform: Transform::from_xyz(1.0, 3.0, 1.0),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("00.png")),
                perceptual_roughness: 1.0,
                alpha_mode: AlphaMode::Mask(1.0),
                cull_mode: None,
                ..default()
            }),
            ..default()
        },
        Animator(0, 9),
    ));
}
