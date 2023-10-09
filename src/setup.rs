use crate::components::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 3.0, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BinderFollwer(Transform::from_xyz(0., 3.0, 19.0)),
    ));

    //spawn a body that is the player
    commands.spawn((
        RigidBody::Dynamic,
        TransformBundle::from(Transform::from_xyz(4.0, 1.0, 4.0)),
        Collider::cuboid(0.5, 1.0, 0.5),
        Controller::default(),
        Binder,
        IsFalling::default(),
        LockedAxes::ROTATION_LOCKED,
        Velocity::zero(),
        Friction {
            coefficient: 20.0,
            combine_rule: CoefficientCombineRule::Average,
        },
    ));

    // for the player texture

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(1.0, 1.0)))),
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
        BinderFollwer(Transform::from_xyz(0.0, 0.0, 0.0)),
    ));
}
