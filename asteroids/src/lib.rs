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
            .add_systems(Update, (move_direction, rotate_ship, thrust_ship));
    }
}

#[derive(Component)]
struct Ship {
    speed: f32,
    rotate_speed: f32,
    velocity: bool,
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
    }, Ship{speed: 0.0, rotate_speed: 1.0, velocity: false}));
}

fn move_direction(keyboard: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Ship>) {
        for mut ship in query.iter_mut() {
            if keyboard.pressed(KeyCode::ArrowLeft) {
                ship.rotate_speed -= 1.0;
            }
            else if keyboard.pressed(KeyCode::ArrowRight) {
                ship.rotate_speed += 1.0;
            }
            else if keyboard.pressed(KeyCode::ArrowUp) {
                ship.velocity = true;
                ship.speed += 1.0;
                ship.rotate_speed = 0.0;
            }
            else if keyboard.pressed(KeyCode::ArrowDown) {
                // ship.velocity = true;
                // ship.speed -= 1.0;
                // ship.rotate_speed = 0.0;;
            }
            else{
                ship.velocity = false;
                ship.rotate_speed = 0.0;
                ship.speed = 0.0;
            }

        ship.rotate_speed = ship.rotate_speed.clamp(-5.0, 5.0);
        ship.speed = ship.speed.clamp(0.0, 5.0);
    }
}

fn rotate_ship(time: Res<Time>, mut query: Query<(&mut Transform, &Ship)>) {
    for (mut transform, ship) in query.iter_mut() {
        transform.rotate_z(ship.rotate_speed * time.delta_seconds());
    }
}

fn thrust_ship(mut query: Query<(&mut Transform, &Ship)>) {
    for (mut transform, ship) in query.iter_mut() {
        if ship.velocity {
            let direction = transform.rotation * Vec3::Y;
            let vec = ship.speed * direction;
            transform.translation += vec.normalize();
        }
    }
}
