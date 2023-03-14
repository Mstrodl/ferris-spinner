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
    .add_systems((setup_system.on_startup(), hello_world_system))
    .run();
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.spawn(Camera2dBundle::default());
  commands.spawn((
    Ferris {},
    SpriteBundle {
      texture: asset_server.load("ferris.png"),
      transform: Transform::from_xyz(100., 0., 0.),
      ..default()
    },
  ));
}

#[derive(Component)]
struct Ferris {}

fn hello_world_system(
  time: Res<Time>,
  mut sprite_position: Query<(&mut Ferris, &mut Transform)>,
  devcade_controls: DevcadeControls,
) {
  for (_, mut transform) in &mut sprite_position {
    if devcade_controls.pressed(Player::P1, Button::StickLeft) {
      transform.translation.x -= 200.0 * time.delta_seconds();
    }
    if devcade_controls.pressed(Player::P1, Button::StickRight) {
      transform.translation.x += 200.0 * time.delta_seconds();
    }
    if devcade_controls.pressed(Player::P1, Button::StickDown) {
      transform.translation.y -= 200.0 * time.delta_seconds();
    }
    if devcade_controls.pressed(Player::P1, Button::StickUp) {
      transform.translation.y += 200.0 * time.delta_seconds();
    }
  }
  // println!("hello world");
}
