//sistema de grid, essencial para todos

use bevy::{
    asset::Handle,
    pbr::StandardMaterial,
    prelude::{
        default, shape, Assets, BuildChildren, Bundle, Changed, Color, Commands, Component, Mesh,
        Name, PbrBundle, Plugin, Query, Res, ResMut, Resource, Transform,
    },
};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use bevy_mod_picking::PickableBundle;
use rand::Rng;

use crate::generic::BasicsAttributes;
use crate::generic::Team;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<GridXYZ>()
            .init_resource::<GridBackup>()
            .add_startup_system(create_grid)
            .add_system(update_cell_material)
            .register_inspectable::<TypeOfCell>()
            .register_inspectable::<TypeOfInteractiveCell>()
            .register_inspectable::<BasicsAttributes>()
            .register_inspectable::<CellAttributes>();
    }
}

impl BasicsAttributes {
    fn standart_pass_cell() -> Self {
        BasicsAttributes {
            hpp: (0, 0),
            movp: (0, 0),
            actp: (0, 0),
            efyp: (0, 0),
            ranp: (0, 0),
            height: (10, 0),
            area: (10, 0),
        }
    }
    fn standart_impa_cell() -> Self {
        BasicsAttributes {
            hpp: (0, 0),
            movp: (0, 0),
            actp: (0, 0),
            efyp: (0, 0),
            ranp: (0, 0),
            height: (0, 0),
            area: (0, 0),
        }
    }

    fn is_pass_attr(&self) -> bool {
        &self.height().0 - &self.height().1 != 0 && &self.area().0 - &self.area().1 != 0
    }
    fn is_pass_type(typo: &TypeOfCell) -> bool {
        if let TypeOfCell::Interactive(TypeOfInteractiveCell::Pass) = typo {
            return true;
        }
        return false;
    }
}

#[derive(Component, Inspectable)]
struct CellAttributes {
    //colocar um vetor aboveit depois
    blockvision: ([bool; 4], [Team; 4]),
    isfull: ([bool; 4], [Team; 4]),
}

impl CellAttributes {
    fn blockvision(&self) -> ([bool; 4], [Team; 4]) {
        self.blockvision
    }

    fn set_blockvision(&mut self, blockvision: ([bool; 4], [Team; 4])) {
        self.blockvision = blockvision;
    }

    fn isfull(&self) -> ([bool; 4], [Team; 4]) {
        self.isfull
    }

    fn set_isfull(&mut self, isfull: ([bool; 4], [Team; 4])) {
        self.isfull = isfull;
    }
}

impl Default for CellAttributes {
    fn default() -> Self {
        Self {
            blockvision: ([false; 4], [Team::Allies; 4]),
            isfull: ([false; 4], [Team::Allies; 4]),
        }
    }
}

#[derive(Component, Inspectable, Clone)]
pub enum TypeOfCell {
    Void,
    Interactive(TypeOfInteractiveCell),
    Coord,
}

impl Default for TypeOfCell {
    fn default() -> Self {
        TypeOfCell::Interactive(TypeOfInteractiveCell::Pass)
    }
}

#[derive(Inspectable, Default, Clone)]
pub enum TypeOfInteractiveCell {
    #[default]
    Pass,
    Impa,
}

#[derive(Resource)]
pub struct GridXYZ {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

impl Default for GridXYZ {
    fn default() -> Self {
        Self { x: 10, y: 0, z: 10 }
    }
}

#[derive(Default)]
struct CellBackup {
    typo: TypeOfCell,
    basicattr: Option<BasicsAttributes>,
    cellattr: Option<CellAttributes>,
    material: Handle<StandardMaterial>,
}

#[derive(Resource, Default)]
struct GridBackup {
    grid: Vec<Vec<CellBackup>>,
}

#[derive(Bundle)]
struct CellBundle {
    #[bundle]
    pbr: PbrBundle,
    #[bundle]
    pb: PickableBundle,
    typeofcell: TypeOfCell,
}

impl CellBundle {
    fn cell(
        material: Handle<StandardMaterial>,
        mesh: Handle<Mesh>,
        typo: TypeOfCell,
        transform: Transform
    ) -> CellBundle {
        CellBundle {
            pbr: PbrBundle {
                mesh,
                material,
                transform,
                ..default()
            },
            pb: PickableBundle::default(),
            typeofcell: typo,
        }
    }
}

#[derive(Bundle)]
struct InteractiveCellBundle {
    basicattr: BasicsAttributes,
    cellattr: CellAttributes,
}

impl InteractiveCellBundle {
    fn pass() -> Self {
        InteractiveCellBundle {
            basicattr: BasicsAttributes::standart_pass_cell(),
            cellattr: CellAttributes::default(),
        }
    }
    fn impa() -> Self {
        InteractiveCellBundle {
            basicattr: BasicsAttributes::standart_impa_cell(),
            cellattr: CellAttributes::default(),
        }
    }
}

fn create_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut grid: ResMut<GridBackup>,
    gridxyz: Res<GridXYZ>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 1. }));
    let material = [
        materials.add(Color::hex("1B3E3C").unwrap().into()),
        materials.add(Color::hex("FFC190").unwrap().into()),
        materials.add(Color::hex("902200").unwrap().into()),
    ];
    let father_grid = commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 1. })),
            material: materials.add(Color::rgba(0., 0., 0., 0.).into()),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        })
        .insert(Name::new("Grid"))
        .id();

    grid.grid = (0..gridxyz.x)
        .map(|i| {
            (0..gridxyz.z)
                .map(|j| {
                    let number = rand::thread_rng().gen_range(0..3);
                    let typo = TypeOfCell::Interactive(TypeOfInteractiveCell::Pass);
                    let transform= Transform::from_xyz(i as f32, 0., j as f32);
                    let cell = commands
                        .spawn(CellBundle::cell(
                            material[number].clone(),
                            mesh.clone(),
                            typo.clone(),
                            transform
                        ))
                        .insert(InteractiveCellBundle::pass())
                        .insert(Name::new(format!("Cell {},{}", i, j)))
                        .id();
                    commands.entity(father_grid).add_child(cell);
                    let interactive_cell = InteractiveCellBundle::pass();
                    CellBackup {
                        basicattr: Some(interactive_cell.basicattr),
                        cellattr: Some(interactive_cell.cellattr),
                        material: material[number].clone(),
                        typo,
                    }
                })
                .collect()
        })
        .collect();
}

fn update_cell_material(
    mut cell: Query<(&mut Handle<StandardMaterial>, &Transform, &TypeOfCell), Changed<TypeOfCell>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    backup: Res<GridBackup>,
) {
    for (mut material, transform, typeofcell) in &mut cell {
        match typeofcell {
            TypeOfCell::Void => {
                *material = materials.add(Color::rgba(0., 0., 0., 0.).into());
            }
            TypeOfCell::Interactive(interactive) => match interactive {
                TypeOfInteractiveCell::Pass => {
                    *material = backup.grid[transform.translation.x as usize]
                        [transform.translation.z as usize]
                        .material
                        .clone();
                }
                TypeOfInteractiveCell::Impa => {
                    *material = materials.add(Color::hex("4E0D00").unwrap().into())
                }
            },

            TypeOfCell::Coord => {
                *material = materials.add(Color::hex("FFFFFF").unwrap().into());
            }
        }
    }
}
