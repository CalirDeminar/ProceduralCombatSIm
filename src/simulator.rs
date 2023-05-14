pub mod simulator {
    use crate::creature::body::body::*;
    use crate::creature::creature::*;
    use crate::creature::humanoid::humanoid::*;
        use rand::Rng;

    pub fn build_force(size: usize) -> Vec<Creature> {
        let mut output: Vec<Creature> = vec![];
        for _i in 0..size {
            output.push(humanoid());
        }
        return output;
    }

    fn random_target<'a>(force: Vec<&'a mut Creature>) -> &'a mut Creature {
        let total_size = force.iter().fold(0, |acc, c| acc + sum_child_part_size_r(&c.body));

        let mut rng = rand::thread_rng();
        let r: f32 = rng.gen();
        let roll = (r*total_size as f32) as u32;
        let mut total = 0;
        for c in force {
            total += sum_child_part_size_r(&c.body);
            if roll > total {
                return c;
            }
        }
        panic!();
    }

    fn run_combat<'a>(force_a: Vec<&'a mut Creature>, force_b: Vec<&'a mut Creature>) -> (Vec<&'a mut Creature>, Vec<&'a mut Creature>){
        let max_length = force_a.len().max(force_b.len());
        let mut a_clone = force_a.iter().map(|p| p.clone()).collect::<Vec<_>>();
        let mut b_clone = force_b.iter().map(|p| p.clone()).collect::<Vec<_>>();
        for i in 0..max_length {
            if i < force_a.len() {
                // let target = random_target(force_b);

                
            }
            if i < force_b.len() {

            }
        }
        return (force_a, force_b);
    }
}