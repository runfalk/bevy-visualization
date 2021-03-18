use bevy::{prelude::*, utils::Duration};
use rand::distributions::{Distribution, Uniform};
use rand::Rng;

use crate::grid;
use crate::rng;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BotPlugin(BotProperties);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BotProperties {
    pub num_bots: usize,
}

pub struct BotMoveTimer(Timer);

#[derive(Clone, Debug, PartialEq, Eq)]
enum AxialCoordinate {
    X(usize),
    Y(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub struct AxialMove {
    origin: grid::Coordinate,
    target: AxialCoordinate,
    start_time: Duration,
    duration: Duration,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BotState {
    Idle(grid::Coordinate),
    Move(AxialMove),
}

impl BotPlugin {
    pub fn new(num_bots: usize) -> Self {
        Self(BotProperties { num_bots })
    }
}

impl AxialCoordinate {
    fn apply(&self, c: &grid::Coordinate) -> grid::Coordinate {
        match *self {
            AxialCoordinate::X(x) => grid::Coordinate { x, y: c.y },
            AxialCoordinate::Y(y) => grid::Coordinate { x: c.x, y },
        }
    }

    /// Number of cells we need to move between this and the given coordinate
    fn distance(&self, c: &grid::Coordinate) -> usize {
        match *self {
            AxialCoordinate::X(x) => x.max(c.x) - x.min(c.x),
            AxialCoordinate::Y(y) => y.max(c.y) - y.min(c.y),
        }
    }
}

/// Spawn all bots
pub fn setup(
    mut commands: Commands,
    bots: Res<BotProperties>,
    grid: Res<grid::GridProperties>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: Local<rng::Rng>,
) {
    commands.insert_resource(BotMoveTimer(Timer::from_seconds(1.0, true)));

    let columns_dist = Uniform::from(0..grid.columns);
    let rows_dist = Uniform::from(0..grid.rows);

    let material = materials.add(Color::rgb(1.0, 0.6, 0.0).into());
    for _ in 0..bots.num_bots {
        let c = grid::Coordinate::new(
            columns_dist.sample(&mut **rng),
            rows_dist.sample(&mut **rng),
        );
        let b = shape::Box::new(
            grid.cell_width - grid.track_width / 2.0,
            0.3,
            grid.cell_depth - grid.track_width / 2.0,
        );
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(b)),
                material: material.clone(),
                ..Default::default()
            })
            .with(BotState::Idle(c));
    }
}

/// Position bots according to their state
fn bot_position(
    time: Res<Time>,
    grid: Res<grid::GridProperties>,
    mut query: Query<(&BotState, &mut Transform)>,
) {
    for (bot_state, mut transform) in query.iter_mut() {
        match *bot_state {
            BotState::Idle(ref c) => {
                (*transform.translation) = *(Vec3::new(0.0, 0.35, 0.0) + grid.coord_to_vec3(c));
            }
            BotState::Move(ref am) => {
                let now = time.time_since_startup();
                let origin_vec = grid.coord_to_vec3(&am.origin);
                if am.start_time >= now {
                    (*transform.translation) = *(Vec3::new(0.0, 0.35, 0.0) + origin_vec);
                } else {
                    let delta = time.time_since_startup() - am.start_time;
                    let target_vec = grid.coord_to_vec3(&am.target.apply(&am.origin));
                    (*transform.translation) = *(Vec3::new(0.0, 0.35, 0.0)
                        + origin_vec.lerp(target_vec, delta.as_secs_f32().min(1.0)));
                }
            }
        }
    }
}

/// Trigger movement for idle bots periodically
fn bot_mover(
    time: Res<Time>,
    grid: Res<grid::GridProperties>,
    mut timer: ResMut<BotMoveTimer>,
    mut query: Query<&mut BotState>,
    mut rng: Local<rng::Rng>,
) {
    // Trigger bot movements once every second
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let delay_dist = Uniform::from(0..=1000);
    let ms_per_cell_dist = Uniform::from(500..1200);

    let mut num_idle = 0;
    let mut num_active = 0;

    for mut bot_state in query.iter_mut() {
        match &mut *bot_state {
            BotState::Idle(ref c) => {
                num_idle += 1;
                let ms_per_cell = ms_per_cell_dist.sample(&mut **rng);
                let x: bool = rng.gen();
                let target = if x {
                    AxialCoordinate::X(
                        Uniform::from((c.x.max(10) - 10)..=(c.x.min(grid.columns - 11) + 10))
                            .sample(&mut **rng),
                    )
                } else {
                    AxialCoordinate::Y(
                        Uniform::from((c.y.max(10) - 10)..=(c.y.min(grid.rows - 11) + 10))
                            .sample(&mut **rng),
                    )
                };
                let duration = Duration::from_millis((ms_per_cell * target.distance(&c)) as u64);

                *bot_state = BotState::Move(AxialMove {
                    origin: c.clone(),
                    target,
                    start_time: time.time_since_startup() + Duration::from_millis(delay_dist.sample(&mut **rng)),
                    duration,
                });
            }
            BotState::Move(ref am) => {
                num_active += 1;
                // If a bot has finished moving we mark it as idle. Due to the timer this means
                // that a bot is idle for at least one second.
                if time.time_since_startup() > am.start_time + am.duration {
                    *bot_state = BotState::Idle(am.target.apply(&am.origin));
                }
            }
        }
    }
    println!("{} idle bots, {} active bots", num_idle, num_active);
}

impl Plugin for BotPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(self.0.clone())
            .add_startup_system(setup.system())
            .add_system(bot_position.system())
            .add_system(bot_mover.system());
    }
}
