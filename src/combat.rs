pub mod combat {
    use crate::creature::body::body::get_ratio_of_working_body_tags;
    use crate::creature::creature::*;
    use crate::creature::body::body::*;
    use rand::Rng;
    pub struct Weapon {
        pub damage: u32,
        pub pen: u32,
        pub rof: u32,
        pub range: u32
    }

    // Add in some form of weapon damage inheritance from creature strength
    //  (If a melee weapon)
    const STAND_IN_WEAPON: Weapon = Weapon {
        damage: 5,
        pen: 1,
        rof: 1,
        range: 1
    };

    // Damage Rules
    // For statuses to be possible
    // Destroyed - Damage > 2x Body Part Size - Paralises all children
    // Missing / Cut off - Damage > Body Part Size && Pen > Body Part Size - All children missing
    // Wound - Damage > Body Part Size (Can also hit internals for remaining damage)
    // Cut - Any

    const BASE_HIT_CHANCE: f32 = 0.1;

    fn is_target_hit(target: &Creature) -> bool {
        let mut rng = rand::thread_rng();
        let r: f32 = rng.gen();

        let target_mobility_modifier = get_ratio_of_working_body_tags(&target.body, BodyPartTag::Stance);
        let hit_chance = BASE_HIT_CHANCE * (2.0 - target_mobility_modifier);
        return r < hit_chance;
    }

    fn resolve_damage_against_part<'a>(part: &'a mut BodyPart, weapon: &Weapon) -> &'a BodyPart {
        let can_remove = weapon.pen > part.size as u32;
        let destruction_chance = 1.0 - (weapon.damage as f32 / (part.size * 2) as f32);
        let wound_chance = (1.0 - (weapon.damage as f32 / part.size as f32)) * (1.0-destruction_chance);
        let cut_chance = 0.2 * (1.0 - (wound_chance + destruction_chance));

        let mut rng = rand::thread_rng();
        let r: f32 = rng.gen();
        if r > destruction_chance {
            if can_remove {
                part.statuses.push(BodyPartStatus::Missing);
            } else {
                part.statuses.push(BodyPartStatus::Destroyed);
            }
        }else if r > destruction_chance + wound_chance {
            part.statuses.push(BodyPartStatus::Wound);
        } else if r > destruction_chance + wound_chance + cut_chance {
            part.statuses.push(BodyPartStatus::Cut);
        }
        return part;
    }

    pub fn resolve_attack_against_creature<'a>(target: &'a mut Creature, weapon: &Weapon) -> &'a Creature{
        if !is_target_hit(&target) {
            return target;
        }
        let ref_part = random_weighted_part(&mut target.body);
        resolve_damage_against_part(ref_part, &STAND_IN_WEAPON);

        println!("{:#?}", target);
        return target;
    }

    // #[test]
    // fn resolve_damage_test() {
    //     use crate::creature::humanoid::humanoid::*;
    //     let mut subject = humanoid();
    //     resolve_damage_against_part(&mut subject.body, &STAND_IN_WEAPON);
    //     recalculate_health(&mut subject);
    //     println!("{:#?}", subject.health_stats);
    //     assert_eq!(subject.health_stats.blood_vol_ptc < 1.0, true);
    // }
}