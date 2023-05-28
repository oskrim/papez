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
    self._grad = (self, 1.0)
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
