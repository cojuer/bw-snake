use bevy::core::FixedTimestep;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Copy, Clone, Component, PartialEq, Eq)]
pub struct Pos {
    pub x: u32,
    pub y: u32,
}

#[derive(Component)]
pub struct Age(u32);

#[derive(Component)]
pub struct SnakeMeta {
    pub len: u32,
    pub dir: Direction,
}

#[derive(Component)]
pub struct Snake;

#[derive(Component)]
pub struct SnakeBody;

#[derive(Component)]
pub struct Food;

#[derive(Component)]
pub struct Blocker;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct Collision;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
            Self::Left => Self::Right,
        }
    }
}

pub struct Scene {
    x_size: u32,
    y_size: u32,
}

const Z_SNAKE: f32 = 10.0;

fn spawn_snake(mut commands: Commands, asset_server: Res<AssetServer>) {
    let snake_image = asset_server.load("images/snake.png");
    commands
        .spawn()
        .insert(Snake)
        .insert(SnakeMeta {
            len: 4,
            dir: Direction::Right,
        })
        .insert(Collision)
        .insert(Pos { x: 5, y: 5 })
        .insert_bundle(SpriteBundle {
            texture: snake_image,
            ..Default::default()
        });
}

fn spawn_snake_body(mut commands: Commands, asset_server: Res<AssetServer>, pos: &Pos) {
    let body_image = asset_server.load("images/snake.png");
    commands
        .spawn()
        .insert(SnakeBody)
        .insert(Age(0))
        .insert(Pos { x: pos.x, y: pos.y })
        .insert(Collision)
        .insert_bundle(SpriteBundle {
            texture: body_image,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, Z_SNAKE)),
            ..Default::default()
        });
}

#[derive(Hash, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
}

impl TileType {
    fn has_collision(&self) -> bool {
        match self {
            TileType::Wall => true,
            TileType::Floor => false,
        }
    }
}

pub struct TileFactory {
    err_material: Handle<Image>,
    materials: HashMap<TileType, Handle<Image>>,
}

impl TileFactory {
    pub fn new(asset_server: &Res<AssetServer>) -> Self {
        let mut materials = HashMap::new();
        materials.insert(TileType::Wall, asset_server.load("images/wall.png"));
        materials.insert(TileType::Floor, asset_server.load("images/floor.png"));
        Self {
            err_material: asset_server.load("images/err.png"),
            materials,
        }
    }

    pub fn spawn(&self, commands: &mut Commands, pos: Pos, tile: TileType) -> Entity {
        let material = self.materials.get(&tile).unwrap_or(&self.err_material);

        let mut ent_cmd = commands.spawn_bundle(SpriteBundle {
            texture: material.clone(),
            ..Default::default()
        });
        ent_cmd.insert_bundle((Tile, pos));
        if tile.has_collision() {
            ent_cmd.insert(Collision);
        }
        ent_cmd.id()
    }
}

const TILE_SIZE: u32 = 32;

fn update_position(mut query: Query<(&mut Transform, &Pos), Changed<Pos>>, scene: Res<Scene>) {
    // offset shows distance from border to the center of the scene
    let offset_x = (scene.x_size as f32 - 1.0) * (TILE_SIZE as f32) / 2.0;
    let offset_y = (scene.y_size as f32 - 1.0) * (TILE_SIZE as f32) / 2.0;

    for (mut transform, pos) in query.iter_mut() {
        // in bevy for 2D x=0,y=0 points to the center of the screen
        // we subtract offset so that center of the scene matches center of the screen
        transform.translation.x = (pos.x * TILE_SIZE) as f32 - offset_x;
        transform.translation.y = (pos.y * TILE_SIZE) as f32 - offset_y;
    }
}

fn create_basic_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tile_factory = TileFactory::new(&asset_server);
    let scene_size: u32 = 10;
    for i in 0..scene_size {
        for j in 0..scene_size {
            if i == 0 || j == 0 || i == scene_size - 1 || j == scene_size - 1 {
                tile_factory.spawn(&mut commands, Pos { x: i, y: j }, TileType::Wall);
            } else {
                tile_factory.spawn(&mut commands, Pos { x: i, y: j }, TileType::Floor);
            }
        }
    }

    let scene = Scene {
        x_size: 10,
        y_size: 10,
    };

    commands.insert_resource(scene);
}

fn move_snake(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&SnakeMeta, &mut Pos)>,
) {
    let (snake_meta, mut pos) = query.single_mut();
    let old_pos = *pos;
    match snake_meta.dir {
        Direction::Up => {
            pos.y += 1;
        }
        Direction::Down => {
            pos.y -= 1;
        }
        Direction::Left => {
            pos.x -= 1;
        }
        Direction::Right => {
            pos.x += 1;
        }
    }

    spawn_snake_body(commands, asset_server, &old_pos);
}

fn check_snake_collides(
    snake_query: Query<(Entity, &SnakeMeta, &Pos), Changed<Pos>>,
    collision_query: Query<(Entity, &Pos), With<Collision>>,
) {
    if snake_query.is_empty() {
        // currently system runs each tick and we are only interested in ticks
        // where snake moved
        return;
    }
    let (snake_id, _, snake_pos) = snake_query.single();
    for (ent_id, ent_pos) in collision_query.iter() {
        if snake_pos == ent_pos && snake_id != ent_id {
            println!("failed");
        }
    }
}

fn despawn_old(
    mut commands: Commands,
    mut body_query: Query<(Entity, &SnakeBody, &mut Age)>,
    snake_query: Query<&SnakeMeta>,
) {
    let snake_meta = snake_query.single();
    for (entity, _, mut age) in body_query.iter_mut() {
        age.as_mut().0 += 1;
        if age.0 + 1 == snake_meta.len {
            commands.entity(entity).despawn();
        }
    }
}

fn control_snake(mut snake_query: Query<&mut SnakeMeta>, inputs: Res<Input<KeyCode>>) {
    let mut snake_meta = snake_query.single_mut();

    if inputs.is_changed() {
        let mut tmp_dir = snake_meta.dir;
        match inputs.get_just_released().next() {
            Some(KeyCode::Up | KeyCode::W) => tmp_dir = Direction::Up,
            Some(KeyCode::Down | KeyCode::S) => tmp_dir = Direction::Down,
            Some(KeyCode::Left | KeyCode::A) => tmp_dir = Direction::Left,
            Some(KeyCode::Right | KeyCode::D) => tmp_dir = Direction::Right,
            _ => {}
        }

        if tmp_dir != snake_meta.dir.opposite() {
            snake_meta.dir = tmp_dir;
        }
    }
}

static POST_CMD: &str = "post_cmd";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(OrthographicCameraBundle::new_2d());
            commands.spawn_bundle(UiCameraBundle::default());
        })
        .add_startup_system(create_basic_scene)
        .add_startup_system(spawn_snake)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(control_snake)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(move_snake)
                .with_system(despawn_old.after(move_snake)),
        )
        .add_stage_after(CoreStage::Update, POST_CMD, SystemStage::parallel())
        .add_system_to_stage(POST_CMD, check_snake_collides)
        // Moving snake is implemented by spawning body segments, we need
        // either to set correct transform or to update transform after
        // spawning. As spawning happens in the end of the stage we run system
        // updating transforms in a new stage.
        .add_system_to_stage(POST_CMD, update_position)
        .run();
}
