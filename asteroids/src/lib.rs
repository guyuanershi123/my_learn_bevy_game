use bevy::app::{App, Startup, Update};
use bevy::asset::Assets;
use bevy::DefaultPlugins;
use bevy::ecs::query::QueryEntityError;
use bevy::input::keyboard::KeyboardInput;
use bevy::math::{vec2, Vec2};
use bevy::prelude::{Camera2dBundle, Color, ColorMaterial, Commands, Component, EventReader, LightGizmoColor, Mesh, MeshBuilder, Plugin, Query, Res, ResMut, Transform, Triangle2d, With};
use bevy::render::mesh::CircleMeshBuilder;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::time::Time;

pub fn run() {
    App::new()
        .add_plugins((DefaultPlugins, Game))
        .run();
}

struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_player)
            .add_systems(Update, (mesh_movement, move_dirction));
    }
}

#[derive(Component)]
struct Position(f32, f32);

fn add_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>) {

    // 2d camera
    //
    commands.spawn(Camera2dBundle::default());

    // rectangle shape
    //
    let shape = Mesh2dHandle(meshes.add(Triangle2d::new(
        Vec2::Y * 10.0,
        Vec2::new(-5.0, -10.0),
        Vec2::new(5.0, -10.0),
    )));

    // color
    let color = Color::WHITE;

    // generate rectangle
    //
    commands.spawn((MaterialMesh2dBundle{
        mesh: shape,
        material: materials.add(color),
        ..Default::default()
    }, Position(0.0,0.0)));
}

fn mesh_movement(time: Res<Time>, mut mesh_position: Query<(&mut Transform, &Position)>) {
    for (mut transform, position) in &mut mesh_position {
        transform.translation.x += position.0;
        transform.translation.y += position.1;
    }
}

fn move_dirction(mut keybordevent: EventReader<KeyboardInput>) {
    for event in keybordevent.read() {
        println!("{:?}", event);
    }
}
