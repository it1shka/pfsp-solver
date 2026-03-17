use std::collections::HashSet;

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

define_operator!(OrderedCrossover);
impl<R: Rng> BinaryOperator<R> for OrderedCrossover {
    fn mutate(&self, rng: &mut R, s1: &mut Solution, s2: &Solution) {
        let n = s1.len();
        let p1 = rng.random_range(0..(n - 1));
        let p2 = rng.random_range((p1 + 1)..n);
        let mut present = vec![false; n];
        for &v in &s1.data[p1..p2] {
            present[v] = true;
        }
        let mut s2_ordered = s2.data[p2..]
            .iter()
            .copied()
            .chain(s2.data[..p2].iter().copied())
            .filter(|&v| !present[v]);
        for v in s1.data[p2..].iter_mut() {
            *v = s2_ordered.next().unwrap();
        }
        for v in s1.data[..p1].iter_mut() {
            *v = s2_ordered.next().unwrap();
        }
    }
}

define_operator!(PartiallyMatchedCrossover);
impl<R: Rng> BinaryOperator<R> for PartiallyMatchedCrossover {
    fn mutate(&self, rng: &mut R, s1: &mut Solution, s2: &Solution) {
        let n = s1.len();
        let p1 = rng.random_range(0..(n - 1));
        let p2 = rng.random_range((p1 + 1)..n);

        let mut mapping = vec![None; n];
        for i in p1..p2 {
            let from = s2.data[i];
            let to = s1.data[i];
            mapping[from] = Some(to);
        }

        s1.data[p1..p2].copy_from_slice(&s2.data[p1..p2]);
        for i in (0..p1).chain(p2..n) {
            while let Some(mapped) = mapping[s1.data[i]] {
                s1.data[i] = mapped;
            }
        }
    }
}

define_operator!(CycleCrossover);
impl<R: Rng> BinaryOperator<R> for CycleCrossover {
    fn mutate(&self, rng: &mut R, s1: &mut Solution, s2: &Solution) {
        let n = s1.len();
        let mut visited = vec![false; n];
        let mut copy_flag = rng.random_bool(0.5);
        let mut s1_positions = vec![0; n];
        for (i, &v) in s1.data.iter().enumerate() {
            s1_positions[v] = i;
        }
        for i in 0..n {
            if visited[i] {
                continue;
            }
            let mut cycle_pointer = i;
            while !visited[cycle_pointer] {
                visited[cycle_pointer] = true;
                if copy_flag {
                    s1.data[cycle_pointer] = s2.data[cycle_pointer];
                }
                cycle_pointer = s1_positions[s2.data[cycle_pointer]];
            }
            copy_flag = !copy_flag;
        }
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
        s1.data[p1..p2].reverse();
    }
}
