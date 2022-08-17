use bevy_rapier2d::prelude::*;
use std::{ptr::null, time::Duration};

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::{gamepad::GamepadButton, keyboard::KeyCode, Input},
    prelude::*,
    render::render_resource::{
        DepthStencilState, RenderPassDepthStencilAttachment,
        RenderPassDescriptor,
    },
    window::WindowMode,
};
use iyes_loopless::prelude::FixedTimestepStage;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;

const GAME_LOOP_UPDATE_RATE: u64 = 10;
const PLAYER_ACCELERATION: f32 = 1.0;
const PLAYER_DECELERATION: f32 = 2.0;
const PLAYER_MAX_SPEED: f32 = 5.0;
const PLAYER_MAX_JUMP_SPEED: f32 = 40.0;
const PLAYER_JUMP_DECELERATION: f32 = 3.0;

#[derive(Component)]
struct Platform;

#[derive(Component)]
struct Player {
    id: usize,
}

#[derive(Component)]
struct Speed {
    value: f32,
}

#[derive(Component)]
struct JumpSpeed {
    value: f32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum StatusTypes {
    Idle,
    RunLeft,
    RunRight,
    Jumping,
    Deceleration,
}
#[derive(Component)]
struct Statuses {
    value: Vec<StatusTypes>,
}

impl Statuses {
    fn add_status(&mut self, status_type: &StatusTypes) {
        self.value.push(*status_type);
        self.value.dedup();
    }

    fn remove_status(&mut self, status_type: &StatusTypes) {
        if let Some(index) =
            self.value.iter().position(|status| status == status_type)
        {
            self.value.swap_remove(index);
        }
    }
}

#[derive(PartialEq)]
enum ViewDirections {
    Left,
    Right,
}

#[derive(Component)]
struct ViewDirection {
    value: ViewDirections,
}

pub const LAUNCHER_TITLE: &str = "Bevy Shell - Template";

pub fn app() -> App {
    let mut game_loop = SystemStage::parallel();

    game_loop.add_system(teleport_wall_system);
    game_loop.add_system(move_player_system);

    // DEV tools
    game_loop.add_system(_dev_tp_player_to_start_system);

    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .add_plugins(DefaultPlugins)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
    .add_plugin(RapierDebugRenderPlugin::default())
    .add_startup_system(setup)
    .add_system(bevy::window::close_on_esc)
    .add_system(keyboard_input_system)
    .add_system(gamepad_system)
    .add_system(move_camera)
    .add_stage_before(
        CoreStage::Update,
        "game_loop",
        FixedTimestepStage::new(Duration::from_millis(GAME_LOOP_UPDATE_RATE))
            .with_stage(game_loop),
    )
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .run();

    return app;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());

    // ground
    commands
        .spawn()
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(WINDOW_WIDTH, 50.0))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0, 2200.0, 0.0,
        )));

    // Spawn player
    commands
        .spawn()
        .insert(Player { id: 1 })
        .insert(ViewDirection {
            value: ViewDirections::Right,
        })
        .insert(Statuses {
            value: vec![StatusTypes::Idle],
        })
        .insert(Speed { value: 0.0 })
        .insert(JumpSpeed {
            value: PLAYER_MAX_JUMP_SPEED,
        })
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(10.0))
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Collider::capsule(
            Vec2::new(0.0, 10.0),
            Vec2::new(0.0, 0.0),
            50.0,
        ))
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("axe_idle.png"),
            ..default()
        })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0, 0.0, 0.0,
        )));

    // Platform
    commands
        .spawn()
        .insert(Platform)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(200.0, 50.0))
        // .insert_bundle(SpriteBundle {
        //     texture: asset_server.load("sprites/platform.png"),
        //     ..default()
        // })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            0.0, -200.0, 0.0,
        )));

    commands
        .spawn()
        .insert(Platform)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(100.0, 20.0))
        // .insert_bundle(SpriteBundle {
        //     texture: asset_server.load("sprites/platform.png"),
        //     ..default()
        // })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -300.0, 80.0, 0.0,
        )));

    commands
        .spawn()
        .insert(Platform)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(100.0, 20.0))
        // .insert_bundle(SpriteBundle {
        //     texture: asset_server.load("sprites/platform.png"),
        //     ..default()
        // })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            -300.0, 230.0, 0.0,
        )));

    commands
        .spawn()
        .insert(Platform)
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(100.0, 20.0))
        // .insert_bundle(SpriteBundle {
        //     texture: asset_server.load("sprites/platform.png"),
        //     ..default()
        // })
        .insert_bundle(TransformBundle::from(Transform::from_xyz(
            400.0, -50.0, 0.0,
        )));
}

