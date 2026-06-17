# Part 1

This part show what a neural network computes.

It covers:

- Input
- Parameter
- Weight
- Bias
- Neuron
- Weighted sum
- Activation function
- Prediction

The implementation contains one artificial neuron. No automatic differentiation, tensors or machine-learning libraries.

So, basically it starts with one neuron and one calculation. No training yet.

## What does a neuron calculate?

A basic artificial neuron computes `y = w * x + b`

Where:

- x is the input
- w is the weight
- b is the bias
- y is the output

For now, think of the weight as controlling how strongly the input affects the result.
The bias shifts the result independently of the input.

Using:

```
input  x = 3
weight w = 2
bias   b = 1
```

The neuron calculates `y = 2×3+1 = 7`

This calculation is also called the neuron's _forward pass_: data moves forward from input to output.
