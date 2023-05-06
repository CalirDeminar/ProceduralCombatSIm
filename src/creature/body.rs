pub mod body {
    use rand::Rng;

    use crate::creature::humanoid::humanoid::humanoid;

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum BodyPartStatus {
        Bruise,
        Cut,
        Wound,
        Paralised,
        Broken,
        Destroyed,
        Missing
    }

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum BodyPartTag {
        // Core
        Breath,
        Thought,
        Nervous,
        Circulation,
        // Sense
        Sight,
        Hearing,
        Smell,
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
        pub statuses: Vec<BodyPartStatus>,
        pub internal: Vec<BodyPart>,
        pub children: Vec<BodyPart>,
        pub size: u32,
    }

    pub fn sum_child_part_size_r(part: &BodyPart) -> u32 {
        if part.children.len() == 0 {
            return part.size;
        }
        return part.size + part.children.iter().fold(0, |acc, p| acc + sum_child_part_size_r(p));
    }
    pub fn sum_internal_part_size_r(part: &BodyPart) -> u32 {
        if part.internal.len() == 0 {
            return part.size;
        }
        return part.size + part.internal.iter().fold(0, |acc, p| acc + sum_internal_part_size_r(p));
    }
    pub fn count_tagged_parts(body: &BodyPart, tag: BodyPartTag) -> usize {
        let local_count: usize = if body.tags.contains(&tag) { 1 } else {0};
        let child_count = body.children.iter()
            .fold(0, |acc, p| acc + count_tagged_parts(p, tag));
        let internal_count = body.internal.iter()
            .fold(0, |acc, p| acc + count_tagged_parts(p, tag));
        return local_count + child_count + internal_count;
    }
    fn sum_tagged_size(body: &BodyPart, tag: BodyPartTag) -> u32 {
        let local_count: u32 = if body.tags.contains(&tag) { body.size } else {0};
        let child_count = body.children.iter()
            .fold(0, |acc, p| acc + sum_tagged_size(p, tag));
        let internal_count = body.internal.iter()
            .fold(0, |acc, p| acc + sum_tagged_size(p, tag));
        return local_count + child_count + internal_count;
    }
    pub fn count_tagged_parts_with_status(body: &BodyPart, tag: BodyPartTag, status: BodyPartStatus) -> usize {
        let local_count: usize = if body.tags.contains(&tag) && body.statuses.contains(&status) { 1 } else {0};
        let child_count = body.children.iter()
            .fold(0, |acc, p| acc + count_tagged_parts_with_status(p, tag, status));
        let internal_count = body.internal.iter()
            .fold(0, |acc, p| acc + count_tagged_parts_with_status(p, tag, status));
        return local_count + child_count + internal_count;
    }
    pub fn sum_status_size(body: &BodyPart, status: BodyPartStatus) -> u32 {
        let local_count: u32 = if  body.statuses.contains(&status) { body.size } else {0}; 
        let child_count = body.children.iter()
            .fold(0, |acc, p| acc + sum_status_size(p, status));
        let internal_count = body.internal.iter()
            .fold(0, |acc, p| acc + sum_status_size(p, status));
        return local_count + child_count + internal_count;
    }
    fn sum_tagged_size_with_status(body: &BodyPart, tag: BodyPartTag, status: BodyPartStatus) -> u32 {
        let local_count: u32 = if body.tags.contains(&tag) && body.statuses.contains(&status) { body.size } else {0};
        let child_count = body.children.iter()
            .fold(0, |acc, p| acc + sum_tagged_size_with_status(p, tag, status));
        let internal_count = body.internal.iter()
            .fold(0, |acc, p| acc + sum_tagged_size_with_status(p, tag, status));
        return local_count + child_count + internal_count;
    }
    pub fn get_ratio_of_working_body_tags(body: &BodyPart, tag: BodyPartTag) -> f32 {
        let total_size = sum_tagged_size(body, tag);
        let destroyed_size = sum_tagged_size_with_status(body, tag, BodyPartStatus::Destroyed);
        let missing_size = sum_tagged_size_with_status(body, tag, BodyPartStatus::Missing);
        let paralised_size = sum_tagged_size_with_status(body, tag, BodyPartStatus::Paralised);
        let broken_size = sum_tagged_size_with_status(body, tag, BodyPartStatus::Broken);
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
        let total_size = sum_child_part_size_r(&body);

        let mut rng = rand::thread_rng();
        let r:f32 = rng.gen();
        let roll = (r * total_size as f32) as u32;

        let (_, rtn) = random_part_is_selected(body, 0, roll);

        return rtn.unwrap();
    }
    // pub fn random_weighted_part_with_internal(body: &BodyPart) -> &BodyPart {
    //     let p = random_weighted_part(body);
    //     if p.internal.len() == 0 {
    //         return p;
    //     }
        
    //     let internals: Vec<&BodyPart> = vec![body.internal.iter().collect(), vec![p]].concat();
    //     let total_size = sum_part_size(&internals);


    //     let mut rng = rand::thread_rng();
    //     let r:f32 = rng.gen();
    //     let roll = (r * total_size as f32) as u32;

    //     let mut t = 0;
    //     for part in internals {
    //         t += part.size;
    //         if t > roll {
    //             return part;
    //         }
    //     }

    //     return body;
    // }

    #[test]
    fn test_sum_child_part_size() {
        use crate::creature::humanoid::humanoid::*;
        let subject = humanoid();
        assert_eq!(count_tagged_parts(&subject.body, BodyPartTag::Breath), 2);
    }

    #[test]
    fn test_random_weighted_part() {
        let mut subject = humanoid();
        for _i in 0..=20 {
            random_weighted_part(&mut subject.body);
        }
    }
}