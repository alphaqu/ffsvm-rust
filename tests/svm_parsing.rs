#![feature(test)]
#![feature(try_from)]

extern crate test;

#[cfg(test)]
mod svm_performance {
    use ffsvm::{DenseSVM, Error, Predict, Problem, Solution};
    use std::convert::TryFrom;
    
    #[test]
    fn load_large() -> Result<(), Error> {
        let model = include_str!("data_misc/model_large.libsvm");
        let svm = DenseSVM::try_from(model)?;
        
        Ok(())
    }
}
