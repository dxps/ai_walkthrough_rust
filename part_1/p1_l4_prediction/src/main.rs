/// Part 1, Lesson 4: Prediction
///
/// This shows three distinct concepts:
/// - `weighted_sum` : internal pre-activation value
/// - `output`       : activated numerical value
/// - `predict`      : task-specific interpretation
///
/// For this design, prediction should not be part of the neuron itself.
///
/// A neuron’s responsibility is numerical computation:
/// `inputs → weighted sum → activation → output`
///
/// The prediction step interprets that output according to a particular task:
/// `output → threshold → bool``
///

struct Neuron {
    weights: Vec<f64>,
    bias: f64,
}

impl Neuron {
    fn new(weights: Vec<f64>, bias: f64) -> Self {
        Self { weights, bias }
    }

    fn weighted_sum(&self, inputs: &[f64]) -> f64 {
        assert_eq!(
            inputs.len(),
            self.weights.len(),
            "each input must have a corresponding weight"
        );

        let mut sum = self.bias;

        for (input, weight) in inputs.iter().zip(self.weights.iter()) {
            sum += input * weight;
        }

        sum
    }

    fn output(&self, inputs: &[f64]) -> f64 {
        sigmoid(self.weighted_sum(inputs))
    }
}

fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

fn predict(output: f64, threshold: f64) -> bool {
    output >= threshold
}

fn main() {
    let neuron = Neuron::new(vec![0.5, -0.5], 0.5);
    let inputs = [2.0, 1.0];
    let threshold = 0.5;

    let z = neuron.weighted_sum(&inputs);
    let output = neuron.output(&inputs);
    let prediction = predict(output, threshold);

    println!("Weighted sum: {z}");
    println!("Output:       {output}");
    println!("Threshold:    {threshold}");
    println!("Prediction:   {prediction}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neuron_calculates_weighted_sum() {
        let neuron = Neuron::new(vec![0.5, -0.5], 0.5);
        let inputs = [2.0, 1.0];

        assert_eq!(neuron.weighted_sum(&inputs), 1.0);
    }

    #[test]
    fn neuron_calculates_activated_output() {
        let neuron = Neuron::new(vec![0.5, -0.5], 0.5);
        let inputs = [2.0, 1.0];

        let output = neuron.output(&inputs);
        let expected = 0.731_058_578_630_004_9;

        assert!((output - expected).abs() < 1e-12);
    }

    #[test]
    fn prediction_is_true_at_or_above_threshold() {
        assert!(predict(0.5, 0.5));
        assert!(predict(0.8, 0.5));
    }

    #[test]
    fn prediction_is_false_below_threshold() {
        assert!(!predict(0.49, 0.5));
        assert!(!predict(0.1, 0.5));
    }

    #[test]
    fn complete_neuron_produces_expected_prediction() {
        let neuron = Neuron::new(vec![0.5, -0.5], 0.5);
        let inputs = [2.0, 1.0];

        let output = neuron.output(&inputs);
        let prediction = predict(output, 0.5);

        assert!(prediction);
    }
}
