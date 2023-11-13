use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::render::camera::{Projection::*, ScalingMode};
use bevy::window::WindowMode;
use bevy_hanabi::prelude::*;
// #[cfg(not(feature = "devcade"))]
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use devcaders::{Button, DevcadeControls, NfcTagRequestComponent, NfcUserRequestComponent, Player};

fn main() {
  let mut app = App::new();
  app
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(if cfg!(feature = "devcade") {
        Window {
          mode: WindowMode::Fullscreen,
          ..default()
        }
      } else {
        default()
      }),
      ..default()
    }))
    .add_plugins(HanabiPlugin);
  // #[cfg(not(feature = "devcade"))]
  // {
  //   app.add_plugins(WorldInspectorPlugin::new());
  // }
  app
    .add_systems(Update, (hello_world_system, spin_system, nfc_system))
    .add_systems(Startup, setup_system)
    .run();
}

#[derive(Component, Default)]
struct AudioEmitter;

fn setup_system(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut effects: ResMut<Assets<EffectAsset>>,
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

  let writer = ExprWriter::new();

  let set_position_modifier = SetPositionCone3dModifier {
    base_radius: writer.lit(0.).expr(),
    top_radius: writer.lit(1.).expr(),
    height: writer.lit(20.).expr(),
    dimension: ShapeDimension::Volume,
  };
  let set_velocity_sphere_modifier = SetVelocitySphereModifier {
    center: writer.lit(Vec3::ZERO).expr(),
    speed: writer.lit(10.0).expr(),
  };
  let set_attribute_modifier =
    SetAttributeModifier::new(Attribute::LIFETIME, writer.lit(5_f32).expr());
  let accel_modifier = AccelModifier::new(writer.lit(Vec3::Y * -3.).expr());
  let color_over_lifetime_modifier = ColorOverLifetimeModifier {
    gradient: color_gradient1,
  };
  let size_over_lifetime_modifier = SizeOverLifetimeModifier {
    gradient: size_gradient1,
    ..Default::default()
  };
  let effect1 = effects.add(
    EffectAsset::new(32768, Spawner::rate(500.0.into()), writer.finish())
      .with_name("emit:rate")
      .init(set_position_modifier)
      // Make spawned particles move away from the emitter origin
      .init(set_velocity_sphere_modifier)
      .init(set_attribute_modifier)
      .update(accel_modifier)
      .render(color_over_lifetime_modifier)
      .render(size_over_lifetime_modifier),
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

  commands.spawn((
    Transform::from_xyz(0.0, 0.0, 0.0),
    AudioEmitter::default(),
    AudioBundle {
      source: asset_server.load("shower.ogg"),
      settings: PlaybackSettings::LOOP.with_spatial(true),
    },
  ));
  let listener = SpatialListener::new(4.0);
  commands.spawn((SpatialBundle::default(), listener));

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

  commands.spawn((
    TextBundle::from_section("No user", Default::default()),
    UserText,
  ));
}

#[derive(Component)]
struct UserText;

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
  mut camera_transform: Query<
    (&mut Transform, &mut Projection, &mut Camera3d),
    Without<SpatialListener>,
  >,
  devcade_controls: DevcadeControls,
  mut material: Query<&mut Handle<StandardMaterial>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut spatial_listeners: Query<&mut Transform, With<SpatialListener>>,
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
          let material = materials.get_mut(&*material).unwrap();
          material.unlit = false;
        }
      }
    } else if let Orthographic(_) = *projection {
      for material in &mut material {
        let material = materials.get_mut(&*material).unwrap();
        material.unlit = true;
      }
    }

    transform.rotate_around(Vec3::ZERO, Quat::from_euler(EulerRot::YXZ, x, y, 0.));

    let mut listener_transform = spatial_listeners.single_mut();
    *listener_transform = *transform;
  }
}

fn hello_world_system(
  time: Res<Time>,
  mut sprite_position: Query<(&mut Transform, &Ferris), Without<AudioEmitter>>,
  devcade_controls: DevcadeControls,
  mut spatial_audio_emitters: Query<&mut Transform, With<AudioEmitter>>,
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
    for mut emitter_transform in &mut spatial_audio_emitters {
      *emitter_transform = transform.clone();
    }
  }
  // println!("hello world");
}

#[derive(Component, Deref, DerefMut)]
struct MyNfcTagRequest(NfcTagRequestComponent);
#[derive(Component, Deref, DerefMut)]
struct MyNfcUserRequest(NfcUserRequestComponent);
fn nfc_system(
  mut commands: Commands,
  mut tags_request: Query<(&mut MyNfcTagRequest, Entity)>,
  mut users_request: Query<(&mut MyNfcUserRequest, Entity)>,
  mut text: Query<&mut Text, With<UserText>>,
) {
  for (mut tags_request, id) in &mut tags_request {
    if let Some(tag) = tags_request.poll() {
      println!("Got a response! {tag:?}");
      commands.entity(id).despawn();
      if let Ok(Some(tag_id)) = tag {
        commands.spawn(MyNfcUserRequest(NfcUserRequestComponent::new(tag_id)));
      }
    } else {
      for mut text in &mut text {
        text.sections[0].value = "No user".to_owned();
      }
    }
  }
  for (mut users_request, id) in &mut users_request {
    if let Some(user) = users_request.poll() {
      // println!("Got a response! {user:?}");
      commands.entity(id).despawn();
      if let Ok(user) = user {
        let username = user["uid"].as_str().unwrap();
        println!("Username is: {username}");
        for mut text in &mut text {
          text.sections[0].value = format!("User: {username}");
        }
      }
    }
  }
  if tags_request.is_empty() && users_request.is_empty() {
    println!("Creating a new request...");
    commands.spawn(MyNfcTagRequest(NfcTagRequestComponent::new()));
  }
}
