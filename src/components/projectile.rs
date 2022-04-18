use bevy::prelude::{Component, Entity};

use rose_data::{EffectBulletMoveType, EffectId, SkillId};

#[derive(Component)]
pub struct Projectile {
    pub source: Entity,
    pub effect_id: Option<EffectId>,
    pub skill_id: Option<SkillId>,
    pub move_type: EffectBulletMoveType,
    pub apply_damage: bool,
    pub parabola_velocity: Option<f32>,
}

impl Projectile {
    pub fn new(
        source: Entity,
        effect_id: Option<EffectId>,
        skill_id: Option<SkillId>,
        move_type: EffectBulletMoveType,
        apply_damage: bool,
    ) -> Self {
        Self {
            source,
            effect_id,
            skill_id,
            move_type,
            apply_damage,
            parabola_velocity: None,
        }
    }
}
