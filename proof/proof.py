from z3 import *

a = Bool('a')
b = Bool('b')
lemma = Implies(a, b)
premises = And(a, lemma)
conjecture = Implies(premises, b)
prove(conjecture)
