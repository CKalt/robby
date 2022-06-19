//**************************************************************************
//* For Robby-the-robot fitness function
//*
//*  Example usage:
//*     How to generate a random Action with uniform distribution.
//*
//*     use rand::Rng;
//*     use crate::srand::RngFactory;
//*
//*     let mut srng = RngFactory::new(Some(28282));
//*     let action: Action = srng.gen();
//*     println!("Random Action = {:?}", action);
//*
//**************************************************************************

use rand::Rng;
use rand::{
    distributions::{Distribution, Standard},
}; // 0.8.0

#[derive(Debug)]
pub enum Action {
    MoveNorth = 0,
    MoveSouth = 1,
    MoveEast = 2,
    MoveWest = 3,
    StayPut = 4,
    Pickup = 5,
    RandomMove = 6,
}

impl Distribution<Action> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Action {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=6) { // rand 0.8
            0 => Action::MoveNorth,
            1 => Action::MoveSouth,
            2 => Action::MoveEast,
            3 => Action::MoveWest,
            4 => Action::StayPut,
            5 => Action::Pickup,
            _ => Action::RandomMove,
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use crate::app::actions::Action;
    use crate::srand::RngFactory;
    #[test]
    #[cfg(opt_rng="ChaCha8Rng")]
    fn test_random_actions() {
        let mut srng = RngFactory::new(Some(2810283));
        let action: Action = srng.gen();
        if let Action::StayPut = action {
            println!("Expected Random Action = {:?}", action);
        }
        else {
            panic!("invalid random action");
        }

        let action: Action = srng.gen();
        if let Action::MoveWest = action {
            println!("Expected Random Action = {:?}", action);
        }
        else {
            panic!("invalid random action");
        }

        let action: Action = srng.gen();
        if let Action::Pickup = action {
            println!("Expected Random Action = {:?}", action);
        }
        else {
            panic!("invalid random action");
        }
    }
    #[test]
    #[cfg(opt_rng="ChaCha20Rng")]
    fn test_random_actions() {
        let mut srng = RngFactory::new(Some(10283));
        let action: Action = srng.gen();
        if let Action::MoveEast = action {
            println!("Expected Random Action = {:?}", action);
        }
        else {
            panic!("invalid random action");
        }

        let action: Action = srng.gen();
        if let Action::Pickup = action {
            println!("Expected Random Action = {:?}", action);
        }
        else {
            panic!("invalid random action");
        }

        let action: Action = srng.gen();
        if let Action::StayPut = action {
            println!("Expected Random Action = {:?}", action);
        }
        else {
            panic!("invalid random action");
        }
    }
}
