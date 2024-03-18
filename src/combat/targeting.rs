pub mod targeting {
    use crate::creature::{body::body::BodyPart, creature::Creature, organs::organs::Organ};
    use rand::Rng;

    fn random_part_is_selected<'a>(body: &'a mut BodyPart, count: u32, roll: u32) -> (u32, Option<&'a mut BodyPart>) {
        let c = count + body.size;
        if c > roll {
            return (c, Some(body));
        }
        return body.children.iter_mut()
            .fold((c, None), |(acc_c, acc_r), p| if acc_r.is_none() { random_part_is_selected(p, acc_c, roll) } else {(c, acc_r)});
    }

    impl Creature {
        pub fn random_weighted_body_part<'a>(self: &'a mut Self) -> &'a mut BodyPart {
            if self.body.children.len() == 0 {
                 return &mut self.body;
            }
            let total_size = self.body.sum_child_part_size_r();

            let mut rng = rand::thread_rng();

            let (_, rtn) = random_part_is_selected(&mut self.body, 0, (rng.gen::<f32>() * total_size as f32) as u32);
            rtn.unwrap()
        }
    }
    impl BodyPart {
        pub fn random_weighted_organ<'a>(self: &'a mut Self) -> Option<&'a mut Organ> {
            if self.organs.len() == 0 {
                return None;
            }
            let total_organ_size = self.organs.iter().fold(0, |acc, o| acc + o.size);

            let mut rng = rand::thread_rng();
            let roll = rng.gen::<f32>();
            if roll > (total_organ_size / self.size) as f32 {
                return None;
            }
            let mut total: u32 = 0;
            for organ in &mut self.organs {
                total += organ.size;
                if total as f32 > roll {
                    return Some(organ);
                }
            }
            None
        }
    }
}