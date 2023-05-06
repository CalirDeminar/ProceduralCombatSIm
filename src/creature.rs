pub mod organs;
pub mod humanoid;
pub mod body;
pub mod creature {
    use crate::creature::body::body::*;

    const BREATH_LOSS_RATE: f32 = 0.1;

    #[derive(Debug, Clone)]
    pub struct HealthStats {
        pub alive: bool,
        // Ptcs 0<->1
        pub blood_vol_ptc: f32,
        pub blood_oxy_ptc: f32
    }

    pub fn base_health_stats() -> HealthStats {
        return HealthStats {
            alive: true,
            blood_vol_ptc: 1.0,
            blood_oxy_ptc: 1.0
        }
    }

    fn propegate_statuses<'a>(part: &'a mut BodyPart) {
        for p in &mut part.children {
            if part.statuses.contains(&BodyPartStatus::Destroyed) {
                p.statuses.push(BodyPartStatus::Destroyed);
            }
            if part.statuses.contains(&BodyPartStatus::Missing) {
                p.statuses.push(BodyPartStatus::Missing);
            }
            if part.statuses.contains(&BodyPartStatus::Paralised) {
                p.statuses.push(BodyPartStatus::Paralised)
            }

            propegate_statuses(p);
        }
        for p in &mut part.internal {
            if part.statuses.contains(&BodyPartStatus::Destroyed) {
                p.statuses.push(BodyPartStatus::Destroyed);
            }
            if part.statuses.contains(&BodyPartStatus::Missing) {
                p.statuses.push(BodyPartStatus::Missing);
            }
            if part.statuses.contains(&BodyPartStatus::Paralised) {
                p.statuses.push(BodyPartStatus::Paralised)
            }

            propegate_statuses(p);
        }
    }

    fn calc_body_part_bleed_amount(part: &BodyPart) -> f32 {
        let base_volume = (sum_child_part_size_r(part) + sum_internal_part_size_r(part)) as f32;
        let destroyed_vol = sum_status_size(&part, BodyPartStatus::Destroyed) as f32;
        let missing_vol = sum_status_size(&part, BodyPartStatus::Missing) as f32;
        let wound_vol = sum_status_size(&part, BodyPartStatus::Wound) as f32 / 2.0;
        let cut_vol = sum_status_size(&part, BodyPartStatus::Cut) as f32 / 4.0;
        
        return (destroyed_vol + missing_vol + wound_vol + cut_vol) / base_volume;
    }

    pub fn recalculate_health<'a>(subject: &'a mut Creature) -> &'a Creature {
        propegate_statuses(&mut subject.body);
        let working_breath_ratio = get_ratio_of_working_body_tags(&subject.body, BodyPartTag::Breath);
        let working_circulation_ratio = get_ratio_of_working_body_tags(&subject.body, BodyPartTag::Circulation);

        // multipler shouldn't be above one, and hearth failure is more severe than breath trouble / failure
        let breath_loss_factor = (1.0 - working_breath_ratio)
            .max(1.0 - (working_circulation_ratio * 2.0))
            .min(1.0)
            .max(0.0) * BREATH_LOSS_RATE;

        subject.health_stats.blood_oxy_ptc -=  breath_loss_factor.max(0.0);

        let total_blood_loss = calc_body_part_bleed_amount(&subject.body);
        
        subject.health_stats.blood_vol_ptc -= total_blood_loss;

        let has_brain =count_tagged_parts(&subject.body, BodyPartTag::Thought) > 0;

        if subject.health_stats.blood_vol_ptc <= 0.0 || 
            subject.health_stats.blood_oxy_ptc <= 0.0 ||
            !has_brain {
            subject.health_stats.alive = false;
        }
        

        return subject;
    }

    fn print_body_part(part: &BodyPart, prefix: &str) {
        let mut line = String::from(prefix);
        line.push_str(&format!("{} ", part.name));
        line.push_str(&format!("- Tags: {:?} ", part.tags));
        line.push_str(&format!("- Statuses: {:?}", part.statuses));
        println!("{}", line);
        for i in &part.internal {
            print_body_part(&i, &format!("{}    ", prefix));
        }
        for i in &part.children {
            print_body_part(&i, &format!("{}    ", prefix));
        }
    }

    pub fn print_creature(creature: &Creature) {
        println!("Species: {:?}", creature.species);
        print_body_part(&creature.body, "");
    }

    #[derive(Debug, Clone)]
    pub struct Creature {
        pub species: String,
        pub health_stats: HealthStats,
        pub body: BodyPart
    }

}

#[cfg(test)]
mod tests {
    use crate::creature::creature::*;
    use crate::creature::body::body::*;
    use crate::creature::humanoid::humanoid::*;

    #[test]
    fn recalculate_health_healthy() {
        let mut subject = humanoid();
        recalculate_health(&mut subject);
        assert_eq!(subject.health_stats.blood_vol_ptc, 1.0);
        assert_eq!(subject.health_stats.blood_oxy_ptc, 1.0);
    }

    #[test]
    fn recalculate_health_oxy() {
        let mut subject = humanoid();
        subject.body.internal
            .iter_mut()
            .find(|p| p.tags.contains(&BodyPartTag::Breath))
            .unwrap()
            .statuses.push(BodyPartStatus::Paralised);

        recalculate_health(&mut subject);
        assert_eq!(subject.health_stats.blood_vol_ptc, 1.0);
        assert_eq!(subject.health_stats.blood_oxy_ptc < 1.0, true);
    }

    #[test]
    fn recalculate_health_blood_vol_limb() {
        let mut subject = humanoid();
        subject.body.children
            .iter_mut()
            .find(|p| p.name.eq("Left Leg"))
            .unwrap()
            .statuses.push(BodyPartStatus::Missing);

        recalculate_health(&mut subject);
        assert_eq!(subject.health_stats.blood_oxy_ptc, 1.0);
        assert_eq!(subject.health_stats.blood_vol_ptc < 1.0, true);
    }

    #[test]
    fn recalculate_health_blood_vol_body() {
        let mut subject = humanoid();
        subject.body.statuses.push(BodyPartStatus::Wound);

        recalculate_health(&mut subject);
        assert_eq!(subject.health_stats.blood_oxy_ptc, 1.0);
        assert_eq!(subject.health_stats.blood_vol_ptc < 1.0, true);
    }

    #[test]
    fn status_propegation_test() {
        let mut subject = humanoid();
        subject.body.statuses.push(BodyPartStatus::Destroyed);

        recalculate_health(&mut subject);
        assert_eq!(count_tagged_parts_with_status(&subject.body, BodyPartTag::Breath, BodyPartStatus::Destroyed), 2);
    }
}