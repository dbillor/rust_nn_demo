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
