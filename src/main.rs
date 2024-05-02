use std::{thread::sleep, time::Duration};

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::prelude::*;

#[derive(Component, Copy, Clone)]
struct HasColor(Color);

#[derive(Component, Debug, Copy, Clone)]
struct Following;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(WorldInspectorPlugin::new());

    app.world.resource_mut::<Gravity>().0 = Vec2::new(0.0, 0.0);
    app.world.spawn(Camera2dBundle::default());
    app.add_systems(Update, (move_cube, follow_cube, draw_body).chain());

    app.world
        .spawn_empty()
        .insert(Name::new("Following"))
        .insert(RigidBody::Static)
        .insert(HasColor(Color::GREEN))
        .insert(Following)
        .insert(Collider::rectangle(100.0, 100.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(TransformBundle::from_transform(Transform::from_xyz(
            0.0, 100.0, 0.0,
        )));

    app.world
        .spawn_empty()
        .insert(Name::new("Follower"))
        .insert(RigidBody::Dynamic)
        .insert(HasColor(Color::GREEN))
        .insert(Collider::rectangle(100.0, 100.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(TransformBundle::from_transform(Transform::from_xyz(
            0.0, -100.0, 0.0,
        )));

    app.run();
}

fn draw_body(mut gizmos: Gizmos, characters: Query<(&GlobalTransform, &Collider, &HasColor)>) {
    for (transform, collider, &HasColor(color)) in characters.iter() {
        let cuboid = collider.shape().as_cuboid().unwrap().half_extents.xy();
        let size = Vec2::new(cuboid.x, cuboid.y);
        gizmos.rect(transform.translation(), Quat::IDENTITY, size * 2.0, color);
    }
}

fn move_cube(mut query: Query<&mut Transform, With<Following>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.translation.x = (time.elapsed_seconds() / 1.0).sin() * 400.0;
    }
}

fn follow_cube(
    mut follower: Query<&mut LinearVelocity, Without<Following>>,
    following: Query<(&GlobalTransform, &Transform), With<Following>>,
    time: Res<Time>,
) {
    for (
        mut vel,
        (
            gt,
            Transform {
                translation: Vec3 { x, .. },
                ..
            },
        ),
    ) in follower.iter_mut().zip(following.iter())
    {
        vel.x = (x - gt.translation().x) / time.delta_seconds();
        // This sleep represents the rest of the systems that need to run before Velocity is
        // actually applied. Without the sleep, the two objects move at the same speed.
        sleep(Duration::from_millis(50))
    }
}
