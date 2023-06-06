// This is an implementation of the Davis-Putnam-Logemann-Loveland (DPLL) algorithm.

package main

import (
	// "fmt"
)

type Clause struct {
	// set of literals
	literals []uint
}

type DPLL struct {
	// set of boolean clauses
	clauses []Clause
}

func NewDPLL() *DPLL {
	return &DPLL{}
}

func (dpll *DPLL) Solve() bool {
	return false
}

func main() {
	dpll := NewDPLL()
	dpll.clauses = append(dpll.clauses, Clause{[]uint{1, 2, 3}})
}
