mod components;
mod setup;

use components::{Direction, *};
use setup::*;

use bevy::{pbr::DirectionalLightShadowMap, prelude::*};
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(DirectionalLightShadowMap { size: 4096 })
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup_scene, setup_player))
        .add_systems(
            Update,
            (
                movement,
                bind_system,
                is_falling_system,
                animate,
                display_events,
                attack,
                remove_in_animation,
            ),
        )
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                intensity: 15000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        },
        BinderFollwer(Transform::from_xyz(0.0, 15.0, 0.0)),
    ));

    commands.spawn((
        Collider::cuboid(10000.0, 0.0001, 10000.0),
        PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(200.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        Friction {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        },
    ));
    commands.spawn((
        Collider::cylinder(0.1, 2.5),
        Sensor,
        //Dialog::new("../story/test.json".into()),
        ActiveEvents::COLLISION_EVENTS,
        TransformBundle::from(Transform::from_xyz(10.0, 0.0, 0.0)),
    ));
}

fn bind_system(
    query: Query<(&Transform, &Binder, Without<BinderFollwer>)>,
    mut follwer_query: Query<(&mut Transform, &BinderFollwer)>,
) {
    let (binder_transform, _, _) = query.single();
    for (mut follwer_transform, follwer) in &mut follwer_query {
        let mut new_transform = *binder_transform;
        new_transform.translation += follwer.0.translation;
        new_transform.scale += follwer.0.scale;
        new_transform.rotation = follwer_transform.rotation;
        *follwer_transform = new_transform;
    }
}

fn movement(
    mut query: Query<
        (&mut Velocity, &IsFalling, &mut Controller, &mut Direction),
        Without<InAnimation>,
    >,
    keyboard: Res<Input<KeyCode>>,
) {
    for (mut velocity, is_falling, mut controller, mut dir) in &mut query {
        let mut speed = 10.0;
        let mut change = false;
        let mut direction = Vec3::ZERO;
        if keyboard.pressed(KeyCode::Space) && !is_falling.falling {
            velocity.linvel.y = 5.0;
        }
        if keyboard.pressed(KeyCode::ControlLeft) {
            speed *= 1.5;
        }
        if keyboard.pressed(KeyCode::W) && !is_falling.falling {
            direction.z -= 1.0;
            change = true;
            *dir = Direction::Up;
        }
        if keyboard.pressed(KeyCode::S) && !is_falling.falling {
            direction.z += 1.0;
            change = true;
            *dir = Direction::Down;
        }
        if keyboard.pressed(KeyCode::A) && !is_falling.falling {
            direction.x -= 1.0;
            change = true;
            *dir = Direction::Left;
        }
        if keyboard.pressed(KeyCode::D) && !is_falling.falling {
            direction.x += 1.0;
            change = true;
            *dir = Direction::Right;
        }
        if change {
            velocity.linvel.z = direction.z * speed;
            velocity.linvel.x = direction.x * speed;
            controller.direction = direction;
        }
    }
}

fn attack(
    mut commands: Commands,
    query: Query<(Entity, &Transform, &Direction), (With<Controller>, Without<InAnimation>)>,
    mouse_buttons: Res<Input<MouseButton>>,
) {
    for (player, player_transfom, dir) in &query {
        let mut tf = player_transfom.clone(); // transform from the player
        match *dir {
            Direction::Up => tf.translation.z -= 1.2,
            Direction::Left => tf.translation.x -= 1.2,
            Direction::Down => tf.translation.z += 1.2,
            Direction::Right => tf.translation.x += 1.2,
        };
        if mouse_buttons.just_released(MouseButton::Left) {
            commands.spawn((
                Collider::cuboid(0.5, 1.0, 1.0),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                TransformBundle::from(tf),
            ));
            commands.entity(player).insert(InAnimation(0.3));
        }
    }
}

fn remove_in_animation(
    mut commands: Commands,
    mut query: Query<(Entity, &mut InAnimation)>,
    time: Res<Time>,
) {
    let seconds = time.delta_seconds();
    for (entity, mut in_animation) in &mut query {
        in_animation.0 -= seconds;
        if in_animation.0 <= 0.0 {
            commands.entity(entity).remove::<InAnimation>();
        }
    }
}

fn is_falling_system(mut query: Query<(&mut IsFalling, &Transform)>) {
    for (mut i_f, tf) in &mut query {
        i_f.falling = (tf.translation.y * 10000.0).round() != (i_f.last_y * 10000.0).round();
        i_f.last_y = tf.translation.y
    }
}

fn animate(
    mut query: Query<
        (&mut Handle<StandardMaterial>, &mut Animator, &mut Transform),
        Without<InAnimation>,
    >,
    controllers: Query<&Controller, Without<Camera>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let controller = controllers.single();
    for (material, mut ani, mut transform) in query.iter_mut() {
        if controller.direction.x == -1.0 {
            transform.rotation = Quat::from_rotation_y(std::f32::consts::PI);
        }
        if controller.direction.x == 1.0 {
            transform.rotation.y = 0.0;
        }
        let color_mat = materials.get_mut(&material).unwrap();
        // poorly programmed please rework
        color_mat.base_color_texture = Some(asset_server.load(format!("{}0.png", ani.0)));
        if ani.0 < ani.1 {
            ani.0 += 1;
        } else {
            ani.0 = 0;
        }
    }
}

fn display_events(mut collision_events: EventReader<CollisionEvent>, query: Query<&Dialog>) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
        for dialog in &query {
            println!("{:?}", dialog);
        }
    }
}
