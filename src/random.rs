use std::marker::{Copy, Sized};
use std::iter::Iterator;
use rand::{ChaChaRng, Rand, Rng};

use vectors::SimdOptimized;

/// Randomizes a data structure
pub trait Randomize {
    /// Randomizes data in a structure (mostly its vectors) within the structure's parameters.
    fn randomize(self) -> Self;
}



impl<T> Randomize for SimdOptimized<T>
where
    T: Sized + Copy + Rand,
{
    fn randomize(mut self) -> Self {
        let mut rng = ChaChaRng::new_unseeded();

        for i in 0..self.vectors {
            let gen = rng.gen_iter::<T>();
            let vector = gen.take(self.attributes).collect::<Vec<T>>();

            self.set_vector(i, vector.as_slice());
        }

        self
    }
}




/// Creates a vector of random
pub fn random_vec<T>(size: usize) -> Vec<T>
where
    T: Rand,
{
    let mut rng = ChaChaRng::new_unseeded();
    rng.gen_iter().take(size).collect()
}
