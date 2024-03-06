pub mod creature_combat {
    use crate::creature::{body::body::{BodyPart, BodyPartTag}, creature::Creature, humanoid::humanoid::humanoid};

    // return - Option(String(noun), String(Verb))
    pub fn get_attack_name_for_part(part: &BodyPart) -> Option<(String, String)> {
        // kicks
        if part.tags.contains(&BodyPartTag::Stance) {
            return Some((String::from("kick"), String::from("kicks")));
        }
        // punches
        if part.tags.contains(&BodyPartTag::Grasp) {
            return Some((String::from("punch"), String::from("punches")));
        }
        None
    }

    pub fn get_attacks_for_part(part: &BodyPart) -> Vec<(&BodyPart, (String, String))> {
        let mut base: Vec<(&BodyPart, (String, String))> = Vec::new();
        let attack_for_current = get_attack_name_for_part(&part);
        if attack_for_current.is_some() {
            base.push((part, attack_for_current.unwrap().clone()));
        }
        part.children.iter().fold(base, |acc, p| vec![acc, get_attacks_for_part(p)].concat())
    }

    #[test]
    fn test(){
        let subject = humanoid();
        // println!("{:?}", subject.body.print());
        println!("{:#?}", get_attacks_for_part(&subject.body));
    }
}