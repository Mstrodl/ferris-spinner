use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::window::WindowMode;
use devcaders::{Button, DevcadeControls, Player};

fn main() {
  App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        mode: WindowMode::Fullscreen,
        ..default()
      }),
      ..default()
    }))
    .add_systems((setup_system.on_startup(), hello_world_system, spin_system))
    .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
  let crab = asset_server.load("ferris.glb#Scene0");
  // ferris
  commands.spawn((
    SceneBundle {
      scene: crab,
      transform: Transform::from_xyz(0.0, -0.2, 0.0),
      ..default()
    },
    Ferris,
  ));
  // light
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 1500.0,
      shadows_enabled: true,
      ..default()
    },
    transform: Transform::from_xyz(4.0, 8.0, 4.0),
    ..default()
  });
  // camera
  commands.spawn((
    Camera3dBundle {
      transform: Transform::from_xyz(0., 0., 1.5).looking_at(Vec3::ZERO, Vec3::Y),
      ..default()
    },
    Camera { x: 0., y: 0. },
  ));
}

#[derive(Component, Debug)]
struct Camera {
  pub x: f32,
  pub y: f32,
}

const CAMERA_SPEED: f32 = PI / 1.;

const CAM_DISTANCE: f32 = 1.;

const CRAB_SPEED: f32 = 0.05;
const ZOOM_SPEED: f32 = 0.5;

#[derive(Component)]
struct Ferris;

fn spin_system(
  time: Res<Time>,
  mut camera_transform: Query<(&mut Transform, &mut Camera)>,
  devcade_controls: DevcadeControls,
) {
  for (mut transform, _) in &mut camera_transform {
    let forward = transform.forward().normalize() * ZOOM_SPEED * time.delta_seconds();

    if devcade_controls.pressed(Player::P2, Button::A1) {
      transform.translation += forward;
    }
    if devcade_controls.pressed(Player::P2, Button::A2) {
      transform.translation -= forward;
    }

    let mut x = 0.;
    let mut y = 0.;

    if devcade_controls.pressed(Player::P2, Button::StickLeft) {
      x -= CAMERA_SPEED * time.delta_seconds();
    }
    if devcade_controls.pressed(Player::P2, Button::StickRight) {
      x += CAMERA_SPEED * time.delta_seconds();
    }
    if devcade_controls.pressed(Player::P2, Button::StickDown) {
      y += CAMERA_SPEED * time.delta_seconds();
    }
    if devcade_controls.pressed(Player::P2, Button::StickUp) {
      y -= CAMERA_SPEED * time.delta_seconds();
    }

    transform.rotate_around(Vec3::ZERO, Quat::from_euler(EulerRot::YXZ, x, y, 0.));
  }
}

fn hello_world_system(
  time: Res<Time>,
  mut sprite_position: Query<(&mut Transform, &Ferris)>,
  devcade_controls: DevcadeControls,
) {
  if devcade_controls.pressed(Player::P1, Button::Menu)
    || devcade_controls.pressed(Player::P2, Button::Menu)
  {
    std::process::exit(0);
  }

  for (mut transform, _) in &mut sprite_position {
    // println!("Transform.translation: {:?}", transform.translation);
    if devcade_controls.pressed(Player::P1, Button::StickLeft) {
      transform.translation.x -= CRAB_SPEED * time.delta_seconds();
    }
    if devcade_controls.pressed(Player::P1, Button::StickRight) {
      transform.translation.x += CRAB_SPEED * time.delta_seconds();
    }
    if devcade_controls.pressed(Player::P1, Button::StickDown) {
      transform.translation.y -= CRAB_SPEED * time.delta_seconds();
    }
    if devcade_controls.pressed(Player::P1, Button::StickUp) {
      transform.translation.y += CRAB_SPEED * time.delta_seconds();
    }
  }
  // println!("hello world");
}
