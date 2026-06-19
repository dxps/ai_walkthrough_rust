# Part 1, Lesson 2: Multiple inputs

In the previous lesson, the neuron used one input:

$$
y = wx + b
$$

Most useful neurons receive several inputs. The formula becomes:

$$
y = w_1x_1 + w_2x_2 + \dots + w_nx_n + b
$$

For three inputs:

$$
y = w_1x_1 + w_2x_2 + w_3x_3 + b
$$

Each input has its own weight.

<br/>

## 1. Concept

Suppose a neuron receives three inputs:

```
x₁ = 2
x₂ = 3
x₃ = 4
```

It also has three weights:

```
w₁ = 0.5
w₂ = 1.0
w₃ = -1.0
```

And a bias: `b = 2`

The neuron calculates: `y = 0.5×2 + 1.0×3 + (−1.0)×4 + 2`

Calculate each weighted input:

```
0.5 × 2 = 1
1.0 × 3 = 3
-1.0 × 4 = -4
```

Add them together: `1 + 3 - 4 = 0`<br/>
Then add the bias: `0 + 2 = 2`<br/>
The neuron output is: `2`

<br/>

## 2. New terminology

### Multiple inputs

A neuron can receive several values at the same time.

For example, if a neuron were evaluating a house, its inputs might represent:

```
x₁ = size
x₂ = number of rooms
x₃ = age of the house
```

For now, the inputs are simply numbers. Their real-world meaning does not affect the calculation.

### One weight per input

Every input has a corresponding weight:

```
input x₁ uses weight w₁
input x₂ uses weight w₂
input x₃ uses weight w₃
```

The weight determines how strongly that particular input affects the output.<br/>
A positive weight makes the input contribute positively.<br/>
A negative weight makes the input contribute negatively.<br/>
A weight of zero causes the neuron to ignore that input.

### Weighted input

A weighted input is an input multiplied by its corresponding weight: `wi * xi` <br/>
​
For example:

```
input = 4
weight = -1
```

The weighted input is: `-1 × 4 = -4`

### Weighted sum

The weighted sum is the sum of all weighted inputs: `w1 * x1 + w2 * x2 + ... + wn * xn`​

The bias is then added afterward: `y = weighted sum + b`

The complete calculation is still the neuron's forward pass.

<br/>

## 3. Why multiple inputs exist

A prediction usually depends on more than one piece of information.

For example, a model predicting whether a plant will grow might consider:

```
amount of sunlight
amount of water
soil quality
temperature
```

Each input contributes differently. The weights allow the neuron to assign a different level and direction of influence to each input.

A neuron with one input can only respond to one value.

A neuron with several inputs can combine several pieces of information into one output.

$$
y = \sum_{i=1}^{n} w_i x_i + b
$$

<br/>

---

## Run

To run both parts (the implementation and the tests), use:

```
cargo run
cargo test
```
