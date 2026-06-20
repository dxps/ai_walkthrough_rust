# Part 1, Lesson 4: Prediction

Lessons 1-3 built the complete numerical calculations:

$$
z = \sum_{i} x_i w_i + b
$$

$$
y = f(z)
$$

Lesson 3 deparated the weighted sum $z$ from the activated output $y$ and implemented identity, ReLU, and sigmoid functions.

<br/>

## 1. Concept

A neuron produces a number. A _prediction_ assigns meaning to that number.

Suppose a neuron is intended to answer a yes-or-no question:

> Will a plant grow successfully?

We could define:

- false means “the model predicts no”
- true means “the model predicts yes”

A sigmoid activation produces a value between 0 and 1.

For example:

- considering `neuron output = 0.73`
- to convert this number into a yes-or-no prediction, we choose a boundary<br/>
  `threshold = 0.5`
- then apply this rule:
    - output >= threshold => true
    - output < threshold => false

The boundary is called a _threshold_.

<br/>

## 2. Why a prediction step exists

The neuron’s output and the prediction are not necessarily the same type of value. The neuron might produce `0.73`, but the program using the neuron might require `true`.

The complete process is therefore:

```
inputs
  |
  v
weighted sum
  |
  v
  z
  |
  v
sigmoid activation
  |
  v
numerical output
  |
  v
threshold
  |
  v
prediction
```

In mathematical form:

$$
z = \sum_{i} x_i w_i + b
$$

$$
y = f(z)
$$

$$
prediction = \begin{cases}
    \text{true} & \text{if } y \geq \text{threshold} \\
    \text{false} & \text{otherwise}
\end{cases}
$$

<br/>

## 3. Small example by hand

- Use two inputs: $x_1 = 2.0$ and $x_2 = 1$
- Use these parameters: $w_1 = 0.5$, $w_2 = -0.5$, $bias = 0.5$
- First, calculate the weighted sum: $z=2.0×0.5+1.0×(−0.5)+0.5$<br/>
  Thus, $z = 1.0 - 0.5 + 0.5 = 1.0$
- Apply sigmoid: $y = \frac{1}{1 + e^{-1}}$ <br/>
  Thus, $y = 0.731$
- Appy the threshold: $0.731 >= 0.5$ <br/>
- Therefore, $prediction = \text{true}$

<br/>

## 4. An important distinction

These three values represent different stages:

```
weighted sum:  1.0
output:        0.731...
prediction:    true
```

- The weighted sum is the value before activation.
- The output is the value after activation.
- The prediction is the interpretation of that output.

For now, do not treat 0.731 as a trustworthy real-world probability. The neuron has not learned its parameters from data. Its weights and bias were selected manually, so this lesson demonstrates computation rather than an accurate model.

<br/>

## Summary

<br/>

---

## Run

To run both parts (the implementation and the tests), use:

```
cargo run
cargo test
```

$$
$$
