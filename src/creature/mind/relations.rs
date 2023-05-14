pub mod relations {
    use crate::creature::mind::mind::*;
    use crate::creature::mind::names::names::*;
    use rand::Rng;
    use rand_distr::{Normal, Distribution};
    use uuid::Uuid;

    const PARTNER_CHANCE: f32 = 0.5;
    const HOMOSEXUALITY_CHANCE: f32 = 0.2;

    const PARENT_PRESENCE_CHANCE: f32 = 0.3;

    fn gen_mind_with_gender_and_relation(name_dict: &NameDictionary, gender: &Gender, age: u32, relations: Vec<Relation>) -> Mind{
        let (first_name, last_name) = random_name(&name_dict, &gender);
        return Mind {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            gender: gender.clone(),
            age,
            relations
        };
    }

    fn gen_partner_gender(input_gender: &Gender) -> Gender {
        let mut rng = rand::thread_rng();
        let partner_type_roll = rng.gen::<f32>();
        let parnet_gender;
        if partner_type_roll > HOMOSEXUALITY_CHANCE {
            parnet_gender = if input_gender.eq(&Gender::Male) {Gender::Female} else {Gender::Male};
        } else {
            parnet_gender = input_gender.clone();
        }
        return parnet_gender;
    }

    pub fn add_partners_to_population(population: Vec<Mind>, name_dict: &NameDictionary) -> Vec<Mind> {
        let mut rng = rand::thread_rng();
        let mut output: Vec<Mind> = Vec::new();
        for mind in population {
            let has_partner = rng.gen::<f32>() > PARTNER_CHANCE;
            if !has_partner {
                output.push(mind.clone());
                continue;
            } else {
                let partner_gender = gen_partner_gender(&mind.gender);
                let mut target = mind.clone();
                let distribution = Normal::new(mind.age.clone() as f32, 4.0).unwrap();
                let relation = gen_mind_with_gender_and_relation(
                    &name_dict, 
                    &partner_gender, 
                    distribution.sample(&mut rand::thread_rng()) as u32, 
                    vec![(RelationVerb::Partner, target.id.clone())]
                );
                target.relations.push((RelationVerb::Partner, relation.id.clone()));
                assert!(relation.relations.iter().any(|(_,m)| m==&target.id));
                output.push(target);
                output.push(relation);
            }
        }
        return output;
    }

    pub fn add_parents_to_population(population: Vec<Mind>, name_dict: &NameDictionary) -> Vec<Mind> {
        let mut rng = rand::thread_rng();
        let mut output: Vec<Mind> = Vec::new();

        for mind in population {
            let parents_present = rng.gen::<f32>() > PARENT_PRESENCE_CHANCE;
            if !parents_present {
                output.push(mind);
            } else {
                let mut mind_m = mind.clone();
                let parent_age_distribution = Normal::new(mind.age.clone() as f32 + 30.0, 5.0).unwrap();
                let mut parent_one = gen_mind_with_gender_and_relation(
                    &name_dict,
                    &Gender::Female, 
                    parent_age_distribution.sample(&mut rand::thread_rng()) as u32, 
                    vec![(RelationVerb::Child, mind.id.clone())]
                );
                parent_one.last_name = String::from(&mind_m.last_name);
                let mut parent_two = gen_mind_with_gender_and_relation(
                    &name_dict,
                    &Gender::Male, 
                    parent_age_distribution.sample(&mut rand::thread_rng()) as u32, 
                    vec![(RelationVerb::Child, mind.id.clone())]
                );
                parent_two.last_name = String::from(&parent_one.last_name);
                let parent_one_alive = parent_one.age < Normal::new(65.0, 10.0).unwrap().sample(&mut rand::thread_rng()) as u32;
                if parent_one_alive {
                    mind_m.relations.push((RelationVerb::Parent, parent_one.id.clone()));
                    parent_two.relations.push((RelationVerb::Partner, parent_one.id.clone()));
                }
                let parent_two_alive = parent_two.age < Normal::new(65.0, 10.0).unwrap().sample(&mut rand::thread_rng()) as u32;
                if parent_two_alive {
                    mind_m.relations.push((RelationVerb::Parent, parent_two.id.clone()));
                    parent_one.relations.push((RelationVerb::Partner, parent_two.id.clone()));
                }
                if parent_one_alive {
                    output.push(parent_one);
                }
                if parent_two_alive {
                    output.push(parent_two);
                }
                output.push(mind_m);
            }
        }
        return output;
    }
}