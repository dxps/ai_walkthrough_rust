/// `weighted_sum` and `relu` are separate functions, just to separate the responsibilities.
/// This separation makes the calculation explicit:
/// inputs
///    |
///    v
/// weighted sum
///    |
///    v
/// pre-activation value z
///    |
///    v
/// activation function
///    |
///    v
/// final output
///
/// The weighted-sum function does not need to know which activation function will be used.
/// We can replace ReLU without changing the weighted-sum code.

fn weighted_sum(inputs: &[f64], weights: &[f64], bias: f64) -> f64 {
    assert_eq!(
        inputs.len(),
        weights.len(),
        "each input must have a corresponding weight"
    );

    let mut sum = bias;

    for (input, weight) in inputs.iter().zip(weights.iter()) {
        sum += input * weight;
    }

    sum
}

fn identity(z: f64) -> f64 {
    z
}

fn relu(z: f64) -> f64 {
    z.max(0.0)
}

fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

fn main() {
    let inputs = [2.0, -3.0];
    let weights = [0.5, 1.0];
    let bias = 1.0;

    let z = weighted_sum(&inputs, &weights, bias);

    println!("Weighted sum: {z}");
    println!("Identity    : {}", identity(z));
    println!("ReLU        : {}", relu(z));
    println!("Sigmoid     : {}", sigmoid(z));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_returns_input_unchanged() {
        assert_eq!(identity(-3.0), -3.0);
        assert_eq!(identity(0.0), 0.0);
        assert_eq!(identity(4.5), 4.5);
    }

    #[test]
    fn relu_replaces_negative_values_with_zero() {
        assert_eq!(relu(-3.0), 0.0);
        assert_eq!(relu(0.0), 0.0);
        assert_eq!(relu(4.5), 4.5);
    }

    #[test]
    fn sigmoid_of_zero_is_one_half() {
        assert_eq!(sigmoid(0.0), 0.5);
    }

    #[test]
    fn neuron_calculation_matches_hand_calculation() {
        let inputs = [2.0, -3.0];
        let weights = [0.5, 1.0];
        let bias = 1.0;

        let z = weighted_sum(&inputs, &weights, bias);
        let output = relu(z);

        assert_eq!(z, -1.0);
        assert_eq!(output, 0.0);
    }
}
