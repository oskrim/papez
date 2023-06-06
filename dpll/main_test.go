package main

import (
	"testing"
	"reflect"
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

func TestSat1(t *testing.T) {
	dpll := NewDPLL()
	dpll.clauses = append(dpll.clauses, Clause{[]uint{0}})
	if dpll.Solve() != true {
		t.Errorf("Expected Solve() to return true, got false")
	}
	if !reflect.DeepEqual(dpll.trail, []uint{0}) {
		t.Errorf("Expected trail to be [0], got %v", dpll.trail)
	}
	if dpll.variables[0] != true {
		t.Errorf("Expected variables to be {0: true}, got %v", dpll.variables)
	}
	if dpll.max_key != 2 {
		t.Errorf("Expected max_key to be 2, got %d", dpll.max_key)
	}
}

func TestSat2(t *testing.T) {
	dpll := NewDPLL()
	dpll.clauses = append(dpll.clauses, Clause{[]uint{1}})
	if dpll.Solve() != true {
		t.Errorf("Expected Solve() to return true, got false")
	}
	if !reflect.DeepEqual(dpll.trail, []uint{1}) {
		t.Errorf("Expected trail to be [0], got %v", dpll.trail)
	}
	if dpll.variables[1] != true {
		t.Errorf("Expected variables to be {1: true}, got %v", dpll.variables)
	}
	if dpll.max_key != 2 {
		t.Errorf("Expected max_key to be 2, got %d", dpll.max_key)
	}
}

func TestUnsat1(t *testing.T) {
	dpll := NewDPLL()
	dpll.clauses = append(dpll.clauses, Clause{[]uint{0}})
	dpll.clauses = append(dpll.clauses, Clause{[]uint{1}})
	if dpll.Solve() != false {
		t.Errorf("Expected Solve() to return false, got true")
	}
}
