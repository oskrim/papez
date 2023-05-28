import math
import numpy as np

class Value(object):
  def __init__(self, data, children=(), op=''):
    self.data = data
    self._grad = 0.0
    self._children = children
    self._op = op

  def __repr__(self):
    return "Value(%s)" % str(self.data)

  def __add__(self, other):
    out = Value(self.data + other.data, children=(self, other), op='+')
    return out

  def __mul__(self, other):
    out = Value(self.data * other.data, children=(self, other), op='*')
    return out

  def __pow__(self, other):
    out = Value(self.data ** other.data, children=(self, other), op='**')
    return out

  def __sub__(self, other):
    return self + (-other)

  def __neg__(self):
    return self * Value(-1.0)

  def __truediv__(self, other):
    return self * other**Value(-1.0)

  def relu(self):
    out = Value(np.maximum(self.data, 0), children=(self,), op='relu')
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
        self._children[0]._grad += self.grad() * self._children[1].data * (self._children[0].data ** (self._children[1].data - 1))
        if self._children[0].data == 0:
          self._children[1]._grad += 0.0
        else:
          self._children[1]._grad += self.grad() * (self._children[0].data ** self._children[1].data) * math.log(math.fabs(self._children[0].data))
      case 'relu':
        self._children[0]._grad += self.grad() * (self._children[0].data > 0)
      case _:
        return
    for child in self._children:
      child.backward()


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
  print('lol() passed')


lol()


# a = Value(-4.0)
# b = Value(2.0)
# c = a + b
# d = a * b + b**Value(3.0)
# c += c + Value(1.0)
# c += Value(1.0) + c + (-a)
# d += d * Value(2.0) + (b + a).relu()
# d += Value(3.0) * d + (b - a).relu()
# e = c - d
# f = e**Value(2.0)
# g = f / Value(2.0)
# g += Value(10.0) / f
# print(f'{g.data:.4f}')
# g.backward()
# print(f'{a.grad():.4f}')
# print(f'{b.grad():.4f}')
