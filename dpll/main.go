// This is an implementation of the Davis-Putnam-Logemann-Loveland (DPLL) algorithm.
// TODO: uint maps could be arrays?
// TODO: run against real sat benchmarks

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
	// the two literals that are being watched
	// are positioned at indices [0, 1] of `literals`
	// a watch literal can be true or unassigned
	// when either watch literal is falsified, then
	// a new watch needs to be found, if cannot be
	// found then the clause is either a unit clause
	// (other watch is undef) or a conflict clause
	// (both watches are false)
}

func (clause *Clause) IsUnit() bool {
	return len(clause.literals) == 3
}

type DPLL struct {
	// Set of boolean clauses
	clauses []Clause
	// Set of variables
	variables map[uint]bool
	// Set of assignments
	trail []uint
	// decision levels of trail
	dl []uint
	// decision level
	lvl uint
	// max_key is the maximum key in variables
	max_key uint
	// map literals -> clauses
	lit_to_clauses map[uint]([]uint)
}

func main() {
	fmt.Println("main")
}

func NewDPLL() *DPLL {
	dpll := DPLL{}
	dpll.clauses = make([]Clause, 0)
	dpll.lit_to_clauses = make(map[uint][]uint)
	dpll.lvl = 0
	return &dpll
}

func (dpll *DPLL) AddClause(clause Clause) {
	clause_len := uint(len(clause.literals))
	if clause_len == 0 {
		return
	}

	// build lit_to_clauses lookup table
	num_clauses := uint(len(dpll.clauses))
	for _, lit := range clause.literals {
		dpll.lit_to_clauses[lit] = append(dpll.lit_to_clauses[lit], num_clauses)
	}

	// prepend watch literals
	clause.literals = append([]uint{2, 2 + clause_len - 1}, clause.literals...)

	dpll.clauses = append(dpll.clauses, clause)
}

func (dpll *DPLL) Solve() bool {
	debug()
	dpll.RegisterVariables()
	dpll.InitialUnitPropagate()
	return dpll.SolveInternal()
}

func (dpll *DPLL) RegisterVariables() {
	dpll.variables = make(map[uint]bool)
	dpll.max_key = 0
	for literal, _ := range dpll.lit_to_clauses {
		dpll.variables[literal] = false
		if literal > dpll.max_key {
			dpll.max_key = literal
		}
	}
	dpll.max_key -= (dpll.max_key & 1)
	dpll.max_key += 2
}

func (dpll *DPLL) InitialUnitPropagate() {
	for _, clause := range dpll.clauses {
		if clause.IsUnit() {
			dpll.Push(clause.literals[2])
		}
	}
}

func (dpll *DPLL) SolveInternal() bool {
	if dpll.AllClausesSatisfied() {
		debug("SAT with", dpll.trail)
		return true
	}

	// Try to assign a variable to true
	// TODO: Invent a better decision heuristic (VSIDS?)
	for pos_lit := uint(0); pos_lit < dpll.max_key; pos_lit += 2 {
		pos_lit_value := dpll.variables[pos_lit]

		neg_lit := pos_lit + 1
		neg_lit_value := dpll.variables[neg_lit]

		// if pos_lit or its negation is already assigned to true, then skip
		if pos_lit_value || neg_lit_value {
			continue
		}

		// Assign pos_lit to true
		if !dpll.Decide(pos_lit) {
			if !dpll.Decide(neg_lit) {
				continue
			}
		}

		// Recurse
		if dpll.SolveInternal() {
			return true
		}
	}

	return false
}

func (dpll *DPLL) Decide(variable uint) bool {
	dpll.lvl += 1
	if !dpll.Push(variable) {
		dpll.Pop()
		return false
	}
	return true
}

func (dpll *DPLL) Push(variable uint) bool {
	debug("pushing", variable)

	if dpll.variables[variable ^ 1] == true {
		debug("conflict", variable)
		return false
	}

	dpll.trail = append(dpll.trail, variable)
	dpll.dl = append(dpll.dl, dpll.lvl)

	dpll.variables[variable] = true
	return dpll.UnitPropagate(variable)
}

func (dpll *DPLL) UnitPropagate(lit uint) bool {
	debug("propagate", lit)

	for _, idx := range dpll.lit_to_clauses[lit] {
		clause := dpll.clauses[idx]
		w0 := clause.literals[0]
		w1 := clause.literals[1]

		// which watch literal is it?
		w := 0
		if clause.literals[w0] == lit {
			w = 0
		} else if clause.literals[w1] == lit {
			w = 1
		} else {
			return true
		}

		for ji, lit := range clause.literals[2:] {
			j := uint(ji) // TODO: how to iterate in uint and avoid this cast?
			if j == w0 || j == w1 || dpll.variables[lit ^ 1] == true {
				continue
			}

			// found a new watch literal
			clause.literals[w] = j
			return true
		}

		// cant find a new watch literal
		other := clause.literals[w1]
		if w == 0 {
			other = clause.literals[w0]
		}
		// assert not false
		if dpll.variables[other ^ 1] == true {
			panic("UnitPropagate: other watch literal cannot be false")
		}
		if dpll.variables[other] == true {
			// already assigned, nothing left to do
			return true
		} else {
			if !dpll.Push(other) {
				return false
			}
		}
	}

	return true
}

func (dpll *DPLL) Pop() {
	if len(dpll.trail) == 0 {
		return
	}
	dpll.lvl -= 1
	for dpll.dl[len(dpll.trail)-1] != dpll.lvl {
		dpll.variables[dpll.trail[len(dpll.trail)-1]] = false
		dpll.trail = dpll.trail[:len(dpll.trail)-1]
		dpll.dl = dpll.dl[:len(dpll.dl)-1]
	}
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
	for _, literal := range clause.literals[2:] {
		if dpll.variables[literal] == true {
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
