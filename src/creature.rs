pub mod organs;
pub mod humanoid;
pub mod body;
pub mod creature {
    use crate::creature::organs::organs::*;
    use crate::creature::humanoid::*;
    use crate::creature::body::body::*;

    const BREATH_LOSS_RATE: f32 = 0.1;

    #[derive(Debug, Clone)]
    pub struct HealthStats {
        pub alive: bool,
        // Ptcs 0<->1
        pub bloodVolPtc: f32,
        pub bloodOxyPtc: f32
    }

    pub fn base_health_stats() -> HealthStats {
        return HealthStats {
            alive: true,
            bloodVolPtc: 1.0,
            bloodOxyPtc: 1.0
        }
    }

    fn get_tagged_parts(body: &BodyPart, tag: BodyPartTag) -> Vec<&BodyPart> {
        let parts = flatten_all(&body);
        let mut output = vec![];
        for part in parts {
            if part.tags.contains(&tag) {
                output.push(part);
            }
        }
        return output;
    }

    fn count_functional_tagged_parts(body: &BodyPart, tag: BodyPartTag) -> usize {
        let parts = get_tagged_parts(&body, tag);
        return parts
            .iter()
            .filter(|p| 
                !(
                    p.statuses.contains(&BodyPartStatus::destroyed) || 
                    p.statuses.contains(&BodyPartStatus::paralised) ||
                    p.statuses.contains(&BodyPartStatus::missing)
                )
            )
            .count();
    }

    fn calc_body_part_bleed_amount(part: &BodyPart, creature_size: i32) -> f32 {
        let base_volume = (part.size as f32) / (creature_size as f32);
        if part.statuses.contains(&BodyPartStatus::destroyed) || 
            part.statuses.contains(&BodyPartStatus::missing) {
            return base_volume;
        }
        if part.statuses.contains(&BodyPartStatus::wound) {
            return base_volume / 2.0;
        }
        if part.statuses.contains(&BodyPartStatus::cut) {
            return base_volume / 4.0;
        }
        return 0.0;
    }

    fn calc_creature_size(body: &BodyPart) -> i32 {
        let parts = flatten_children(body);
        let mut total = 0;
        for part in parts {
            total += part.size;
        }
        return total;
    }

    fn get_ratio_of_working_body_tags(body: &BodyPart, tag: BodyPartTag) -> f32 {
        // TODO - rework this to work by part size ratios
        let total_count = get_tagged_parts(body, tag).len();
        let working_count = count_functional_tagged_parts(body, tag);
        return (working_count as f32) / (total_count as f32);
    }

    pub fn recalculate_health<'a>(subject: &'a mut Creature) -> &'a Creature {
        let creature_size = calc_creature_size(&subject.body);
        
        let working_breath_ratio = get_ratio_of_working_body_tags(&subject.body, BodyPartTag::breath);
        let working_circulation_ratio = get_ratio_of_working_body_tags(&subject.body, BodyPartTag::circulation);

        // multipler shouldn't be above one, and hearth failure is more severe than breath trouble / failure
        let breath_loss_factor = working_breath_ratio.max(working_circulation_ratio * 2.0).min(1.0) * BREATH_LOSS_RATE;
        subject.health_stats.bloodOxyPtc -=  breath_loss_factor.min(0.0);

        let mut total_blood_loss = 0.0;
        let parts = flatten_all(&subject.body);
        for part in parts {
            total_blood_loss += calc_body_part_bleed_amount(part, creature_size);
        }
        subject.health_stats.bloodVolPtc -= total_blood_loss;
        

        return subject;
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
    use crate::creature::{humanoid::humanoid::humanoid, body::body::random_weighted_part_with_internal};

    #[test]
    fn flatten() {
        let subject = humanoid();
        println!("{:#?}", subject);
        for _i in 0..100 {
            let part = random_weighted_part_with_internal(&subject.body);
            println!("{:?}", part.name);
        }
    }
}