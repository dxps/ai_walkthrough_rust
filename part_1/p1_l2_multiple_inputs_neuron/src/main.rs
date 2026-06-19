fn neuron(inputs: &[f64], weights: &[f64], bias: f64) -> f64 {
    assert_eq!(
        inputs.len(),
        weights.len(),
        "inputs and weights must be the same length"
    );
    let mut weighted_input = 0.0;
    for idx in 0..inputs.len() {
        weighted_input += inputs[idx] * weights[idx];
    }
    weighted_input + bias
}

fn main() {
    let inputs = [2.0, 3.0, 4.0];
    let weights = [0.5, 1.0, -1.0];
    let bias = 2.0;

    let output = neuron(&inputs, &weights, bias);

    println!("Neuron output: {output}");
}

#[cfg(test)]
mod tests {

    use super::neuron;

    #[test]
    fn neuron_calculates_multiple_weighted_inputs_plus_bias() {
        let inputs = [2.0, 3.0, 4.0];
        let weights = [0.5, 1.0, -1.0];
        let bias = 2.0;

        let output = neuron(&inputs, &weights, bias);

        assert_eq!(output, 2.0);
    }

    #[test]
    fn neuron_supports_different_numbers_of_inputs() {
        let inputs = [1.0, 2.0, 3.0, 4.0];
        let weights = [1.0, 1.0, 1.0, 1.0];

        let output = neuron(&inputs, &weights, 0.0);

        assert_eq!(output, 10.0);
    }

    #[test]
    #[should_panic(expected = "inputs and weights must be the same length")]
    fn neuron_rejects_different_input_and_weight_lengths() {
        let inputs = [1.0, 2.0, 3.0];
        let weights = [1.0, 2.0];

        neuron(&inputs, &weights, 0.0);
    }
}
