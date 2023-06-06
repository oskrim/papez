package main

import (
	"testing"
)

func TestNoClauses(t *testing.T) {
	dpll := NewDPLL()
	if len(dpll.clauses) != 0 {
		t.Errorf("Expected length of clauses to be 0, got %d", len(dpll.clauses))
	}
	if dpll.Solve() != true {
		t.Errorf("Expected Solve() to return true, got false")
	}
}

func TestUnitSat1(t *testing.T) {
	dpll := NewDPLL()
	dpll.clauses = append(dpll.clauses, Clause{[]uint{0}})
	if dpll.Solve() != true {
		t.Errorf("Expected Solve() to return true, got false")
	}
}
