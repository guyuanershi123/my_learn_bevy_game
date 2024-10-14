use std::ops::{Add, Sub};
use bevy::app::{App, Startup, Update};
use bevy::asset::Assets;
use bevy::DefaultPlugins;
use bevy::ecs::query::QueryEntityError;
use bevy::input::keyboard::KeyboardInput;
use bevy::math::{vec2, Vec2};
use bevy::prelude::{ButtonInput, Camera2dBundle, Color, ColorMaterial, Commands, Component, EventReader, KeyCode, LightGizmoColor, Mesh, MeshBuilder, Plugin, Quat, Query, Res, ResMut, Transform, Triangle2d, Vec3, With};
use bevy::render::mesh::CircleMeshBuilder;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::time::{Time, Timer};

pub fn run() {
    App::new()
        .add_plugins((DefaultPlugins, Game))
        .run();
}

struct Game;

impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_player)
            .add_systems(Update, (move_dirction));
    }
}

#[derive(Component)]
struct Ship {
    speed: f32
}

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
    }, Ship{speed: 200.0}));
}

fn move_dirction(time: Res<Time>,
                mut keyboard: Res<ButtonInput<KeyCode>>,
                mut query: Query<(&mut Transform, &Ship)>) {
        for (mut transform, ship) in query.iter_mut() {
            let mut direction = Vec2::ZERO;
            let mut angle = 0.0;

            if keyboard.pressed(KeyCode::ArrowLeft) {
                direction.x -= 1.0;
            }
            else if keyboard.pressed(KeyCode::ArrowRight) {
                direction.x += 1.0;
            }
            else if keyboard.pressed(KeyCode::ArrowUp) {
                direction.y += 1.0;
            }
            else if keyboard.pressed(KeyCode::ArrowDown) {
                direction.y -= 1.0;
            }
            else if keyboard.pressed(KeyCode::KeyS) {
                angle = 1.0;
            }
            else if keyboard.pressed(KeyCode::KeyD) {
                angle = -1.0;
            }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation.x += direction.x * ship.speed * time.delta_seconds();
        transform.translation.y += direction.y * ship.speed * time.delta_seconds();
            transform.rotate_local_z(angle * time.delta_seconds());
    }
}
