pub mod organs;
pub mod humanoid;
pub mod body;
pub mod creature {
    use crate::creature::organs::organs::*;
    use crate::creature::humanoid::*;
    use crate::creature::body::body::*;

    #[derive(Debug, Clone)]
    pub struct HealthStats {
        alive: bool,
        // Ptcs 0<->1
        bloodVolPtc: f64,
        bloodOxyPtc: f64
    }

    pub fn baseHealthStats() -> HealthStats {
        return HealthStats {
            alive: true,
            bloodVolPtc: 1.0,
            bloodOxyPtc: 1.0
        }
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