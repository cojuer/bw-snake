use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component)]
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

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Scene {
    x_size: u32,
    y_size: u32,
}

fn add_snake(mut commands: Commands, asset_server: Res<AssetServer>) {
    let snake_image = asset_server.load("images/snake.png");
    commands
        .spawn()
        .insert(Snake)
        .insert(SnakeMeta {
            len: 3,
            dir: Direction::Right,
        })
        .insert(Pos { x: 5, y: 5 })
        .insert_bundle(SpriteBundle {
            texture: snake_image,
            ..Default::default()
        });
}

#[derive(Hash, PartialEq, Eq)]
pub enum TileType {
    Wall,
    Floor,
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

        commands
            .spawn_bundle(SpriteBundle {
                texture: material.clone(),
                ..Default::default()
            })
            .insert_bundle((Tile, pos, Collision))
            .id()
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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(|mut commands: Commands| {
            commands.spawn_bundle(OrthographicCameraBundle::new_2d());
            commands.spawn_bundle(UiCameraBundle::default());
        })
        .add_startup_system(create_basic_scene)
        .add_startup_system(add_snake)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_system(update_position)
        .run();
}
