struct VecNorm {
    position: Vec<f64>,
    norm1: f64,
    norm2: f64,
}

impl VecNorm {
    fn new() -> VecNorm {
        VecNorm {
            position: Vec::<f64>::new(),
            norm1: 0.0,
            norm2: 0.0,
        }
    }

    fn add(&mut self, value: f64) {
        self.position.push(value);
        self.norm1();
        self.norm2();
    }

    fn norm1(&mut self) {
        self.norm1 = self.position.iter().sum();
    }

    fn norm2(&mut self) {
        let norm2: f64 = self.position.iter().sum();
        self.norm2 = norm2.sqrt();
    }
}

fn main() {
    let mut vec1 = VecNorm::new();
    vec1.add(5.0);
    vec1.add(7.0);

    println!("L1 norm of vector {:?} is {}", vec1.position, vec1.norm1);
    println!("L2 norm of vector {:?} is {}", vec1.position, vec1.norm2);
}
