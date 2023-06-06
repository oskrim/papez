// This is an implementation of the Davis-Putnam-Logemann-Loveland (DPLL) algorithm.

package main

import (
	"os"
	"fmt"
)

type Clause struct {
	// Literals are represented by positive integers
	// Odd integers are negated literals
	// Even integers are positive literals
	literals []uint
}

type DPLL struct {
	// Set of boolean clauses
	clauses []Clause
	// Set of variables
	variables map[uint]bool
	// Set of assignments
	trail []uint
	// max_key is the maximum key in variables
	max_key uint
}

func main() {
	dpll := NewDPLL()
	dpll.clauses = make([]Clause, 0)
}

func NewDPLL() *DPLL {
	return &DPLL{}
}

func (dpll *DPLL) Solve() bool {
	dpll.RegisterVariables()
	return dpll.SolveInternal()
}

func (dpll *DPLL) RegisterVariables() {
	dpll.variables = make(map[uint]bool)
	dpll.max_key = 0
	for _, clause := range dpll.clauses {
		for _, literal := range clause.literals {
			dpll.variables[literal] = false
			if literal > dpll.max_key {
				dpll.max_key = literal
			}
		}
	}
	dpll.max_key -= (dpll.max_key & 1)
	dpll.max_key += 2
}

func (dpll *DPLL) SolveInternal() bool {
	if dpll.AllClausesSatisfied() {
		debug("SAT with", dpll.trail)
		return true
	}

	// Try to assign a variable to true
	for pos_lit := uint(0); pos_lit < dpll.max_key; pos_lit += 2 {
		pos_lit_value := dpll.variables[pos_lit]

		neg_lit := pos_lit + 1
		neg_lit_value := dpll.variables[neg_lit]

		// if pos_lit or its negation is already assigned to true, then skip
		if pos_lit_value || neg_lit_value {
			continue
		}

		// Assign pos_lit to true
		dpll.Push(pos_lit)

		// Recurse
		if dpll.SolveInternal() {
			return true
		}

		// Try the negation of pos_lit
		dpll.Push(neg_lit)

		// Recurse
		if dpll.SolveInternal() {
			return true
		}
	}

	dpll.Pop()

	return false
}

func (dpll *DPLL) Push(variable uint) {
	dpll.variables[variable] = true
	dpll.trail = append(dpll.trail, variable)
}

func (dpll *DPLL) Pop() {
	if len(dpll.trail) == 0 {
		return
	}
	dpll.variables[dpll.trail[len(dpll.trail)-1]] = false
	dpll.trail = dpll.trail[:len(dpll.trail)-1]
}

func (dpll *DPLL) AllClausesSatisfied() bool {
	for _, clause := range dpll.clauses {
		if !dpll.ClauseSatisfied(clause) {
			return false
		}
	}
	return true
}

func (dpll *DPLL) ClauseSatisfied(clause Clause) bool {
	for _, literal := range clause.literals {
		if dpll.variables[literal] {
			return true
		}
	}
	return false
}

func debug(args ...interface{}) {
	if os.Getenv("DEBUG") == "1" {
		fmt.Println("DEBUG:", args)
	}
}
