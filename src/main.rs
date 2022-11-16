use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::{
    Inspectable, InspectorPlugin, RegisterInspectable, WorldInspectorPlugin,
};
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .init_resource::<CameraSpeed>()
        .add_plugin(InspectorPlugin::<CameraSpeed>::new())
        .add_plugins(DefaultPickingPlugins)
        .init_resource::<Grid>()
        .add_startup_system(create_grid)
        .add_startup_system(spawn_light)
        .add_startup_system(spawn_cameras)
        .add_system(switch_camera)
        .add_system(move_camera)
        .add_system(update_cell_material)
        .register_inspectable::<TypeOfCell>()
        .run();
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
    commands
        .spawn(Camera3dBundle {
            camera: Camera {
                is_active: false,
                ..default()
            },
            projection: OrthographicProjection {
                scaling_mode: ScalingMode::FixedVertical(2.0),
                scale: 3.,
                ..default()
            }
            .into(),
            transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Name::new("Ortho Camera"))
        .insert(PickingCameraBundle::default());
}

fn spawn_light(mut commands: Commands) {
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(0., 8., 0.),
            ..default()
        })
        .insert(Name::new("Main light"));
}

// Esse sistema só funciona para 2 cameras!
fn switch_camera(keyboard_input: Res<Input<KeyCode>>, mut camera: Query<&mut Camera>) {
    if keyboard_input.just_pressed(KeyCode::V) {
        for mut i in &mut camera {
            if i.is_active {
                i.is_active = false;
            } else {
                i.is_active = true;
            }
        }
    }
}

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

#[derive(Component, Inspectable)]
enum TypeOfCell {
    Void,  // nao andavel e não renderizada
    Impa,  //nao andavel e renderizada
    Pass,  //andavel e renderizada
    Coord, //nao andavel e renderizada
}

#[derive(Default)]
struct StandardCell {
    typo: u8,
    material: Handle<StandardMaterial>,
}

#[derive(Resource, Default)]
struct Grid {
    grid: Vec<Vec<StandardCell>>,
}
const GRID_X: u8 = 10;
const GRID_Z: u8 = 10;

fn create_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut grid: ResMut<Grid>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));
    let material = [
        materials.add(Color::hex("1B3E3C").unwrap().into()),
        materials.add(Color::hex("FFC190").unwrap().into()),
        materials.add(Color::hex("902200").unwrap().into()),
    ];

    let father_grid = commands
        .spawn(PbrBundle {
            mesh: mesh.clone(),
            material: materials.add(Color::rgba(0., 0., 0., 0.).into()),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        })
        .insert(Name::new("Grid"))
        .id();

    grid.grid = (0..GRID_X)
        .map(|i| {
            (0..GRID_Z)
                .map(|j| {
                    let material_index = rand::thread_rng().gen_range(0..3);
                    let cell = commands
                        .spawn(PbrBundle {
                            mesh: mesh.clone(),
                            material: { material[material_index].clone() },
                            transform: Transform::from_xyz(i as f32, 0., j as f32),
                            ..default()
                        })
                        .insert(TypeOfCell::Pass)
                        .insert(Name::new(format!("Cell {},{}", i, j)))
                        .insert(PickableBundle::default())
                        .id();
                    commands.entity(father_grid).add_child(cell);
                    StandardCell {
                        typo: 3,
                        material: material[material_index].clone(),
                    }
                })
                .collect()
        })
        .collect();
}

fn update_cell_material(
    mut cell: Query<(&mut Handle<StandardMaterial>, &Transform, &TypeOfCell), Changed<TypeOfCell>>,
    grid: Res<Grid>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut material, transform, typeofcell) in &mut cell {
        match typeofcell {
            TypeOfCell::Void => {
                *material = materials.add(Color::rgba(0., 0., 0., 0.).into());
            }
            TypeOfCell::Impa => {
                *material = materials.add(Color::hex("4E0D00").unwrap().into());
            }
            TypeOfCell::Pass => {
                *material = grid.grid[transform.translation.x as usize]
                    [transform.translation.z as usize]
                    .material
                    .clone();
            }
            TypeOfCell::Coord => {
                *material = materials.add(Color::hex("FFFFFF").unwrap().into());
            }
        }
    }
}
