use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_kajiya::{EnvironmentSettings, KajiyaCamera, KajiyaCameraBundle};
use bevy_kajiya::{KajiyaRendererDefaultPlugins, KajiyaSceneDescriptor};
use dolly::prelude::{CameraRig, Position, Smooth, YawPitch};

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Bevy Kajiya Playground".to_string(),
            width: 1920.,
            height: 1080.,
            vsync: false,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(KajiyaSceneDescriptor {
            scene_file_name: "gas_stations".to_string(),
            ..Default::default()
        })
        .add_plugins(KajiyaRendererDefaultPlugins)
        .add_startup_system(setup_world)
        .add_system(sun_move)
        .add_system(rotator_system)
        .add_system(drive_camera)
        .run();
}

fn setup_world(mut commands: Commands, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();
    commands.spawn_bundle(KajiyaCameraBundle {
        camera: KajiyaCamera {
            aspect_ratio: window.requested_width() / window.requested_height(),
            ..Default::default()
        },
        ..Default::default()
    });

    let camera_rig = CameraRig::builder()
        .with(Position::new(dolly::glam::Vec3::ZERO))
        .with(YawPitch::new().rotation_quat(dolly::glam::Quat::IDENTITY))
        .with(Smooth::new_position_rotation(1.0, 1.0))
        .build();

    commands.insert_resource(camera_rig);
}

fn sun_move(time: Res<Time>, mut query: Query<&mut EnvironmentSettings, With<KajiyaCamera>>) {
    let scale = 0.00005;
    for mut env in query.iter_mut() {
        let theta = 180.0 * (time.time_since_startup().as_secs_f32() * scale).cos();
        let phi = 180.0 * (time.time_since_startup().as_secs_f32() * scale).sin();
        env.sun_theta_phi = (theta, phi);
    }
}

/// this component indicates what entities should rotate
#[derive(Component, Clone)]
struct Rotator;

/// rotates the parent, which will result in the child also rotating
fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Rotator>>) {
    for mut transform in query.iter_mut() {
        transform.rotation *= Quat::from_rotation_x(3.0 * time.delta_seconds());
    }
}

fn drive_camera(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_buttons: Res<Input<MouseButton>>,
    mut camera_rig: ResMut<CameraRig>,
    mut query: Query<&mut Transform, With<KajiyaCamera>>,
) {
    let time_delta_seconds: f32 = time.delta_seconds();

    let mut move_vec = Vec3::ZERO;
    let mut boost = 0.0;

    if keys.pressed(KeyCode::LShift) {
        boost = 1.0;
    }
    if keys.pressed(KeyCode::LControl) {
        boost = -1.0;
    }

    if keys.pressed(KeyCode::W) {
        move_vec.z -= 1.0;
    }
    if keys.pressed(KeyCode::S) {
        move_vec.z += 1.0;
    }
    if keys.pressed(KeyCode::A) {
        move_vec.x -= 1.0;
    }
    if keys.pressed(KeyCode::D) {
        move_vec.x += 1.0;
    }

    if keys.pressed(KeyCode::Q) {
        move_vec.y += 1.0;
    }
    if keys.pressed(KeyCode::E) {
        move_vec.y -= 1.0;
    }

    let mut mouse_delta = Vec2::ZERO;
    if mouse_buttons.pressed(MouseButton::Right) {
        for event in mouse_motion_events.iter() {
            mouse_delta += event.delta;
        }
    }

    let move_vec = camera_rig.final_transform.rotation * move_vec * 10.0f32.powf(boost);

    camera_rig
        .driver_mut::<Position>()
        .translate(move_vec * time_delta_seconds * 2.5);

    camera_rig
        .driver_mut::<YawPitch>()
        .rotate_yaw_pitch(-0.1 * mouse_delta.x, -0.1 * mouse_delta.y);

    camera_rig.update(time_delta_seconds);

    let mut camera_transform = query.iter_mut().next().unwrap();
    camera_transform.translation = camera_rig.final_transform.position;
    camera_transform.rotation = camera_rig.final_transform.rotation;
}
