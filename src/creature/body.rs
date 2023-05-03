pub mod body {
    use rand::Rng;

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum BodyPartStatus {
        bruise,
        cut,
        wound,
        paralised,
        broken,
        destroyed
    }

    #[derive(PartialEq, Debug, Clone, Copy)]
    pub enum BodyPartTag {
        // Core
        breath,
        thought,
        nervous,
        circulation,
        // Sense
        sight,
        hearing,
        smell,
        // Action
        fly,
        grasp,
        stance,
        // Location
        left,
        right,
        joint
    }
    #[derive(PartialEq, Debug, Clone)]
    pub struct BodyPart {
        pub name: String,
        pub tags: Vec<BodyPartTag>,
        pub statuses: Vec<BodyPartStatus>,
        pub internal: Vec<BodyPart>,
        pub children: Vec<BodyPart>,
        pub size: i32,
    }

    fn sum_part_size(parts: &Vec<&BodyPart>) -> i32 {
        let mut total = 0;
        for part in parts {
            total += part.size;
        }
        return total;
    }
    pub fn flatten_children(body: &BodyPart) -> Vec<&BodyPart> {
        if body.children.len() == 0 {
            return vec![&body];
        }
        let mut children: Vec<Vec<&BodyPart>> = vec![vec![body]];
        for child in &body.children {
            children.push(flatten_children(&child));
        }

        return children.concat();
    }
    pub fn flatten_internals(body: &BodyPart) -> Vec<&BodyPart> {
        let parts = flatten_children(&body);
        let mut output: Vec<Vec<&BodyPart>> = vec![vec![]];
        for part in parts {
            let internal = part.internal.iter().collect();
            output.push(internal);
        }
        return output.concat();
    }
    pub fn flatten_all(body: &BodyPart) -> Vec<&BodyPart> {
        return vec![flatten_children(&body), flatten_internals(&body)].concat();
    }
    pub fn random_weighted_part(body: &BodyPart) -> &BodyPart {
        let all_parts = flatten_children(&body);
        let total_size = sum_part_size(&all_parts);

        let mut rng = rand::thread_rng();
        let r:f32 = rng.gen();
        let roll = (r * total_size as f32) as i32;

        let mut t = 0;
        for part in all_parts {
            t += part.size;
            if t > roll {
                return part;
            }
        }

        return body;
    }
    pub fn random_weighted_part_with_internal(body: &BodyPart) -> &BodyPart {
        let p = random_weighted_part(body);
        if p.internal.len() == 0 {
            return p;
        }
        
        let internals: Vec<&BodyPart> = vec![body.internal.iter().collect(), vec![p]].concat();
        let total_size = sum_part_size(&internals);


        let mut rng = rand::thread_rng();
        let r:f32 = rng.gen();
        let roll = (r * total_size as f32) as i32;

        let mut t = 0;
        for part in internals {
            t += part.size;
            if t > roll {
                return part;
            }
        }

        return body;
    }

    #[test]
    fn flatten_children_humanoid() {
        use crate::creature::humanoid::humanoid::*;
        let subject = humanoid();
        let flattened = flatten_children(&subject.body);
        assert_eq!(flattened.len(), 6);
    }

    #[test]
    fn flatten_internals_humanoid() {
        use crate::creature::humanoid::humanoid::*;
        let subject = humanoid();
        let internals = flatten_internals(&subject.body);
        assert_eq!(internals.len(), 9);
    }

    #[test]
    fn flatten_all_humanoid() {
        use crate::creature::humanoid::humanoid::*;
        let subject = humanoid();
        let all = flatten_all(&subject.body);
        assert_eq!(all.len(), 15);
    }
}