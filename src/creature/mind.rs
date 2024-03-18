pub mod mind {
    use rand::Rng;

    #[derive(Debug, Clone)]
    pub struct Mind {
        // multpliers (0.5-1.5)
        pub pain_tolerence: f32,
        pub endurance: f32,
        // skill (standins)
        pub combat_skill: f32,
        pub dodging_skill: f32
    }

    pub fn random_mind() -> Mind {
        let mut rng = rand::thread_rng();
        Mind {
            pain_tolerence: rng.gen::<f32>() + 0.5,
            endurance: rng.gen::<f32>() + 0.5,
            combat_skill: rng.gen::<f32>() + 0.5,
            dodging_skill: rng.gen::<f32>() + 0.5,
        }
    }
}