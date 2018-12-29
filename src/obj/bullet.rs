use super::{Object, health::Health, weapon::Weapon};

#[derive(Debug, Clone)]
pub struct Bullet<'a> {
    pub obj: Object,
    pub weapon: &'a Weapon,
}

impl Bullet<'_> {
    pub fn apply_damage(&self, health: &mut Health) {
        health.weapon_damage(self.weapon.damage, self.weapon.penetration)
    }
}