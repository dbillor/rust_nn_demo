mod neural_network;

fn main() {
    let mut nn = neural_network::NeuralNetwork::new(vec![2, 3, 1]);
    println!("Neural network created with structure: {:?}", nn.get_structure());
}
