pub mod organs;
pub mod humanoid;
pub mod body;
pub mod creature {
    use crate::creature::organs::organs::*;
    use crate::creature::humanoid::*;
    use crate::creature::body::body::*;

    #[derive(Debug, Clone)]
    pub struct HealthStats {
        pub alive: bool,
        // Ptcs 0<->1
        pub bloodVolPtc: f64,
        pub bloodOxyPtc: f64
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
                    p.statuses.contains(&BodyPartStatus::paralised)
                )
            )
            .count();
    }

    pub fn recalculate_health(subject: Creature) -> Creature {
        
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