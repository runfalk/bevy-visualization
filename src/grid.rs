use bevy::prelude::*;

pub const CELL_WIDTH: f32 = 0.5;
pub const CELL_DEPTH: f32 = 0.5;

pub const TRACK_WIDTH: f32 = 0.05;
pub const TRACK_HEIGHT: f32 = 0.025;

#[derive(Clone, Debug)]
pub struct GridPlugin(GridProperties);

#[derive(Clone, Debug)]
pub struct GridProperties {
    pub columns: usize,
    pub rows: usize,
    pub cell_width: f32,
    pub cell_depth: f32,
    pub track_width: f32,
    pub track_height: f32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl GridPlugin {
    pub fn new(columns: usize, rows: usize) -> Self {
        Self(GridProperties {
            columns,
            rows,
            cell_width: CELL_WIDTH,
            cell_depth: CELL_DEPTH,
            track_width: TRACK_WIDTH,
            track_height: TRACK_HEIGHT,
        })
    }
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl GridProperties {
    /// Get the world coorinate in the middle of the given grid coordinate
    pub fn coord_to_vec3(&self, c: &Coordinate) -> Vec3 {
        let cell_x_offset = self.track_width / 2.0 + self.cell_width / 2.0;
        let cell_z_offset = self.track_width / 2.0 + self.cell_depth / 2.0;
        Vec3::new(
            cell_x_offset + c.x as f32 * self.cell_width,
            0.0,
            cell_z_offset + c.y as f32 * self.cell_depth,
        )
    }
}

fn setup(
    mut commands: Commands,
    grid: Res<GridProperties>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material = materials.add(Color::rgb(0.2, 0.2, 0.2).into());

    for i in 0..=grid.columns {
        let b = shape::Box::new(
            grid.track_width,
            grid.track_height,
            grid.rows as f32 * grid.cell_depth,
        );
        let min_x = b.min_x;
        let min_z = b.min_z;
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(b)),
            material: material.clone(),
            transform: Transform::from_xyz(
                i as f32 * grid.cell_width - min_x,
                -grid.track_height / 2.0,
                -min_z,
            ),
            ..Default::default()
        });
    }

    for i in 0..=grid.rows {
        let b = shape::Box::new(
            grid.columns as f32 * grid.cell_width,
            grid.track_height,
            grid.track_width,
        );
        let min_x = b.min_x;
        let min_z = b.min_z;
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(b)),
            material: material.clone(),
            transform: Transform::from_xyz(
                -min_x,
                -grid.track_height / 2.0,
                i as f32 * grid.cell_depth - min_z,
            ),
            ..Default::default()
        });
    }
}

impl Plugin for GridPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(self.0.clone())
            .add_startup_system(setup.system());
    }
}
