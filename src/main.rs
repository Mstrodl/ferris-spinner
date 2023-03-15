use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::camera::{Projection::*, ScalingMode};
use bevy::window::WindowMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
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
    .add_plugin(WorldInspectorPlugin::new())
    .add_systems((setup_system.on_startup(), hello_world_system, spin_system))
    .run();
}

fn setup_system(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  asset_server: Res<AssetServer>,
) {
  let crab = asset_server.load("ferris.glb#Scene0");
  // /Sketchfab_model/root/GLTFS_SceneRootNode/Armature_33");
  // ferris
  commands.spawn((
    SceneBundle {
      scene: crab,
      transform: Transform::from_xyz(0.0, -0.2, 0.0),
      ..default()
    },
    Ferris,
  ));

  commands.spawn(PbrBundle {
    mesh: meshes.add(shape::Box::new(10., 10., 5.).into()),

    transform: Transform::from_xyz(0., -5.25, -2.5),
    ..default()
  });

  // light
  commands.spawn(PointLightBundle {
    point_light: PointLight {
      intensity: 1500.0,
      ..default()
    },
    transform: Transform::from_xyz(3.0, 8.0, 3.0),
    ..default()
  });
  // camera
  commands.spawn((
    Camera3dBundle {
      projection: OrthographicProjection {
        scale: 1.0 / 1.5,
        scaling_mode: ScalingMode::FixedVertical(2.0),
        ..default()
      }
      .into(),
      transform: Transform::from_xyz(0., 0., 1.5).looking_at(Vec3::ZERO, Vec3::Y),
      ..default()
    },
    Camera3d,
  ));
  // 2d cam
  // commands.spawn((
  //   Camera2dBundle {
  //     camera: Camera {
  //       is_active: true,
  //       order: 10,
  //       ..default()
  //     },
  //     transform: Transform::from_xyz(0., 0., 1.5).looking_at(Vec3::ZERO, Vec3::Y),
  //     ..default()
  //   },
  //   Camera2d,
  // ));
}

#[derive(Component)]
struct Camera3d;
#[derive(Component)]
struct Camera2d;

const CAMERA_SPEED: f32 = PI / 1.;

const CRAB_SPEED: f32 = 0.5;
const ZOOM_SPEED: f32 = 0.5;

#[derive(Component)]
struct Ferris;

fn spin_system(
  time: Res<Time>,
  mut camera_transform: Query<(&mut Transform, &mut Projection, &mut Camera3d)>,
  devcade_controls: DevcadeControls,
  crab: Query<Entity, With<Ferris>>,
  mut commands: Commands,
  mut material: Query<&mut Handle<StandardMaterial>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  for (mut transform, mut projection, _) in &mut camera_transform {
    let forward = transform.forward().normalize() * ZOOM_SPEED * time.delta_seconds();

    let mut changed = false;
    if devcade_controls.pressed(Player::P2, Button::A1) {
      transform.translation += forward;
      changed = true;
    }
    if devcade_controls.pressed(Player::P2, Button::A2) {
      transform.translation -= forward;
      changed = true;
    }

    let mut x = 0.;
    let mut y = 0.;

    if devcade_controls.pressed(Player::P2, Button::StickLeft) {
      changed = true;
      x -= CAMERA_SPEED * time.delta_seconds();
    }
    if devcade_controls.pressed(Player::P2, Button::StickRight) {
      changed = true;
      x += CAMERA_SPEED * time.delta_seconds();
    }
    if devcade_controls.pressed(Player::P2, Button::StickDown) {
      changed = true;
      y += CAMERA_SPEED * time.delta_seconds();
    }
    if devcade_controls.pressed(Player::P2, Button::StickUp) {
      changed = true;
      y -= CAMERA_SPEED * time.delta_seconds();
    }

    if changed {
      if let Orthographic(_) = *projection {
        *projection = Perspective(PerspectiveProjection { ..default() });
        for material in &mut material {
          // println!("Found material: {:?}", material);
          let material = materials.get_mut(&material).unwrap();
          material.unlit = false;
        }
      }
    } else if let Orthographic(_) = *projection {
      for material in &mut material {
        // println!("Found material: {:?}", material);
        let material = materials.get_mut(&material).unwrap();
        material.unlit = true;
      }
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
