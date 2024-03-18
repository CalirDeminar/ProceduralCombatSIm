pub mod creature_modifiers {
    use crate::creature::{body::body::BodyPartTag, creature::Creature, organs::organs::OrganFunction};

    impl Creature {
        fn blood_state_modifier(self: &Self) -> f32 {
            self.health_stats.blood_oxy_ptc.min(self.health_stats.blood_vol_ptc)
        }
        pub fn to_hit_modifier(self: &Self) -> f32 {
            let skill_mod = self.mind.combat_skill;
            let injury_mod = self.blood_state_modifier() 
                * self.ratio_of_working_tagged_parts(BodyPartTag::Stance) 
                * self.ratio_of_working_tagged_organs(OrganFunction::Sight);
            skill_mod * injury_mod
        }
        pub fn to_be_hit_modifier(self: &Self) -> f32 {
            let skill_mod = self.mind.dodging_skill;
            let injury_mod = self.blood_state_modifier() 
                * self.ratio_of_working_tagged_parts(BodyPartTag::Stance) 
                * self.ratio_of_working_tagged_organs(OrganFunction::Sight);
            skill_mod * injury_mod
        }
    }
}