# Part 1, Lesson 2: Multiple inputs

In the previous lesson, a neuron calculated a weighted sum:

$$
z = x_1 * w_1 + x_2 * w_2 + \dots + x_n * w_n  b
$$

where:

- $x_i$ are inputs.
- $w_i$ are weights.
- $b$ is the bias.
- $z$ is the weighted sum.

An activation function adds one final operation: $y = f(z)$

The weighted sum `z` is sometimes called the neuron's _pre-activation value_. The final result `y` is the neuron's _output_ or _activation_.

<br/>

## 1. Concept

An _activation function_ is a function applied to a neuron's weighted sum.

Without an activation function: $y=z$

With an activation function: $y=f(z)$

A neuron therefore performs two distinc steps:

1. Combine its inputs using weights and a bias.
2. Transform that result using an activation function.

For two inputs:

$$
z = x_1 * w_1 + x_2 * w_2 + b
$$

$$
y = f(z)
$$

<br/>

## 2. Why activation functions exist

Suppose we connect several neurons together, but every neuron only calculates weighted sums.

The first neuron might calculate: $h=xw_1+b_1$

A second neuron then uses $h$: $y=hw_2+b_2$

Subsitute the first equation into the second: $y=(xw_1+b_1)w_2+b_2$

Expand it: $y=xw_1w_2+b_1w_2+b_2$

The result is still equivalent to: $y=xW+B$

where $W$ and $B$ are some combined weight and bias.

This means that stacking weighted sums does not give the network fundamentally new behavior. Ten such layers could still be replaced by one weighted-sum layer.

Activation functions solve this by introducing _nonlinearity_.

A nonlinear function cannot generally be reduced to multiplication and addition. Once nonlinear activation functions are placed between layers, stacking layers allows a network to represent more complicated relationships.

We will see this directly when we build the XOR network in Part 3.

<br/>

## 3. The identity activation

The simplest activation function is the _identity function_: $f(z)=z$ <br/>
It returns its input unchanged.<br/>
Using the identity function is the same as not using an activation function.<br/>
It is still useful in some places, particularly when a model should produce an unrestricted numerical output. However, it does not introduce nonlinearity.

<br/>

## 4. ReLU (activation function)

A common activation function is the _rectified linear unit_ (ReLU, pronounced "ree-loo"), defined as: $ReLU(z)=max(0,z)$

In plain language, if $z$ is negative, return $0$. Otherwise, return $z$.

ReLU is nonlinear because its behavior changes at zero. There is no single multiplication and addition operation that behaves like ReLU for all possible inputs.

Its graph has this general shape:

```
output
  ^
  |          /
  |         /
  |        /
  |_______/________> input
          0
```

<br/>

## 5. A complete calculation by hand

- Consider a neuron with two inputs: $x_1=2$ and $x_2=-3$.
- Its parameters are: $w_1=0.5$, $w_2=1.0$, and $b=1.0$.
- First, calculate the weighted sum: $z=x_1w_1+x_2w_2+b$
- Subsitute the values: $z = 2*0.5-3*1.0+1.0 = 1-3+1 = -1$.
- Now, apply ReLU: $y=ReLU(z)=ReLU(-1)=max(0,-1)=0$

The neuron's weighted sum is $-1$, but its final output is $0$.

The distinction is important:

```
  weighted sum / pre-activation: -1.0
  activation / final output:      0.0
```

<br/>

## 6. Sigmoid (activation function)

Another historically important activation function is the sigmoid function: $\sigma(z) = \frac{1}{1 + e^{-z}}$

<br/>

---

## Run

To run both parts (the implementation and the tests), use:

```
cargo run
cargo test
```
