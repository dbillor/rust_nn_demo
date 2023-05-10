use rand::Rng;

#[derive(Debug)]
struct Neuron {
    weights: Vec<f64>,
    bias: f64,
}

#[derive(Debug)]
pub struct NeuralNetwork {
    layers: Vec<Vec<Neuron>>,
}

impl NeuralNetwork {
    pub fn new(structure: Vec<usize>) -> Self {
        let mut rng = rand::thread_rng();
        let layers = structure.windows(2).map(|layer_sizes| {
            (0..layer_sizes[1]).map(|_| {
                Neuron {
                    weights: (0..layer_sizes[0]).map(|_| rng.gen_range(-1.0..1.0)).collect(),
                    bias: rng.gen_range(-1.0..1.0),
                }
            }).collect()
        }).collect();

        NeuralNetwork { layers }
    }

    pub fn get_structure(&self) -> Vec<usize> {
        self.layers.iter().map(|layer| layer.len()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neural_network_creation() {
        let nn = NeuralNetwork::new(vec![2, 3, 1]);
        let structure = nn.get_structure();
        assert_eq!(structure, vec![3, 1]);
    }
}
