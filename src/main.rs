use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable, WorldInspectorPlugin, InspectorPlugin};
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickableMesh, Selection};
use ebm::camera::CameraPlugin;
use ebm::grid::{GridPlugin,GridXYZ,TypeOfCell};

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
        .add_plugin(GridPlugin)
        .add_event::<JustClickedCellEvent>()
        .init_resource::<SelectedPlayer>()
        .add_plugin(InspectorPlugin::<SelectedPlayer>::new())
        .add_startup_system(spawn_light)
        .add_startup_system(spawn_player)
        .add_system(select_player)
        .add_system(just_selected_cell_event_system)
        .add_system(move_player)
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
    grid: Res<GridXYZ>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::hex("A9A96B").unwrap().into()),
            transform: Transform::from_xyz((grid.x / 2) as f32, PLAYER_Y, (grid.z / 2) as f32),
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(PickableBundle::default())
        .insert(Player::default());
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.5 })),
            material: materials.add(Color::hex("FFB54A").unwrap().into()),
            transform: Transform::from_xyz((grid.x / 3) as f32, PLAYER_Y, (grid.z / 3) as f32),
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