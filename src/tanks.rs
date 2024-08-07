// TODO: Fix bearing using a sprite bundle

use crate::utils::Health;
use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;

pub const TANK_SPEED: f32 = 2.0 / 3.0;
pub const TANK_SIZE: f32 = 20.0;
const TURRET_SIZE: f32 = 16.0;

#[derive(Component)]
pub struct Tank;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Ai;

// Stores the tanks's speed
#[derive(Component)]
pub struct Velocity {
    pub value: Vec2,
}

// For whether an AI is active or not
#[derive(Component)]
pub struct Active {
    pub value: bool,
}

// Time since last shot fired
#[derive(Component)]
pub struct AttackTimer {
    pub value: f32,
}

// Time since last got damage
#[derive(Component)]
pub struct HitTimer(pub f32);

// So that the flash yellow on damage system knows what colour to return to
#[derive(Component)]
pub struct OriginalColour(pub Color);

// How long an AI is continuing in a direction for
#[derive(Component)]
pub struct Steps {
    pub value: f32,
}

// The direction an AI is moving in
#[derive(Component)]
pub struct DirectionAi {
    pub value: u8,
}

#[derive(Component)]
pub struct Turret;

// The bearing entity is a child of a tank and the parent to the turret
#[derive(Component)]
pub struct Bearing;

// Defines a Tank Bundle that can spawn a tank in a single commands.spawn(TankBundle{ ... })
#[derive(Bundle)]
pub struct TankBundle<M: bevy::sprite::Material2d> {
    tank: Tank, // Marker component
    material_bundle: bevy::sprite::MaterialMesh2dBundle<M>, // Colour
    attack_timer: AttackTimer,
    hit_timer: HitTimer,
    health: Health,
    velocity: Velocity,
    original_colour: OriginalColour,
}

// The AI Bundle is an extension to the Tank Bundle
#[derive(Bundle)]
pub struct AiBundle {
    ai: Ai,
    active: Active,
    steps: Steps,
    direction_ai: DirectionAi,
}

#[derive(Bundle)]
pub struct BearingBundle {
    // Sprite bundle is used to give the bearing a translation
    // This is not necessary in later Bevy versions as the SpatialBundle exists
    sprite_bundle: SpriteBundle,
    bearing: Bearing,
}

impl BearingBundle {
    pub fn new() -> BearingBundle {
        BearingBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE,
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(1.0, 1.0, 1.0),
                    translation: Vec3::new(0.0, 0.0, 0.),
                    ..default()
                },
                ..default()
            },
            bearing: Bearing,
        }
    }
}

#[derive(Bundle)]
pub struct TurretBundle {
    sprite_bundle: SpriteBundle,
    turret: Turret,
    original_color: OriginalColour,
}

impl TurretBundle {
    pub fn new() -> TurretBundle {
        TurretBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0., 0., 0.),
                    ..default()
                },
                transform: Transform {
                    scale: Vec3::new(TURRET_SIZE, TURRET_SIZE, 0.),
                    translation: Vec3::new(TANK_SIZE + 4.0, 0.0, -1.0), // The "TANK_SIZE+4.0" is reset every frame due to a system anyway
                    ..default()
                },
                ..default()
            },
            turret: Turret,
            original_color: OriginalColour(Color::BLACK),
        }
    }
}

// TANK BUNDLE
impl TankBundle<ColorMaterial> {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        health: u8,
    ) -> TankBundle<ColorMaterial> {
        TankBundle {
            material_bundle: bevy::sprite::MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle { radius: TANK_SIZE })),
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..default()
                },
                ..default()
            },
            tank: Tank,
            attack_timer: AttackTimer { value: 0.0 },
            hit_timer: HitTimer(1.0),
            health: Health { value: health },
            velocity: Velocity {
                value: Vec2::new(0.0, 0.0),
            },
            original_colour: OriginalColour(Color::BLACK),
        }
    }
}

// A default AI bundle
impl AiBundle {
    pub fn new() -> AiBundle {
        AiBundle {
            active: Active { value: true },
            steps: Steps { value: 0.0 },
            direction_ai: DirectionAi { value: 0 },
            ai: Ai,
        }
    }
}
