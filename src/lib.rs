use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::{
    App, Camera2dBundle, Commands, OrthographicProjection, Schedule,
    SystemStage, Transform,
};
use bevy::window::WindowDescriptor;
use bevy::DefaultPlugins;
use bevy_ggrs::GGRSPlugin;
use net::GGRSConfig;

const WINDOW_WIDTH: f32 = 1920.0;
const WINDOW_HEIGHT: f32 = 1080.0;
const FPS: f32 = 60.0;
const ROLLBACK_DEFAULT: &str = "rollback_default";

pub const LAUNCHER_TITLE: &str = "Dota Smash";

pub fn app() -> App {
    let mut app = App::new();

    GGRSPlugin::<GGRSConfig>::new()
        // define frequency of rollback game logic update
        .with_update_frequency(FPS as usize)
        // define system that returns inputs given a player handle, so GGRS can send the inputs around
        .with_input_system(players::input)
        // register types of components AND resources you want to be rolled back
        .register_rollback_type::<Transform>()
        // these systems will be executed as part of the advance frame update
        .with_rollback_schedule(
            Schedule::default().with_stage(
                ROLLBACK_DEFAULT,
                SystemStage::parallel()
                    .with_system(players::move_player_system)
                    .with_system(net::increase_frame_system),
            ),
        )
        // make it happen in the bevy app
        .build(&mut app);

    app.insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        ..Default::default()
    })
    .insert_resource(GameState {
        stage: GameStage::Init,
    })
    .insert_resource(net::FrameCount { frame: 0 })
    .add_plugins(DefaultPlugins)
    // .add_plugin(LogDiagnosticsPlugin::default())
    // .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_system(bevy::window::close_on_esc)
    .add_startup_system(setup_camera)
    .add_startup_system(net::setup_socket)
    .add_system(net::setup_session)
    .add_system(players::setup_players)
    .add_system(players::animate_players)
    .run();

    return app;
}

#[derive(PartialEq, Debug)]
pub enum GameStage {
    Init,
    SetupSocket,
    SetupSession,
    SpawnPlayers,
    Gameplay,
}

#[derive(Debug)]
pub struct GameState {
    stage: GameStage,
}

mod net {
    use bevy::{
        prelude::{info, Commands, Component, ResMut, Stage},
        reflect::{self, Reflect},
        tasks::IoTaskPool,
    };
    use bevy_ggrs::SessionType;
    use bytemuck::{Pod, Zeroable};
    use ggrs::{Config, PlayerType, SessionBuilder};
    use matchbox_socket::WebRtcSocket;

    use crate::{GameStage, GameState};

    #[repr(C)]
    #[derive(Copy, Clone, PartialEq, Pod, Zeroable)]
    pub struct BoxInput {
        pub inp: u8,
    }

    #[derive(Debug)]
    pub struct GGRSConfig;
    impl Config for GGRSConfig {
        type Input = BoxInput;
        type State = u8;
        type Address = String;
    }
    #[derive(Default, Reflect, Hash, Component)]
    #[reflect(Hash)]
    pub struct FrameCount {
        pub frame: u32,
    }

    pub fn setup_socket(
        mut commands: Commands,
        mut game_state: ResMut<GameState>,
    ) {
        if game_state.stage != GameStage::Init {
            return; // Nothing to do we are not in the init phase
        }

        game_state.stage = GameStage::SetupSocket; // Go into SetupSocket stage

        let room_url = "ws://192.168.2.170:3536/next_2";

        info!("Connecting to matchbox server: {:?}", room_url);

        let (socket, message_loop) = WebRtcSocket::new(room_url);

        // The message loop needs to be awaited, or nothing will happen.
        // We do this here using bevy's task system.
        IoTaskPool::get().spawn(message_loop).detach();

        commands.insert_resource(Some(socket));

        game_state.stage = GameStage::SetupSession; // Done creating socket, create session
    }

    pub fn setup_session(
        mut commands: Commands,
        mut game_state: ResMut<GameState>,
        mut socket: ResMut<Option<WebRtcSocket>>,
    ) {
        if game_state.stage != GameStage::SetupSession {
            return;
        }

        let socket = socket.as_mut();

        // Check for new connections
        socket.as_mut().unwrap().accept_new_connections();

        let players = socket.as_ref().unwrap().players();

        let num_players = 2;
        if players.len() < num_players {
            return; // wait for more playere
        }

        info!("All peers have joined, going in-game");
        // consume the socket (currently required because GGRS takes ownership of its socket)
        let socket = socket.take().unwrap();

        // create a GGRS P2P session
        let mut session_builder = SessionBuilder::<GGRSConfig>::new()
            .with_num_players(num_players)
            .with_max_prediction_window(12)
            .with_fps(60 as usize)
            .expect("Invalid FPS")
            .with_input_delay(2);

        let mut handles = Vec::new();
        for (i, player_type) in socket.players().iter().enumerate() {
            if *player_type == PlayerType::Local {
                handles.push(i);
            }
            session_builder = session_builder
                .add_player(player_type.clone(), i)
                .expect("Invalid player added.");
        }

        // start the GGRS session
        let session = session_builder.start_p2p_session(socket).unwrap();

        commands.insert_resource(session);
        commands.insert_resource(SessionType::P2PSession);

        game_state.stage = GameStage::SpawnPlayers;
    }

