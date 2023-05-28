import math
import numpy as np

class Value(object):
  def __init__(self, data):
    self.data = data
    self._grad = None

  def __repr__(self):
    return "Value(%s)" % str(self.data)

  def __add__(self, other):
    out = Value(self.data + other.data)
    out._grad = self.grad() + other.grad()
    return out

  def __mul__(self, other):
    out = Value(self.data * other.data)
    out._grad = self.data * other.grad() + other.data * self.grad()
    return out

  def grad(self):
    if self._grad is None:
      self._grad = 1.0
    return self._grad


assert (Value(5.0) + Value(4.0)).data == 9.0
assert (Value(5.0) * Value(4.0)).data == 20.0

x = Value(4.0)
assert x.grad() == 1.0

y = x * x
assert y.grad() == 8.0

z = x * x * x
assert z.grad() == 48.0

a = x + y
assert a.grad() == 9.0

b = a + z
assert b.grad() == 57.0

c = a * b
assert c.data == 1680
assert c.grad() == 1896.0


a = Value(-4.0)
b = Value(2.0)
c = a + b
d = a * b + b**3
c += c + 1
c += 1 + c + (-a)
d += d * 2 + (b + a).relu()
d += 3 * d + (b - a).relu()
e = c - d
f = e**2
g = f / 2.0
g += 10.0 / f
print(f'{g.data:.4f}') # prints 24.7041, the outcome of this forward pass
g.backward()
print(f'{a.grad:.4f}') # prints 138.8338, i.e. the numerical value of dg/da
print(f'{b.grad:.4f}') # prints 645.5773, i.e. the numerical value of dg/db
