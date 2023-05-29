package main

import (
	"fmt"
)

type Literal struct {
	// the variable
	variable string
	// the value of the variable
	value bool
}

type Clause struct {
	// set of literals
	literals []Literal
}

type DPLL struct {
	// set of boolean clauses
	clauses []Clause
	// set of variables, hash table from string to bool
	variables map[string]bool
}

func NewDPLL() *DPLL {
	return &DPLL{}
}

func (d *DPLL) removeClauses(literal Literal) {
	result := d.clauses[:]
	i := 0

	// Remove all clauses containing the literal.
	for _, clause := range d.clauses {
		for _, l := range clause.literals {
			if l == literal {
				continue
			}
		}
		result[i] = clause
		i++
	}
	d.clauses = result[:i]
}

func (d *DPLL) removeNegations(literal Literal) {
	for k, clause := range d.clauses {
		result := clause.literals[:]
		i := 0
		for _, l := range clause.literals {
			if l.variable == literal.variable && l.value != literal.value {
				continue
			}
			result[i] = l
			i++
		}
		d.clauses[k].literals = result[:i]
	}
}

// This is an implementation of the Davis-Putnam-Logemann-Loveland (DPLL) algorithm.
func (d *DPLL) Solve() bool {
	// If the set of clauses is empty, return true.
	if len(d.clauses) == 0 {
		return true
	}

	// loop enough times
	for i := 0; i < 10; i++ {
		for _, clause := range d.clauses {
			// If any clause is empty, return false.
			if len(clause.literals) == 0 {
				return false
			}

			// Unit clause
			if len(clause.literals) == 1 {
				// Assign the literal in the unit clause to true.
				d.variables[clause.literals[0].variable] = clause.literals[0].value
				// Remove all clauses containing the literal.
				d.removeClauses(clause.literals[0])
				// Remove all instances of the negation of the literal from all clauses.
				d.removeNegations(clause.literals[0])
			}
		}
	}
	return false
}

func main() {
	dpll := NewDPLL()

	dpll.variables = make(map[string]bool)

	dpll.clauses = []Clause{
		Clause{
			literals: []Literal{
				Literal{
					variable: "a",
					value: true,
				},
			},
		},
		Clause{
			literals: []Literal{
				Literal{
					variable: "a",
					value: true,
				},
				Literal{
					variable: "b",
					value: true,
				},
			},
		},
		Clause{
			literals: []Literal{
				Literal{
					variable: "a",
					value: true,
				},
				Literal{
					variable: "b",
					value: false,
				},
			},
		},
		Clause{
			literals: []Literal{
				Literal{
					variable: "a",
					value: false,
				},
				Literal{
					variable: "b",
					value: true,
				},
			},
		},
	}

	dpll.Solve()

	fmt.Println("Clauses:")
	for _, clause := range dpll.clauses {
		fmt.Printf("%v\n", clause)
	}

	fmt.Println("\nSolution:")
	for k, v := range dpll.variables {
		fmt.Printf("%s=%t\n", k, v)
	}

	// Clauses:
	// {[{a true}]}
	// {[{a true} {b true}]}
	// {[{a true}]}
	// {[{b true}]}

	// Solution:
	// a=true
	// b=true
}
