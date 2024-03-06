pub mod body {
    use rand::Rng;

    use crate::creature::{creature::{print_body_part, print_creature}, humanoid::humanoid::humanoid, organs::organs::{Organ, OrganFunction}};

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum StatusTag {
        Bruise,
        Cut,
        Wound,
        Paralised,
        Broken,
        Destroyed,
        Missing
    }

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum LocationTag {
        Left,
        Right,
        Front,
        Back,
        Upper,
        Lower,
    }

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum ActionTag {
        StanceFly,
        StanceWalk,
        StanceSwim,
        Grasp,
        Weapon
    }

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum BodyPartTag {
        // Action
        Fly,
        Grasp,
        Stance,
        // Location
        Left,
        Right,
        Joint
    }
    #[derive(PartialEq, Debug, Clone)]
    pub struct BodyPart {
        pub name: String,
        pub tags: Vec<BodyPartTag>,
        pub statuses: Vec<StatusTag>,
        pub organs: Vec<Organ>,
        pub children: Vec<BodyPart>,
        pub size: u32,
    }
    impl BodyPart {
        pub fn sum_child_part_size_r(self: &Self) -> u32 {
            if self.children.len() == 0 {
                return self.size;
            }
            self.size + self.children.iter().fold(0, |acc, p| acc + p.sum_child_part_size_r())
        }
        pub fn sum_internal_organ_part_size_r(self: &Self) -> u32 {
            if self.organs.len() == 0 {
                return 0;
            }
            self.organs.iter().fold(0, |acc, o| acc + o.size)
        }
        pub fn count_organs_with_function(self: &Self, function: OrganFunction) -> usize {
            let own_organ_count = self.organs.iter().filter(|o| o.functions.contains(&function)).count();
            self.children.iter().fold(own_organ_count, |acc, c| acc + c.count_organs_with_function(function))
        }
        pub fn count_organs_with_function_and_condition(self: &Self, function: OrganFunction, condition: Option<StatusTag>) -> usize {
            let local_count = self.organs.iter().filter(|o| o.functions.contains(&function) && (condition.is_none() || o.conditions.contains(&condition.unwrap()))).count();
            self.children.iter().fold(local_count, |acc, c| acc + c.count_organs_with_function_and_condition(function, condition))
        }
        pub fn sum_organ_size_with_function_and_condition(self: &Self, function: OrganFunction, condition: Option<StatusTag>) -> u32 {
            let organ_count = self.organs.iter().fold(0, |acc, o| acc + if o.functions.contains(&function) && (condition.is_none() || o.conditions.contains(&condition.unwrap())) { o.size} else {0});
            self.children.iter().fold(organ_count, |acc, c| acc + c.sum_organ_size_with_function_and_condition(function, condition))
        }
        pub fn sum_part_size_with_function_and_condition(self: &Self, function: BodyPartTag, condition: Option<StatusTag>) -> u32 {
            let own_size = if self.tags.contains(&function) && (condition.is_none() || self.statuses.contains(&condition.unwrap())) { self.size} else {0};
            self.children.iter().fold(own_size, |acc, c| acc + c.sum_part_size_with_function_and_condition(function, condition))
        }
        pub fn sum_status_size(self: &Self, status: StatusTag) -> u32 {
            let self_size = if self.statuses.contains(&status) { self.size } else { 0};
            let self_organ_size = self.organs.iter().fold(0, |acc, o| acc + if o.conditions.contains(&status) { o.size} else {0});
            self.children.iter().fold(self_size + self_organ_size, |acc, c| acc + c.sum_status_size(status))
        }
        pub fn print(self: &Self) {
            print_body_part(self, "");
        }
    }
    pub fn get_ratio_of_working_organ_tags(body: &BodyPart, function: OrganFunction) -> f32 {
        let total_size = body.sum_organ_size_with_function_and_condition(function, None);
        let destroyed_size = body.sum_organ_size_with_function_and_condition(function, Some(StatusTag::Destroyed));
        let missing_size = body.sum_organ_size_with_function_and_condition(function, Some(StatusTag::Missing));
        let paralised_size = body.sum_organ_size_with_function_and_condition(function, Some(StatusTag::Paralised));
        let broken_size = body.sum_organ_size_with_function_and_condition(function, Some(StatusTag::Broken));
        let working_size = total_size - (destroyed_size + missing_size + paralised_size + broken_size);
        return (working_size as f32) / (total_size as f32);
    }
    pub fn get_ratio_of_working_body_tags(body: &BodyPart, function: BodyPartTag) -> f32 {
        let total_size = body.sum_part_size_with_function_and_condition(function, None);
        let destroyed_size = body.sum_part_size_with_function_and_condition(function, Some(StatusTag::Destroyed));
        let missing_size = body.sum_part_size_with_function_and_condition(function, Some(StatusTag::Missing));
        let paralised_size = body.sum_part_size_with_function_and_condition(function, Some(StatusTag::Paralised));
        let broken_size = body.sum_part_size_with_function_and_condition(function, Some(StatusTag::Broken));
        let working_size = total_size - (destroyed_size + missing_size + paralised_size + broken_size);
        return (working_size as f32) / (total_size as f32);
    }
    fn random_part_is_selected<'a>(body: &'a mut BodyPart, count: u32, roll: u32) -> (u32, Option<&'a mut BodyPart>) {
        let c = count + body.size;
        if c > roll {
            return (c, Some(body));
        }
        return body.children.iter_mut()
            .fold((c, None), |(acc_c, acc_r), p| if acc_r.is_none() { random_part_is_selected(p, acc_c, roll) } else {(c, acc_r)});
    }
    pub fn random_weighted_part<'a>(body: &'a mut BodyPart) -> &'a mut BodyPart {
        if body.children.len() == 0 {
            return body;
        }
        let total_size = body.sum_child_part_size_r();

        let mut rng = rand::thread_rng();
        let r:f32 = rng.gen();
        let roll = (r * total_size as f32) as u32;

        let (_, rtn) = random_part_is_selected(body, 0, roll);

        return rtn.unwrap();
    }
    pub fn random_weighted_internal<'a>(body: &'a mut BodyPart) -> Option<&'a mut BodyPart> {
        if body.organs.len() == 0 {
            return None;
        }
        let internal_size = body.sum_internal_organ_part_size_r();

        let mut rng = rand::thread_rng();
        let r:f32 = rng.gen();
        let roll = (r * internal_size as f32) as u32;

        let (_, rtn) = random_part_is_selected(body, 0, roll);

        return rtn;    
    }


    #[test]
    fn test_sum_child_part_size() {
        use crate::creature::humanoid::humanoid::*;
        let subject = humanoid();
        // println!("{:#?}", subject);
        assert_eq!(subject.body.count_organs_with_function(OrganFunction::Breath), 2);
    }

    #[test]
    fn test_random_weighted_part() {
        let mut subject = humanoid();
        for _i in 0..=20 {
            random_weighted_part(&mut subject.body);
        }
    }
}