fn _dev_tp_player_to_start_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    if (keyboard_input.just_pressed(KeyCode::R)) {
        for (_, mut transform) in &mut query {
            transform.translation.x = 0.0;
            transform.translation.y = 0.0;
        }
    }
}

fn teleport_wall_system(mut query: Query<(&Player, &mut Transform)>) {
    for (_, mut transform) in &mut query {
        if transform.translation.x <= -WINDOW_WIDTH / 2.0 {
            transform.translation.x = WINDOW_WIDTH / 2.0;
        }

        if transform.translation.x >= WINDOW_WIDTH / 2.0 {
            transform.translation.x = -WINDOW_WIDTH / 2.0;
        }
    }
}

fn move_player_system(
    mut query: Query<(
        &Player,
        &mut Transform,
        &mut Speed,
        &mut Statuses,
        &ViewDirection,
        &mut JumpSpeed,
    )>,
) {
    for (
        _,
        mut transform,
        mut speed,
        mut statuses,
        view_direction,
        mut jump_speed,
    ) in &mut query
    {
        if statuses.value.contains(&StatusTypes::Jumping) {
            if (jump_speed.value > 0.0) {
                jump_speed.value -= PLAYER_JUMP_DECELERATION;
            } else if jump_speed.value <= 0.0 {
                // Don't jump anymore
                jump_speed.value = PLAYER_MAX_JUMP_SPEED;
                statuses.remove_status(&StatusTypes::Jumping);
            }

            transform.translation.y += jump_speed.value;
        }

        if statuses.value.contains(&StatusTypes::RunLeft)
            || statuses.value.contains(&StatusTypes::RunRight)
        {
            // Accelerate the player
            if speed.value <= PLAYER_MAX_SPEED {
                speed.value += PLAYER_ACCELERATION
            }
        } else {
            if speed.value > 0.0 {
                let new_value = speed.value - PLAYER_DECELERATION;

                if new_value < 0. {
                    speed.value = 0.;
                } else {
                    speed.value = new_value;
                }
            }
        }

        let move_speed = (speed.value * speed.value).sqrt();

        if view_direction.value == ViewDirections::Left {
            transform.translation.x -= move_speed;
        } else if view_direction.value == ViewDirections::Right {
            transform.translation.x += move_speed;
        }
    }
}

fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Statuses, &mut ViewDirection, &mut Sprite)>,
) {
    for (player, mut statuses, mut view_direction, mut sprite) in &mut query {
        if player.id == 1 {
            if keyboard_input.just_pressed(KeyCode::W) {
                if statuses.value.contains(&StatusTypes::Jumping) {
                    return;
                }

                statuses.value.push(StatusTypes::Jumping);
            }
            if keyboard_input.just_pressed(KeyCode::A) {
                sprite.flip_x = false;
                view_direction.value = ViewDirections::Left;
                statuses.value.push(StatusTypes::RunLeft);
            }
            if keyboard_input.just_pressed(KeyCode::D) {
                sprite.flip_x = true;
                view_direction.value = ViewDirections::Right;
                statuses.value.push(StatusTypes::RunRight);
            }
            if keyboard_input.just_released(KeyCode::A) {
                statuses.remove_status(&StatusTypes::RunLeft);
                statuses.value.push(StatusTypes::Deceleration);
            }
            if keyboard_input.just_released(KeyCode::D) {
                statuses.remove_status(&StatusTypes::RunRight);
                statuses.value.push(StatusTypes::Deceleration);
            }
        }

        statuses.value.dedup();
    }
}

