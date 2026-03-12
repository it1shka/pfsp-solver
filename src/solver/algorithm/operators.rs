use crate::solver::solution::Solution;

pub trait Operator {
    fn probability(&self) -> f32;
}

pub trait UnaryOperator: Operator {
    fn mutate(&self, s1: &mut Solution);
}

pub trait BinaryOperator: Operator {
    fn mutate(&self, s1: &mut Solution, s2: &Solution);
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

define_operator!(OrderedCrossover);
impl BinaryOperator for OrderedCrossover {
    fn mutate(&self, s1: &mut Solution, s2: &Solution) {
        todo!()
    }
}

define_operator!(PartiallyMatchedCrossover);
impl BinaryOperator for PartiallyMatchedCrossover {
    fn mutate(&self, s1: &mut Solution, s2: &Solution) {
        todo!()
    }
}

define_operator!(CycleCrossover);
impl BinaryOperator for CycleCrossover {
    fn mutate(&self, s1: &mut Solution, s2: &Solution) {
        todo!()
    }
}

define_operator!(SwapMutation);
impl UnaryOperator for SwapMutation {
    fn mutate(&self, s1: &mut Solution) {
        todo!()
    }
}

define_operator!(InversionMutation);
impl UnaryOperator for InversionMutation {
    fn mutate(&self, s1: &mut Solution) {
        todo!()
    }
}
