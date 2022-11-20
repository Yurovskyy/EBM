use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use bevy_mod_picking::PickingCameraBundle;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSpeed>()
            .add_plugin(InspectorPlugin::<CameraSpeed>::new())
            .add_startup_system(spawn_cameras)
            // .add_system(switch_camera)
            .add_system(move_camera);
    }
}

#[derive(Resource, Inspectable)]
struct CameraSpeed(f32);

impl Default for CameraSpeed {
    fn default() -> Self {
        Self(2.)
    }
}

fn spawn_cameras(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(9., 9., 9.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Name::new("Main camera"))
        .insert(PickingCameraBundle::default());
    // commands
    //     .spawn(Camera3dBundle {
    //         camera: Camera {
    //             is_active: false,
    //             ..default()
    //         },
    //         projection: OrthographicProjection {
    //             scaling_mode: ScalingMode::FixedVertical(2.0),
    //             scale: 3.,
    //             ..default()
    //         }
    //         .into(),
    //         transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //         ..default()
    //     })
    //     .insert(Name::new("Ortho Camera"))
    //     .insert(PickingCameraBundle::default());
}

// Esse sistema só funciona para 2 cameras!
// fn switch_camera(keyboard_input: Res<Input<KeyCode>>, mut camera: Query<&mut Camera>) {
//     if keyboard_input.just_pressed(KeyCode::V) {
//         for mut i in &mut camera {
//             if i.is_active {
//                 i.is_active = false;
//             } else {
//                 i.is_active = true;
//             }
//         }
//     }
// }

fn move_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera: Query<(&mut Transform, &Camera)>,
    camera_speed: Res<CameraSpeed>,
    timer: Res<Time>,
) {
    if keyboard_input.pressed(KeyCode::W)
        || keyboard_input.pressed(KeyCode::S)
        || keyboard_input.pressed(KeyCode::A)
        || keyboard_input.pressed(KeyCode::D)
    {
        for (mut transform, i) in &mut camera {
            if i.is_active {
                for j in keyboard_input.get_pressed() {
                    match j {
                        KeyCode::W => {
                            let mut foward = transform.forward();
                            foward.y = 0.;
                            transform.translation +=
                                foward * camera_speed.0 * timer.delta_seconds();
                        }
                        KeyCode::S => {
                            let mut back = transform.back();
                            back.y = 0.;
                            transform.translation += back * camera_speed.0 * timer.delta_seconds();
                        }
                        KeyCode::A => {
                            let mut left = transform.left();
                            left.y = 0.;
                            transform.translation += left * camera_speed.0 * timer.delta_seconds();
                        }
                        KeyCode::D => {
                            let mut right = transform.right();
                            right.y = 0.;
                            transform.translation += right * camera_speed.0 * timer.delta_seconds();
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
