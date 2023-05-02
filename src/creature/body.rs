pub mod body {
    use rand::Rng;

    #[derive(Debug, Clone, Copy)]
    pub enum BodyPartTags {
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
    #[derive(Debug, Clone)]
    pub struct BodyPart {
        pub name: String,
        pub tags: Vec<BodyPartTags>,
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
    fn flatten_body(body: &BodyPart) -> Vec<&BodyPart> {
        if body.children.len() == 0 {
            return vec![&body];
        }
        let mut children: Vec<Vec<&BodyPart>> = vec![vec![body]];
        for child in &body.children {
            children.push(flatten_body(&child));
        }

        return children.concat();
    }
    fn flatten_internals(body: &BodyPart) -> Vec<&BodyPart> {
        if body.children.len() == 0 {
            return vec![&body];
        }
        let mut children: Vec<Vec<&BodyPart>> = vec![vec![body]];
        for child in &body.internal {
            children.push(flatten_internals(&child));
        }

        return children.concat();
    }
    pub fn random_weighted_part(body: &BodyPart) -> &BodyPart {
        let all_parts = flatten_body(&body);
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
        let all_parts = flatten_internals(&body);
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
}