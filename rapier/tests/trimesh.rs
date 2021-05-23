#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{CollisionShape, RigidBody};
use heron_rapier::rapier::dynamics::IntegrationParameters;
use heron_rapier::RapierPlugin;

fn test_app() -> App {
    let mut builder = App::build();
    builder
        .init_resource::<TypeRegistryArc>()
        .add_plugin(CorePlugin)
        .add_plugin(RapierPlugin {
            step_per_second: None,
            parameters: IntegrationParameters::default(),
        });
    builder.app
}

#[test]
fn trimesh_collision_doesnt_crash() {
    let positions = vec![
        //Cube 1
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 1.0),
        Vec3::new(1.0, 1.0, 0.0),
        Vec3::new(1.0, 1.0, 1.0),
    ];

    let indices = vec![
        [0, 1, 2],
        [1, 3, 2],
        [1, 5, 3],
        [5, 7, 3],
        [5, 4, 7],
        [4, 6, 7],
        [4, 0, 6],
        [0, 2, 6],
    ];

    let mut app = test_app();

    app.world.spawn().insert_bundle((
        RigidBody::Dynamic,
        CollisionShape::TriMesh {
            positions: positions.clone(),
            indices: indices.clone(),
        },
        Transform::default(),
        GlobalTransform::default(),
    ));

    app.world.spawn().insert_bundle((
        RigidBody::Dynamic,
        CollisionShape::TriMesh {
            positions: positions.clone(),
            indices: indices.clone(),
        },
        Transform::default(),
        GlobalTransform::default(),
    ));

    // This shouldn't panic
    app.update();
}
