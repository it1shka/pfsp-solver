use rand::{Rng, RngExt};

use crate::solver::solution::Solution;

pub trait Operator {
    fn probability(&self) -> f32;
}

pub trait UnaryOperator<R: Rng>: Operator {
    fn mutate(&self, rng: &mut R, s1: &mut Solution);
}

pub trait BinaryOperator<R: Rng>: Operator {
    fn mutate(&self, rng: &mut R, s1: &mut Solution, s2: &Solution);
}

macro_rules! define_operator {
    ($op_name:ident) => {
        #[derive(Clone, Copy)]
        pub struct $op_name {
            p: f32,
        }

        impl Operator for $op_name {
            fn probability(&self) -> f32 {
                self.p
            }
        }
    };
}

define_operator!(SimpleCrossover);
impl<R: Rng> BinaryOperator<R> for SimpleCrossover {
    fn mutate(&self, rng: &mut R, s1: &mut Solution, s2: &Solution) {
        let split = rng.random_range(0..s1.len());
        let direction = rng.random_bool(0.5);
        if direction {
            s1.data[split..].copy_from_slice(&s2.data[split..]);
        } else {
            s1.data[..split].copy_from_slice(&s2.data[..split]);
        }
    }
}

define_operator!(OrderedCrossover);
impl<R: Rng> BinaryOperator<R> for OrderedCrossover {
    fn mutate(&self, rng: &mut R, s1: &mut Solution, s2: &Solution) {
        todo!()
    }
}

define_operator!(PartiallyMatchedCrossover);
impl<R: Rng> BinaryOperator<R> for PartiallyMatchedCrossover {
    fn mutate(&self, rng: &mut R, s1: &mut Solution, s2: &Solution) {
        todo!()
    }
}

define_operator!(CycleCrossover);
impl<R: Rng> BinaryOperator<R> for CycleCrossover {
    fn mutate(&self, rng: &mut R, s1: &mut Solution, s2: &Solution) {
        todo!()
    }
}

define_operator!(SwapMutation);
impl<R: Rng> UnaryOperator<R> for SwapMutation {
    fn mutate(&self, rng: &mut R, s1: &mut Solution) {
        let idx1 = rng.random_range(0..s1.len());
        let mut idx2 = rng.random_range(0..(s1.len() - 1));
        if idx2 >= idx1 {
            idx2 += 1;
        }
        let temp = s1.data[idx1];
        s1.data[idx1] = s1.data[idx2];
        s1.data[idx2] = temp;
    }
}

define_operator!(InversionMutation);
impl<R: Rng> UnaryOperator<R> for InversionMutation {
    fn mutate(&self, rng: &mut R, s1: &mut Solution) {
        let p1 = rng.random_range(0..s1.len() - 1);
        let p2 = rng.random_range((p1 + 1)..s1.len());
        // let inversed = s1.data[p1..p2].into_iter
        // s1.data[p1..p2].copy_from_slice
        todo!()
    }
}
