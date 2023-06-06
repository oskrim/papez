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
	for _, clause := range dpll.clauses {
		for _, literal := range clause.literals {
			dpll.variables[literal] = false
		}
	}
}

func (dpll *DPLL) SolveInternal() bool {
	if dpll.AllClausesSatisfied() {
		return true
	}

	// Try to assign a variable to true
	for pos_lit := uint(0); true; pos_lit += 2 {
		// if pos_lit does not exist in variables, then we have tried all variables
		if pos_lit_value, ok := dpll.variables[pos_lit]; !ok {
			break
		} else {
			neg_lit := pos_lit + 1
			neg_lit_value := dpll.variables[neg_lit]

			// if pos_lit or its negation is already assigned to true, then skip
			if pos_lit_value || neg_lit_value {
				continue
			}

			// assign pos_lit to true
			dpll.variables[pos_lit] = true
			dpll.trail = append(dpll.trail, pos_lit)
			debug("Assigned", pos_lit, "to true")

			// Recurse
			if dpll.SolveInternal() {
				return true
			}

			// Try the negation of pos_lit
			dpll.variables[pos_lit] = false
			dpll.variables[neg_lit] = true
			dpll.trail = append(dpll.trail, neg_lit)
			debug("Assigned", neg_lit, "to true")

			// Recurse
			if dpll.SolveInternal() {
				return true
			}

			// Neither variable can be assigned to true, backtrack
			dpll.variables[neg_lit] = false
		}
	}

	return false
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
