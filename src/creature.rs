pub mod organs;
pub mod humanoid;
pub mod body;
pub mod creature {
    use crate::creature::body::body::*;

    use super::organs::organs::{Organ, OrganFunction};

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
            if part.statuses.contains(&StatusTag::Destroyed) && !p.statuses.contains(&StatusTag::Destroyed) {
                p.statuses.push(StatusTag::Destroyed);
            }
            if part.statuses.contains(&StatusTag::Missing) && !p.statuses.contains(&StatusTag::Missing) {
                p.statuses.push(StatusTag::Missing);
            }
            if part.statuses.contains(&StatusTag::Paralised) && !p.statuses.contains(&StatusTag::Paralised) {
                p.statuses.push(StatusTag::Paralised)
            }

            propegate_statuses(p);
        }
        for p in &mut part.organs {
            if part.statuses.contains(&StatusTag::Destroyed) && !p.conditions.contains(&StatusTag::Destroyed) {
                p.conditions.push(StatusTag::Destroyed);
            }
            if part.statuses.contains(&StatusTag::Missing) && !p.conditions.contains(&StatusTag::Missing) {
                p.conditions.push(StatusTag::Missing);
            }
            if part.statuses.contains(&StatusTag::Paralised) && !p.conditions.contains(&StatusTag::Paralised) {
                p.conditions.push(StatusTag::Paralised)
            }
        }
    }

    fn calc_body_part_bleed_amount(part: &BodyPart) -> f32 {
        let base_volume = (part.sum_child_part_size_r() + part.sum_internal_organ_part_size_r()) as f32;
        let destroyed_vol = part.sum_status_size(StatusTag::Destroyed) as f32;
        let missing_vol = part.sum_status_size(StatusTag::Missing) as f32;
        let wound_vol = part.sum_status_size(StatusTag::Wound) as f32 / 2.0;
        let cut_vol = part.sum_status_size(StatusTag::Cut) as f32 / 4.0;
        
        return (destroyed_vol + missing_vol + wound_vol + cut_vol) / base_volume;
    }

    pub fn recalculate_health<'a>(subject: &'a mut Creature) -> &'a Creature {
        propegate_statuses(&mut subject.body);
        let working_breath_ratio = get_ratio_of_working_organ_tags(&subject.body, OrganFunction::Breath);
        let working_circulation_ratio = get_ratio_of_working_organ_tags(&subject.body, OrganFunction::Circulation);

        // multipler shouldn't be above one, and hearth failure is more severe than breath trouble / failure
        let breath_loss_factor = (1.0 - working_breath_ratio)
            .max(1.0 - (working_circulation_ratio * 2.0))
            .min(1.0)
            .max(0.0) * BREATH_LOSS_RATE;

        subject.health_stats.blood_oxy_ptc = (subject.health_stats.blood_oxy_ptc - breath_loss_factor.max(0.0)).min(1.0).max(0.0);

        let total_blood_loss = calc_body_part_bleed_amount(&subject.body);
        
        subject.health_stats.blood_vol_ptc = (subject.health_stats.blood_vol_ptc - total_blood_loss).min(1.0).max(0.0);

        let has_brain = &subject.body.count_organs_with_function(OrganFunction::Thought) > &0;

        if subject.health_stats.blood_vol_ptc <= 0.0 || 
            subject.health_stats.blood_oxy_ptc <= 0.0 ||
            !has_brain {
            subject.health_stats.alive = false;
        }
        

        return subject;
    }

    pub fn print_body_part(part: &BodyPart, prefix: &str) {
        let mut line = String::from(prefix);
        line.push_str(&format!("{} ", part.name));
        line.push_str(&format!("- Tags: {:?} ", part.tags));
        line.push_str(&format!("- Statuses: {:?}", part.statuses));
        println!("{}", line);
        for i in &part.organs {
            print_organ(&i, &format!("{}    ", prefix));
        }
        for i in &part.children {
            print_body_part(&i, &format!("{}    ", prefix));
        }
    }

    fn print_organ(part: &Organ, prefix: &str) {
        let mut line = String::from(prefix);
        line.push_str(&format!("{} ", part.name));
        line.push_str(&format!("- Functions: {:?} ", part.functions));
        line.push_str(&format!("- Conditions: {:?}", part.conditions));
        println!("{}", line);
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
    use crate::creature::organs::organs::OrganFunction;

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
        subject.body.organs
            .iter_mut()
            .find(|p| p.functions.contains(&OrganFunction::Breath))
            .unwrap()
            .conditions.push(StatusTag::Paralised);

        recalculate_health(&mut subject);
        assert_eq!(subject.health_stats.blood_vol_ptc, 1.0);
        assert_eq!(subject.health_stats.blood_oxy_ptc < 1.0, true);
    }

    #[test]
    fn recalculate_health_blood_vol_limb() {
        let mut subject = humanoid();
        subject.body.children
            .iter_mut()
            .find(|p| p.name.eq("Left Upper Leg"))
            .unwrap()
            .statuses.push(StatusTag::Missing);

        recalculate_health(&mut subject);
        
        assert_eq!(subject.health_stats.blood_oxy_ptc, 1.0);
        assert_eq!(subject.health_stats.blood_vol_ptc < 1.0, true);
    }

    #[test]
    fn recalculate_health_blood_vol_body() {
        let mut subject = humanoid();
        subject.body.statuses.push(StatusTag::Wound);

        recalculate_health(&mut subject);
        assert_eq!(subject.health_stats.blood_oxy_ptc, 1.0);
        assert_eq!(subject.health_stats.blood_vol_ptc < 1.0, true);
    }

    #[test]
    fn status_propegation_test() {
        let mut subject = humanoid();
        subject.body.statuses.push(StatusTag::Destroyed);
        subject.body.print();
        recalculate_health(&mut subject);
        assert_eq!(subject.body.count_organs_with_function_and_condition(OrganFunction::Breath, Some(StatusTag::Destroyed)), 2);
    }
}