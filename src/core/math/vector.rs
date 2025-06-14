const DEFAULT_SIZE: usize = 0;

#[cfg(not(feature = "embedded"))]
use ndarray::Array1;

#[cfg(feature = "embedded")]
use heapless::Vec;

pub struct Vector<const N: usize = DEFAULT_SIZE> {
    #[cfg(not(feature = "embedded"))]
    data: Array1<f64>,
    
    #[cfg(feature = "embedded")]
    data: Vec<f64, N>,
}

impl<const N: usize> Vector<N> {
    pub fn new<I>(data: I) -> Self
    where I: IntoIterator<Item=f64>
    {
        #[cfg(not(feature = "embedded"))]
        {
            Vector { data: Array1::from_iter(data) }
        }
        #[cfg(feature = "embedded")]
        {
            if (N == DEFAULT_SIZE) {
                panic!("Must specify a valid Vector size for embedded systems (non-heap allocated)! Please specify a valid size.");
            }
            Vector { data: data.into_iter().collect() }
        }
    }
    
    #[cfg(not(feature = "embedded"))]
    pub fn from_array(data: Array1<f64>) -> Self {
        Vector { data }
    }
    
    pub fn zeros() -> Self {
        if (N == DEFAULT_SIZE) {
            panic!("Must specify a valid size template to initialize as zeros, i.e. Vector::<10>::zeros(); (not Vector::zeros())");
        }
        #[cfg(not(feature = "embedded"))]
        {
            Vector { data: Array1::zeros(N) }
        }
        #[cfg(feature = "embedded")]
        {
            Vector { data: Vec::new() }
        }
    }
 
    pub fn ones() -> Self {
        if (N == DEFAULT_SIZE) {
            panic!("Must specify a valid size template to initialize as ones, i.e. Vector::<10>::ones(); (not Vector::ones())");
        }

        #[cfg(not(feature = "embedded"))]
        {
            Vector { data: Array1::ones(N) }
        }
        #[cfg(feature = "embedded")]
        {
            let mut vec = Vec::new();
            for _ in 0..N { vec.push(1.0).unwrap(); }
            Vector { data: vec }
        }
    }
    
    pub fn linspace(start: f64, end: f64) -> Self {
        #[cfg(not(feature = "embedded"))]
        {
            Vector { data: Array1::linspace(start, end, N) }
        }
        
        #[cfg(feature = "embedded")]
        {
            let step = if N > 1 { (end-start) / (N-1) as f64 } else { 0.0 };
            let mut data: Vec<f64, N> = Vec::new();
            let mut total_: f64 = start;
            for _ in 0..N { 
                data.push(total_).unwrap(); 
                total_ += step;
            }
            Vector { data: data }
        }
    }
    
    pub fn range(start: f64, step: f64) -> Self {
        #[cfg(not(feature = "embedded"))]
        {
            Vector { data: Array1::range(start, start + step * (N as f64), step) }
        }
        
        #[cfg(feature = "embedded")]
        {
            let mut data: Vec<f64, N> = Vec::new();
            let mut total_: f64 = start;
            
            for _ in 0..N {
                data.push(total_).unwrap();
                total_ += step;
            }
            
            Vector { data: data }
        }
    }
}


