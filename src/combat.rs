pub mod creature_weapons;
pub mod creature_modifiers;
pub mod targeting;
pub mod combat {
    use crate::creature::body::body::get_ratio_of_working_body_tags;
    use crate::creature::creature::*;
    use crate::creature::body::body::*;
    use crate::creature::organs::organs::OrganFunction;
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
        damage: 500,
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

    fn is_target_hit(attacker: &Creature, target: &Creature) -> bool {
        let mut rng = rand::thread_rng();
        let r: f32 = rng.gen();

        let attacker_hit_modifier = attacker.to_hit_modifier();
        let hit_chance = BASE_HIT_CHANCE * target.to_be_hit_modifier() * attacker_hit_modifier;
        // println!("To Hit Chance: {} - attacker_mult: {} target_mult: {}", hit_chance, attacker_hit_modifier, target.to_be_hit_modifier());
        r < hit_chance
    }

    fn damage_chance(damage: f32, size: f32, sqrt_multiplier: f32) -> f32 {
        1.0-((size.sqrt() *sqrt_multiplier)/damage).min(1.0).max(0.0)
    }

    fn resolve_damage_against_part<'a>(part: &'a mut BodyPart, weapon: &Weapon) -> &'a BodyPart {
        let can_pen = weapon.pen > part.size as u32;
        let destruction_chance = damage_chance(weapon.damage as f32, part.size as f32, 50.0);
        let wound_chance = damage_chance(weapon.damage as f32, part.size as f32, 15.0);
        let cut_chance = if can_pen {1.0} else {0.5} * (1.0 - (wound_chance + destruction_chance)).max(0.0).min(1.0);

        // println!("Part Hit: {}, Size: {}, Damage: {}, Pen: {}", part.name, part.size, weapon.damage, weapon.pen);
        // println!("Destruction Chance: {} Wound Chance: {} Cut Chance: {}", destruction_chance, wound_chance, cut_chance);

        let mut rng = rand::thread_rng();
        let r: f32 = rng.gen();

        if r < destruction_chance {
            if can_pen {
                part.statuses.push(StatusTag::Missing);
            } else {
                part.statuses.push(StatusTag::Destroyed);
            }
        }else if r + destruction_chance < wound_chance {
            part.statuses.push(StatusTag::Wound);
            let new_damage = (weapon.damage as i32 - part.size as i32).max(0) as u32;
            let new_pen = (weapon.pen as i32 - part.size as i32).max(0) as u32;
            if new_damage > 0 {
                let reduced_weapon = Weapon {
                        damage: new_damage,
                        pen: new_pen,
                        rof: weapon.rof,
                        range: weapon.range
                    };

                    if let Some(p) = random_weighted_internal(part) {
                        resolve_damage_against_part(p, &reduced_weapon);
                    }
            }
 
        } else if r + destruction_chance + wound_chance > cut_chance {
            part.statuses.push(StatusTag::Cut);
        }
        return part;
    }

    pub fn resolve_attack_against_creature<'a>(attacker: &Creature, target: &'a mut Creature, weapon: &Weapon) -> &'a Creature{
        if !is_target_hit(&attacker, &target) {
            return target;
        }
        let ref_part = random_weighted_part(&mut target.body);
        resolve_damage_against_part(ref_part, &weapon);
        return target;
    }

    #[test]
    fn resolve_damage_test() {
        use crate::creature::humanoid::humanoid::*;
        let mut subject_1 = humanoid();
        let mut subject_2 = humanoid();
        
        for i in 0..=50 {
            if !subject_1.health_stats.alive || !subject_2.health_stats.alive {
                continue;
            }
            resolve_attack_against_creature(&subject_1, &mut subject_2, &STAND_IN_WEAPON);
            resolve_attack_against_creature(&subject_2, &mut subject_1, &STAND_IN_WEAPON);
            recalculate_health(&mut subject_1);
            recalculate_health(&mut subject_2);
            println!("R{:?}----", i);
            println!("1: {:#?}", subject_1.health_stats);
            println!("2: {:#?}", subject_2.health_stats);
            
        }

        print_creature(&subject_1);
        println!("{:#?}", subject_1.health_stats);
        print_creature(&subject_2);
        println!("{:#?}", subject_2.health_stats);
    }
}