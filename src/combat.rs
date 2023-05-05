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

    const STAND_IN_WEAPON: Weapon = Weapon {
        damage: 5,
        pen: 1,
        rof: 1,
        range: 1
    };

    const BASE_HIT_CHANCE: f32 = 0.1;

    fn is_target_hit(target: &Creature) -> bool {
        let mut rng = rand::thread_rng();
        let r: f32 = rng.gen();

        let mobility_modifier = get_ratio_of_working_body_tags(&target.body, BodyPartTag::Stance);
        let hit_chance = BASE_HIT_CHANCE * (2.0 - mobility_modifier);
        return r < hit_chance;
    }

    fn resolve_damage_against_part<'a>(part: &'a mut BodyPart, weapon: &Weapon) -> &'a BodyPart {
        part.statuses.push(BodyPartStatus::Wound);
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

    #[test]
    fn resolve_damage_test() {
        use crate::creature::humanoid::humanoid::*;
        let mut subject = humanoid();
        resolve_attack_against_creature(&mut subject, &STAND_IN_WEAPON);
        recalculate_health(&mut subject);
        println!("{:#?}", subject);
        println!("{:#?}", subject.health_stats);
    }
}