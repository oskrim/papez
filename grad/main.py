import math
import numpy as np
np.random.seed(42)


def pp(node, indent=0):
  if isinstance(node, list):
    return [pp(n, indent) for n in node]
  print(' ' * indent, node)
  for child in node._children:
    pp(child, indent + 2)


class Value(object):
  def __init__(self, data, children=(), op=''):
    self.data = data
    self._grad = 0.0
    self._children = children
    self._op = op

  def __repr__(self):
    return "Value(%.4f, %.4f, %s)" % (self.data, self._grad, self._op)

  def __add__(self, other):
    if isinstance(other, (int, float)):
      other = Value(other)
    out = Value(self.data + other.data, children=(self, other), op='+')
    return out

  def __mul__(self, other):
    if isinstance(other, (int, float)):
      other = Value(other)
    out = Value(self.data * other.data, children=(self, other), op='*')
    return out

  def __pow__(self, other):
    if isinstance(other, (int, float)):
      other = Value(other)
    out = Value(self.data ** other.data, children=(self, other), op='**')
    return out

  def __radd__(self, other):
    return self + other

  def __rmul__(self, other):
    return self * other

  def __rpow__(self, other):
    return other**self

  def __sub__(self, other):
    return self + (-other)

  def __neg__(self):
    return self * Value(-1.0)

  def __truediv__(self, other):
    return self * other**Value(-1.0)

  def exp(self):
    out = Value(np.exp(self.data), children=(self,), op='exp')
    return out

  def relu(self):
    out = Value(np.maximum(self.data, 0), children=(self,), op='relu')
    return out

  def tanh(self):
    out = Value(np.tanh(self.data), children=(self,), op='tanh')
    return out

  def grad(self):
    if self._grad == 0.0:
      self._grad = 1.0
    return self._grad

  def backward(self):
    match self._op:
      case '+':
        self._children[0]._grad += self.grad()
        self._children[1]._grad += self.grad()
      case '*':
        self._children[0]._grad += self.grad() * self._children[1].data
        self._children[1]._grad += self.grad() * self._children[0].data
      case '**':
        self._children[0]._grad += self.grad() * self._children[1].data * self.data / self._children[0].data
        self._children[1]._grad += self.grad() * self.data * math.log(math.fabs(self._children[0].data))
      case 'relu':
        self._children[0]._grad += self.grad() * (self._children[0].data > 0)
      case 'tanh':
        self._children[0]._grad += self.grad() * (1 - self.data ** 2)
      case 'exp':
        self._children[0]._grad += self.grad() * np.exp(self._children[0].data)
      case _:
        pass
    for child in self._children:
      child.backward()

  def zero_grad(self):
    self._grad = 0.0
    for child in self._children:
      child.zero_grad()


class Neuron(object):
  def __init__(self, dim):
    self.w = [Value(np.random.uniform(-1, 1)) for _ in range(dim)]
    self.b = Value(np.random.uniform(-1, 1))

  def __call__(self, inputs):
    return self.forward(inputs)

  def forward(self, inputs):
    act = sum((w * x for w, x in zip(self.w, inputs)), self.b)
    return act.tanh()

  def parameters(self):
    return self.w + [self.b]


class Layer(object):
  def __init__(self, dim_in, dim_out):
    self.neurons = [Neuron(dim_in) for _ in range(dim_out)]

  def __call__(self, inputs):
    return self.forward(inputs)

  def forward(self, inputs):
    return [n(inputs) for n in self.neurons]

  def parameters(self):
    return [p for n in self.neurons for p in n.parameters()]


class MLP(object):
  def __init__(self, *dims):
    self.layers = [Layer(dim_in, dim_out) for dim_in, dim_out in zip(dims[:-1], dims[1:])]

  def __call__(self, inputs):
    return self.forward(inputs)

  def forward(self, inputs):
    for layer in self.layers:
      inputs = layer(inputs)
    return inputs

  def parameters(self):
    return [p for layer in self.layers for p in layer.parameters()]


xs = [
  # Value(1.0),
  # Value(2.0),
  [Value(1.0), Value(1.0)],
  [Value(1.0), Value(1.0)],
  [Value(-1.0), Value(3.0)],
  [Value(1.0), Value(5.0)],
]

ys = [
  Value(0.5),
  Value(0.5),
  Value(-0.5),
  Value(-1.0),
]

class Simple(object):
  def __init__(self):
    self.w = Value(np.random.uniform(-1, 1))
    self.b = Value(np.random.uniform(-1, 1))

  def __call__(self, x):
    return self.forward(x)

  def forward(self, x):
    return (self.w * x + self.b).tanh()

  def parameters(self):
    return [self.w, self.b]

mlp = MLP(2, 3, 1)
# mlp = Neuron(2)

for i in range(3000):
  ypred = [mlp(x)[0] for x in xs]
  loss = sum(((y - yhat)**Value(2.0) for y, yhat in zip(ys, ypred)), Value(0.0))

  print("%.4f" % loss.data, end=' ')
  for yhat in ypred:
    print("%.4f" % yhat.data, end=' ')
  print()

  loss.backward()
  for p in mlp.parameters():
    p.data -= 0.1 * p.grad()
  loss.zero_grad()

assert (Value(5.0) + Value(4.0)).data == 9.0
assert (Value(5.0) * Value(4.0)).data == 20.0

one = Value(5.0)
two = Value(7.0)
three = Value(3.0)
compute = (one + two) * three
compute.backward()
assert compute.data == 36.0
assert one.grad() == 3.0
assert two.grad() == 3.0
assert three.grad() == 12.0


def lol():
  a = Value(2.0)
  b = Value(-3.0)
  e = a * b
  c = Value(10.0)
  f = Value(-2.0)
  d = c + e
  L = d * f
  L.backward()
  assert L.data == -8.0
  assert f.grad() == 4.0
  assert d.grad() == -2.0
  assert c.grad() == -2.0
  assert e.grad() == -2.0
  assert b.grad() == -4.0
  assert a.grad() == 6.0


lol()

# a = Value(-4.0)
# b = Value(2.0)
# c = a + b
# d = a * b + b**3.0
# c += c + 1.0
# c += 1.0 + c + (-a)
# d += d * 2.0 + (b + a).relu()
# d += 3.0 * d + (b - a).relu()
# e = c - d
# f = e**2.0
# g = f / 2.0
# g += 10.0 / f
# print(f'{g.data:.4f}')
# g.backward()
# print(f'{a.grad():.4f}')
# print(f'{b.grad():.4f}')
