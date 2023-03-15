use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::camera::{Projection::*, ScalingMode};
use bevy::window::WindowMode;
use bevy_hanabi::prelude::*;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use devcaders::{Button, DevcadeControls, Player};

fn main() {
  let mut app = App::new();
  app
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        mode: WindowMode::Fullscreen,
        ..default()
      }),
      ..default()
    }))
    .add_plugin(HanabiPlugin);
  #[cfg(debug_assertions)]
  {
    app.add_plugin(WorldInspectorPlugin::new());
  }
  app
    .add_systems((setup_system.on_startup(), hello_world_system, spin_system))
    .run();
}

#[derive(Resource)]
struct AudioController(Handle<SpatialAudioSink>);

fn setup_system(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut effects: ResMut<Assets<EffectAsset>>,
  audio: Res<Audio>,
  mut spatial_audio_sinks: Res<Assets<SpatialAudioSink>>,
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

  let mut color_gradient1 = Gradient::new();
  color_gradient1.add_key(0., Vec4::new(0.0, 0.0, 1., 1.0));
  color_gradient1.add_key(1., Vec4::new(0.0, 0.0, 1., 1.0));

  let mul = 3.;
  let mut size_gradient1 = Gradient::new();
  size_gradient1.add_key(0.0, Vec2::splat(0.1 / mul));
  size_gradient1.add_key(0.5, Vec2::splat(0.1 / mul));
  size_gradient1.add_key(0.8, Vec2::splat(0.08 / mul));
  size_gradient1.add_key(1.0, Vec2::splat(0.0));

  let effect1 = effects.add(
    EffectAsset {
      name: "emit:rate".to_string(),
      capacity: 32768,
      spawner: Spawner::rate(500.0.into()),
      ..Default::default()
    }
    .with_property("my_accel", graph::Value::Float3(Vec3::new(0., -3., 0.)))
    .init(InitPositionCone3dModifier {
      base_radius: 0.,
      top_radius: 1.,
      height: 20.,
      dimension: ShapeDimension::Volume,
    })
    // Make spawned particles move away from the emitter origin
    .init(InitVelocitySphereModifier {
      center: Vec3::ZERO,
      speed: 10.0.into(),
    })
    .init(InitLifetimeModifier {
      lifetime: 5_f32.into(),
    })
    .update(AccelModifier::constant(Vec3::Y * -3.))
    .render(ColorOverLifetimeModifier {
      gradient: color_gradient1,
    })
    .render(SizeOverLifetimeModifier {
      gradient: size_gradient1,
    }),
  );

  commands.spawn((
    Name::new("emit:rate"),
    ParticleEffectBundle {
      effect: ParticleEffect::new(effect1),
      transform: Transform::from_translation(Vec3::new(0., 10., 0.))
        .with_rotation(Quat::from_rotation_x(-PI)),
      ..Default::default()
    },
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
  let camera_transform = Transform::from_xyz(0., 0., 1.5).looking_at(Vec3::ZERO, Vec3::Y);
  commands.spawn((
    Camera3dBundle {
      projection: OrthographicProjection {
        scale: 1.0 / 1.5,
        scaling_mode: ScalingMode::FixedVertical(2.0),
        ..default()
      }
      .into(),
      transform: camera_transform,
      ..default()
    },
    Camera3d,
  ));

  let spatial = audio.play_spatial(
    asset_server.load("shower.ogg"),
    camera_transform,
    0.5,
    Vec3::ZERO,
  );
  let spatial = spatial_audio_sinks.get_handle(spatial);
  commands.insert_resource(AudioController(spatial));
  // spatial_audio_sinks.get(&spatial).unwrap();
  // spatial_audio_sinks.add(spatial.unwrap());

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
  mut material: Query<&mut Handle<StandardMaterial>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  spatial_audio_sinks: Res<Assets<SpatialAudioSink>>,
  audio_controller: Res<AudioController>,
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
          let material = materials.get_mut(&material).unwrap();
          material.unlit = false;
        }
      }
    } else if let Orthographic(_) = *projection {
      for material in &mut material {
        let material = materials.get_mut(&material).unwrap();
        material.unlit = true;
      }
    }

    transform.rotate_around(Vec3::ZERO, Quat::from_euler(EulerRot::YXZ, x, y, 0.));

    if let Some(spatial_sink) = spatial_audio_sinks.get(&audio_controller.0) {
      spatial_sink.set_listener_position(*transform, 0.25);
    }
  }
}

fn hello_world_system(
  time: Res<Time>,
  mut sprite_position: Query<(&mut Transform, &Ferris)>,
  devcade_controls: DevcadeControls,
  spatial_audio_sinks: Res<Assets<SpatialAudioSink>>,
  audio_controller: Res<AudioController>,
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
    // if let Some(spatial_sink) = spatial_audio_sinks.get(&audio_controller.0) {
    //   spatial_sink.set_emitter_position(transform.translation);
    // }
  }
  // println!("hello world");
}