    pub fn increase_frame_system(mut frame_count: ResMut<FrameCount>) {
        frame_count.frame += 1;
    }
}

mod players {
    use bevy::{
        prelude::{
            info, AssetServer, Assets, Commands, Component, Deref, DerefMut,
            Handle, In, Input, KeyCode, Query, Res, ResMut, Transform, Vec2,
            Vec3, With,
        },
        sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
        time::{Time, Timer},
        transform::TransformBundle,
    };
    use bevy_ggrs::{Rollback, RollbackIdProvider};
    use ggrs::{InputStatus, P2PSession, PlayerHandle};

    use crate::{
        net::{BoxInput, GGRSConfig},
        GameStage, GameState,
    };

    const INPUT_UP: u8 = 1 << 0;
    const INPUT_DOWN: u8 = 1 << 1;
    const INPUT_LEFT: u8 = 1 << 2;
    const INPUT_RIGHT: u8 = 1 << 3;

    #[derive(Component, Deref, DerefMut)]
    pub struct AnimationTimer(Timer);

    #[derive(Component)]
    pub struct Player {
        pub handle: usize,
    }

    pub fn setup_players(
        mut commands: Commands,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut rip: ResMut<RollbackIdProvider>,
        mut session: Option<ResMut<P2PSession<GGRSConfig>>>,
        mut game_state: ResMut<GameState>,
    ) {
        if game_state.stage != GameStage::SpawnPlayers {
            return;
        }

        if session.is_none() {
            info!("Session is none...");
            return;
        }

        let num_players = session.unwrap().num_players();

        for handle in 0..num_players {
            let texture_handle = asset_server.load("venomancer_idle.png");
            let texture_atlas = TextureAtlas::from_grid(
                texture_handle,
                Vec2::new(100.0, 100.0),
                5,
                1,
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            let mut transform = Transform::default();

            match handle {
                0 => transform = Transform::from_xyz(-100.0, 0.0, 0.0),
                1 => transform = Transform::from_xyz(100.0, 0.0, 0.0),
                _ => (),
            }

            commands
                .spawn()
                .insert(Player { handle })
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    ..Default::default()
                })
                .insert(AnimationTimer(Timer::from_seconds(0.15, true)))
                .insert_bundle(TransformBundle::from(transform.with_scale(
                    Vec3 {
                        x: 2.0,
                        y: 2.0,
                        z: 1.0,
                    },
                )))
                .insert(Rollback::new(rip.next_id()));
        }

        game_state.stage = GameStage::Gameplay;
    }

    pub fn animate_players(
        time: Res<Time>,
        texture_atlases: Res<Assets<TextureAtlas>>,
        mut query: Query<(
            &Player,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &Handle<TextureAtlas>,
        )>,
    ) {
        for (player, mut timer, mut sprite, texture_atlas_handle) in &mut query
        {
            timer.tick(time.delta());
            if timer.just_finished() {
                let texture_atlas =
                    texture_atlases.get(texture_atlas_handle).unwrap();
                sprite.index =
                    (sprite.index + 1) % texture_atlas.textures.len();
            }
        }
    }

    pub fn input(
        _handle: In<PlayerHandle>,
        keyboard_input: Res<Input<KeyCode>>,
    ) -> BoxInput {
        let mut input: u8 = 0;

        if keyboard_input.pressed(KeyCode::W) {
            input |= INPUT_UP;
        }
        if keyboard_input.pressed(KeyCode::A) {
            input |= INPUT_LEFT;
        }
        if keyboard_input.pressed(KeyCode::S) {
            input |= INPUT_DOWN;
        }
        if keyboard_input.pressed(KeyCode::D) {
            input |= INPUT_RIGHT;
        }

        BoxInput { inp: input }
    }

    pub fn move_player_system(
        mut query: Query<(&mut Transform, &Player), With<Rollback>>,
        inputs: Res<Vec<(BoxInput, InputStatus)>>,
    ) {
        for (mut t, p) in query.iter_mut() {
            let input = inputs[p.handle as usize].0.inp;

            if input & INPUT_LEFT != 0 && input & INPUT_RIGHT == 0 {
                t.translation.x -= 50.;
            }
            if input & INPUT_LEFT == 0 && input & INPUT_RIGHT != 0 {
                t.translation.x += 50.;
            }
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 2.0,
            ..Default::default()
        },
        ..Default::default()
    });
}
