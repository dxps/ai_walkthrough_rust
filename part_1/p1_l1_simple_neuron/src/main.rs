fn neuron(input: f64, weight: f64, bias: f64) -> f64 {
    weight * input + bias
}

fn main() {
    let input = 3.0;
    let weight = 2.0;
    let bias = 1.0;

    let output = neuron(input, weight, bias);

    println!("Neuron output: {output}");
}

#[cfg(test)]
mod tests {
    use super::neuron;

    #[test]
    fn neuron_calculates_weighted_input_plus_bias() {
        let output = neuron(3.0, 2.0, 1.0);

        assert_eq!(output, 7.0);
    }
}