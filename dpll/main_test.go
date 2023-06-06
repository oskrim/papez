package main

import (
	"testing"
)

func TestNewDPLL(t *testing.T) {
	dpll := NewDPLL()
	if len(dpll.clauses) != 0 {
		t.Errorf("Expected length of clauses to be 0, got %d", len(dpll.clauses))
	}
}

func TestUnitSat1(t *testing.T) {
	dpll := NewDPLL()
	dpll.clauses = append(dpll.clauses, Clause{[]uint{0}})
	if dpll.Solve() != true {
		t.Errorf("Expected Solve() to return true, got false")
	}
}
