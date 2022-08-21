use crate::{
    entities::player::{Player, ViewDirection},
    maps::map_one::Platform,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::tess::math::Translation;
use bevy_rapier2d::{
    prelude::{Collider, CollisionEvent, RigidBody, Velocity},
    rapier::prelude::CollisionEventFlags,
};

#[derive(Component)]
pub struct Fireball {
    pub player_id: usize,
}

impl Fireball {
    pub fn new_for_player(
        player: &Player,
        player_transform: &Transform,
        commands: &mut Commands,
        asset_server: &mut AssetServer,
    ) -> Self {
        let fireball = Self {
            player_id: player.id,
        };

        let texture_handle = asset_server.load("fireball.png");
        let mut velocity = Velocity::default();
        let mut transform = Transform::default();

        transform.translation = player_transform.translation;

        if player.view_direction == ViewDirection::Right {
            velocity.linvel.x = 1000.0;
            transform.translation.x += 50.0;
        } else {
            velocity.linvel.x = -1000.0;
            transform.translation.x -= 50.0;
        };

        commands
            .spawn()
            .insert_bundle(SpriteBundle {
                texture: texture_handle,
                transform: transform,
                ..Default::default()
            })
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(15.0))
            .insert(velocity);

        return fireball;
    }
}

pub struct FireballPlugin;
impl Plugin for FireballPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(fireball_system);
    }
}

fn fireball_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<(
        Entity,
        Option<&Platform>,
        Option<&Fireball>,
        Option<&Velocity>,
    )>,
) {
    for collision_event in collision_events.iter() {
        println!("WTF {:?}", collision_event);
    }

    let mut fireball_entities = Vec::<Entity>::new();
    let mut platform_entities = Vec::<Entity>::new();

    for (entity, platform, fireball, _) in query.iter() {
        if let Some(_) = fireball {
            fireball_entities.push(entity);
        } else if let Some(_) = platform {
            platform_entities.push(entity);
        }
    }

    for collision_event in collision_events.iter() {
        for fireball_entity in fireball_entities.iter() {
            for platform_entity in platform_entities.iter() {
                let possible_collision_event = CollisionEvent::Started(
                    *fireball_entity,
                    *platform_entity,
                    CollisionEventFlags::SENSOR,
                );

                if (collision_event == &possible_collision_event) {
                    commands.entity(*fireball_entity).despawn();
                }
            }
        }
    }
}
