use faster::{ PackedIterator };
use manyvectors::ManyVectors;


/// Core support vector machine 
#[derive(Debug)]
pub struct SVM<T> {
    /// Total number of support vectors
    pub total_support_vectors: usize,
    
    /// Number of attributes per support vector
    pub num_attributes: usize,
    
    pub rho: Vec<f64>,
    
    pub extra: T,
    
    /// All classes
    pub classes: Vec<Class>,
}



/// Represents one class of the SVM model. 
#[derive(Debug)]
pub struct Class {
    /// The label of this class
    pub label: u32,

    /// The number of support vectors in this class
    pub num_support_vectors: usize,

    /// Coefficients between this class and n-1 other classes. 
    pub coefficients: ManyVectors<f64>,

    /// All support vectors in this class.
    pub support_vectors: ManyVectors<f32>,
}



/// A single problem we should classify.
#[derive(Debug)]
pub struct Problem {
    /// A vector of `num_attributes` features.
    pub features: Vec<f32>,

    /// Kernel values. A vector for each class. 
    pub kernel_values: ManyVectors<f64>,

    /// All votes for a given class label.
    pub vote: Vec<u32>,

    /// Decision values. 
    pub decision_values: Vec<f64>,

    /// Computed label. This is what we update eventually.
    pub label: u32
}


impl Class {

    pub fn with_dimensions(classes: usize, support_vectors: usize, attributes: usize) -> Class {
        Class {
            label: 0,
            num_support_vectors: support_vectors,
            coefficients: ManyVectors::with_dimension(classes - 1, support_vectors, Default::default()),
            support_vectors: ManyVectors::with_dimension(support_vectors, attributes, Default::default()),
        }
    }
    
}



impl Problem {

    pub fn with_dimension(total_sv: usize, num_classes: usize, num_attributes: usize) -> Problem {
        let num_decision_values = num_classes * (num_classes - 1) / 2;

        Problem {
            features: vec![Default::default(); num_attributes],
            kernel_values: ManyVectors::with_dimension(num_classes, total_sv, Default::default()),
            decision_values: vec![Default::default(); num_decision_values],
            vote: vec![Default::default(); num_classes],
            label: 0
        }
    }

    pub fn from_svm<T>(svm: &SVM<T>) -> Problem {
        Problem::with_dimension(svm.total_support_vectors, svm.classes.len(), svm.num_attributes)
    }

}