// fn gamepad_system(
//     gamepads: Res<Gamepads>,
//     button_inputs: Res<Input<GamepadButton>>,
//     button_axes: Res<Axis<GamepadButton>>,
//     axes: Res<Axis<GamepadAxis>>,
//     mut query: Query<(&Player, &mut Statuses, &mut ViewDirection, &mut Sprite)>,
// ) {
//     for gamepad in gamepads.iter().cloned() {
//         for (player, mut statuses, mut view_direction, mut sprite) in &mut query
//         {
//             if button_inputs.just_pressed(GamepadButton {
//                 gamepad,
//                 button_type: GamepadButtonType::West,
//             }) {
//                 if statuses.value.contains(&StatusTypes::Jumping) {
//                     return;
//                 }

//                 statuses.value.push(StatusTypes::Jumping);
//             }

//             if button_inputs.just_pressed(GamepadButton {
//                 gamepad,
//                 button_type: GamepadButtonType::North,
//             }) {
//                 sprite.flip_x = false;
//                 view_direction.value = ViewDirections::Left;
//                 statuses.value.push(StatusTypes::RunLeft);
//             }
//             if button_inputs.just_pressed(GamepadButton {
//                 gamepad,
//                 button_type: GamepadButtonType::East,
//             }) {
//                 sprite.flip_x = true;
//                 view_direction.value = ViewDirections::Right;
//                 statuses.value.push(StatusTypes::RunRight);
//             }

//             if button_inputs.just_released(GamepadButton {
//                 gamepad,
//                 button_type: GamepadButtonType::North,
//             }) {
//                 statuses.remove_status(&StatusTypes::RunLeft);
//                 statuses.value.push(StatusTypes::Deceleration);
//             }

//             if button_inputs.just_released(GamepadButton {
//                 gamepad,
//                 button_type: GamepadButtonType::East,
//             }) {
//                 statuses.remove_status(&StatusTypes::RunRight);
//                 statuses.value.push(StatusTypes::Deceleration);
//             }

//             statuses.value.dedup();
//         }
//     }
// }

fn gamepad_system(
    gamepads: Res<Gamepads>,
    button_inputs: Res<Input<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    mut query: Query<(&Player, &mut Statuses, &mut ViewDirection, &mut Sprite)>,
) {
    for gamepad in gamepads.iter().cloned() {
        for (player, mut statuses, mut view_direction, mut sprite) in &mut query
        {
            if button_inputs.just_pressed(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::West,
            }) {
                if statuses.value.contains(&StatusTypes::Jumping) {
                    return;
                }

                statuses.value.push(StatusTypes::Jumping);
            }

            if button_inputs.just_pressed(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::LeftTrigger,
            }) {
                sprite.flip_x = false;
                view_direction.value = ViewDirections::Left;
                statuses.value.push(StatusTypes::RunLeft);
            }
            if button_inputs.just_pressed(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::RightTrigger,
            }) {
                sprite.flip_x = true;
                view_direction.value = ViewDirections::Right;
                statuses.value.push(StatusTypes::RunRight);
            }

            if button_inputs.just_released(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::LeftTrigger,
            }) {
                statuses.remove_status(&StatusTypes::RunLeft);
                statuses.value.push(StatusTypes::Deceleration);
            }

            if button_inputs.just_released(GamepadButton {
                gamepad,
                button_type: GamepadButtonType::RightTrigger,
            }) {
                statuses.remove_status(&StatusTypes::RunRight);
                statuses.value.push(StatusTypes::Deceleration);
            }

            statuses.value.dedup();
        }
    }
}

fn move_camera(
    mut query: Query<(Option<&Player>, Option<&Camera>, &mut Transform)>,
) {
    let mut entities = vec![];

    for (player, camera, mut transform) in query.iter_mut() {
        entities.push((player, camera, transform));
    }

    let mut player_translation: Vec3 = Vec3::default();

    for entity in entities {
        let player = entity.0;
        let camera = entity.1;

        // Get player translation
        if let Some(_) = player {
            let transform = entity.2;
            player_translation = transform.translation;
            continue;
        }

        // Move camera
        if let Some(_) = camera {
            let mut camera_transform = entity.2;
            camera_transform.translation = player_translation;
            continue;
        }
    }
}
