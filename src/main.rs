use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin, InspectorPlugin};
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickableMesh, Selection};
use ebm::camera::CameraPlugin;
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "EBM - Efrim Battle Mode".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(CameraPlugin)
        .add_event::<JustClickedCellEvent>()
        .init_resource::<Grid>()
        .init_resource::<SelectedPlayer>()
        .add_plugin(InspectorPlugin::<SelectedPlayer>::new())
        .add_startup_system(create_grid)
        .add_startup_system(spawn_light)
        .add_startup_system(spawn_player)
        .add_system(update_cell_material)
        .add_system(select_player)
        .add_system(just_selected_cell_event_system)
        .add_system(move_player)
        .register_inspectable::<TypeOfCell>()
        .run();
}

fn spawn_light(mut commands: Commands) {
    commands
        .spawn(PointLightBundle {
            transform: Transform::from_xyz(0., 8., 0.),
            ..default()
        })
        .insert(Name::new("Main light"));
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

#[derive(Component, Inspectable)]
struct Player {
    health: i32,
    move_speed: f32,
    move_range: u8,
}

const PLAYER_Y: f32 = 0.3;

impl Default for Player {
    fn default() -> Self {
        Self {
            health: 15,
            move_speed: 1.,
            move_range: 6,
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::hex("A9A96B").unwrap().into()),
            transform: Transform::from_xyz((GRID_X / 2) as f32, PLAYER_Y, (GRID_Z / 2) as f32),
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(PickableBundle::default())
        .insert(Player::default());
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::hex("FFB54A").unwrap().into()),
            transform: Transform::from_xyz((GRID_X / 3) as f32, PLAYER_Y, (GRID_Z / 3) as f32),
            ..default()
        })
        .insert(Name::new("Player2"))
        .insert(PickableBundle::default())
        .insert(Player::default());
}

#[derive(Resource, Default,Inspectable)]
struct SelectedPlayer(Option<Entity>);

fn select_player(
    player_query: Query<
        (Entity, &Selection, ChangeTrackers<Selection>),
        (Changed<Selection>, With<PickableMesh>, With<Player>),
    >,
    mut selected_player: ResMut<SelectedPlayer>,
) {
    for (entity, selection, selection_change) in player_query.iter() {
        if selection_change.is_added() {
            continue;
        }
        if selection.selected() {
            selected_player.0 = Some(entity);
        } 
    }
}

struct JustClickedCellEvent(Transform);

fn just_selected_cell_event_system(
    mut selected_cell_event: EventWriter<JustClickedCellEvent>,
    selection_query: Query<
        (&Transform, &Selection, ChangeTrackers<Selection>),
        (Changed<Selection>, With<PickableMesh>, With<TypeOfCell>),
    >,
) {
    for (transform, selection, selection_change) in selection_query.iter() {
        if selection_change.is_added() {
            continue;
        }
        if selection.selected() {
            selected_cell_event.send(JustClickedCellEvent(*transform))
        }
    }
}

fn move_player(
    mut players: Query<(Entity, &mut Transform), With<Player>>,
    selected_player: Res<SelectedPlayer>,
    mut cell_event: EventReader<JustClickedCellEvent>,
) {
    for cell in cell_event.iter() {
        for (player, mut player_transform) in &mut players {
            if selected_player.0 == Some(player) {
                player_transform.translation = cell.0.translation;
                player_transform.translation.y = PLAYER_Y;
            }
        }
    }
}

//next
//selection system md
//grid sytem md 
//player system md 
//move player system md