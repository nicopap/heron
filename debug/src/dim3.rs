use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use heron_core::{CollisionShape, RigidBody, SensorShape};

use crate::shape3d_wireframe::{
    add_capsule, add_convex_hull, add_cuboid, add_height_field, add_rounded_cuboid, add_sphere,
};

use super::DebugColor;

#[allow(clippy::type_complexity)]
fn add_shape_outlines(
    shapes: Query<
        '_,
        (
            &CollisionShape,
            &Transform,
            Option<&GlobalTransform>,
            Option<&RigidBody>,
            Option<&SensorShape>,
            Option<&Parent>,
        ),
    >,
    transforms: Query<'_, &GlobalTransform>,
    color: Res<'_, DebugColor>,
    mut lines: ResMut<'_, DebugLines>,
) {
    for (shape, local_transform, trans, rigid_body_option, sensor_option, parent) in shapes.iter() {
        let (origin, orient) = if let Some(t) = trans {
            (t.translation, t.rotation)
        } else {
            // May refactor when https://github.com/rust-lang/rust/issues/87335 merges
            let parent = if let Some(Parent(p)) = parent {
                p
            } else {
                continue;
            };
            let parent_global = if let Ok(p) = transforms.get(*parent) {
                p
            } else {
                continue;
            };
            let origin = parent_global.translation + local_transform.translation;
            let orient = parent_global.rotation * local_transform.rotation;
            (origin, orient)
        };
        let color = color.for_collider_type(rigid_body_option, sensor_option.is_some());
        match shape {
            CollisionShape::Cuboid {
                half_extends,
                border_radius,
            } => match border_radius {
                Some(bevel) => {
                    add_rounded_cuboid(origin, orient, *half_extends, *bevel, color, &mut lines);
                }
                None => {
                    add_cuboid(origin, orient, *half_extends, color, &mut lines);
                }
            },
            CollisionShape::Sphere { radius } => {
                add_sphere(origin, orient, *radius, color, &mut lines);
            }
            CollisionShape::Capsule {
                half_segment,
                radius,
            } => add_capsule(origin, orient, *half_segment, *radius, color, &mut lines),
            CollisionShape::ConvexHull {
                points,
                border_radius: _,
            } => {
                // NOTE: won't work with ConvexHull with border_radius set,
                // absolutely no idea how to handle it here
                add_convex_hull(origin, orient, points, color, &mut lines);
            }
            CollisionShape::HeightField { size, heights } => {
                add_height_field(origin, orient, *size, heights, color, &mut lines);
            }
        }
    }
}

pub(crate) fn systems() -> SystemSet {
    SystemSet::new().with_system(add_shape_outlines.system())
}
