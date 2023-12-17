use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_rapier3d::prelude::*;

mod components;

pub use components::{Direction, *};

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (player_movement, zoom))
            .add_systems(Startup, init_system);
    }
}

fn player_movement(
    mut query: Query<(&mut Velocity, &Collider, &Transform, &mut Direction), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    rapier_context: Res<RapierContext>,
) {
    let (mut player, collider, transform, mut direction) = query.single_mut();
    if keyboard_input.pressed(KeyCode::W) {
        *direction = Direction::Up;
        player.linvel.x = -3.0;
        player.linvel.z = -3.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        *direction = Direction::Down;
        player.linvel.x = 3.0;
        player.linvel.z = 3.0;
    }
    if keyboard_input.pressed(KeyCode::A) {
        *direction = Direction::Left;
        player.linvel.x = -3.0;
        player.linvel.z = 3.0;
    }
    if keyboard_input.pressed(KeyCode::D) {
        *direction = Direction::Right;
        player.linvel.x = 3.0;
        player.linvel.z = -3.0;
    }
    let filter = QueryFilter::exclude_dynamic();
    let mut on_ground = false;
    rapier_context.intersections_with_shape(
        transform.translation,
        transform.rotation,
        &collider,
        filter,
        |_entity| {
            //println!("The entity {:?} intersects our shape.", entity);
            on_ground = true;
            true
        },
    );
    if keyboard_input.pressed(KeyCode::Space) && on_ground {
        player.linvel.y = 6.0;
    }
}

fn zoom(
    mut mouse_wheel_reader: EventReader<bevy::input::mouse::MouseWheel>,
    mut query: Query<&mut Projection, With<Camera>>,
) {
    const ZOOM_SENSITIVITY: f32 = 0.3;
    const ZOOM_MIN: f32 = 2.5;
    const ZOOM_MAX: f32 = 16.5;
    for mouse_wheel in mouse_wheel_reader.read() {
        if let Ok(mut projection) = query.get_single_mut() {
            if let Projection::Orthographic(orthographic) = &*projection {
                let zoom_scalar = 1.0 - ZOOM_SENSITIVITY * mouse_wheel.y;
                let zoomed = orthographic.scale * zoom_scalar;
                let scale = zoomed.max(ZOOM_MIN).min(ZOOM_MAX);
                *projection = Projection::Orthographic(OrthographicProjection {
                    scale,
                    scaling_mode: ScalingMode::FixedVertical(2.0),
                    ..default()
                });
            }
        }
    }
}

#[derive(Component)]
struct Camera;

fn init_system(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands
        .spawn((
            RigidBody::Dynamic,
            LockedAxes::ROTATION_LOCKED,
            Collider::cylinder(0.5, 0.3),
            //Collider::ball(0.4),
            Velocity::default(),
            KinematicCharacterController {
                max_slope_climb_angle: 45.0_f32.to_radians(),
                min_slope_slide_angle: 30.0_f32.to_radians(),
                slide: false,
                /*autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Relative(0.3),
                    min_width: CharacterLength::Relative(0.5),
                    include_dynamic_bodies: true,
                }),*/
                ..default()
            },
            Player,
            TransformBundle::from(Transform::from_xyz(0.0, 3.0, 0.0)),
            GravityScale(1.0),
            Ccd::enabled(),
            Sleeping::disabled(),
            Direction::Down,
        ))
        .with_children(|child| {
            child.spawn((
                Sensor,
                TransformBundle::from(Transform::from_xyz(0.0, -0.52, 0.0)),
                Collider::cylinder(0.08, 0.28),
            ));
            child.spawn((
                Camera3dBundle {
                    projection: OrthographicProjection {
                        scale: 3.0,
                        scaling_mode: ScalingMode::FixedVertical(2.0),
                        ..default()
                    }
                    .into(),
                    transform: Transform::from_xyz(10.0, 10.0, 10.0)
                        .looking_at(Vec3::ZERO, Vec3::Y),
                    ..default()
                },
                Camera,
            ));
        });
}
