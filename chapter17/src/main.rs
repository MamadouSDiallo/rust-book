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
        let vec_len: usize = self.position.len();
        let mut norm1: f64 = 0.0;
        for i in 0..vec_len {
            norm1 = norm1 + self.position[i].abs()
        }
        self.norm1 = norm1;
    }

    fn norm2(&mut self) {
        let vec_len: usize = self.position.len();
        let mut norm2: f64 = 0.0;
        for i in 0..vec_len {
            norm2 = norm2 + self.position[i].powi(2)
        }
        self.norm2 = norm2.sqrt();
    }
}

fn main() {
    let mut vec1 = VecNorm::new();
    vec1.add(5.0);
    vec1.add(7.0);
    vec1.add(45.0);

    println!("L1 norm of vector {:?} is {}", vec1.position, vec1.norm1);
    println!("L2 norm of vector {:?} is {}", vec1.position, vec1.norm2);
}
