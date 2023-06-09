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
	if dpll.lvl != 0 {
		t.Errorf("Expected lvl to be 0, got %d", dpll.lvl)
	}
}

func TestSat1(t *testing.T) {
	dpll := NewDPLL()
	dpll.AddClause(Clause{[]uint{0}})
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
	if dpll.lvl != 0 {
		t.Errorf("Expected lvl to be 0, got %d", dpll.lvl)
	}
}

func TestSat2(t *testing.T) {
	dpll := NewDPLL()
	dpll.AddClause(Clause{[]uint{1}})
	if dpll.Solve() != true {
		t.Errorf("Expected Solve() to return true, got false")
	}
	if !reflect.DeepEqual(dpll.trail, []uint{1}) {
		t.Errorf("Expected trail to be [1], got %v", dpll.trail)
	}
	if dpll.variables[1] != true {
		t.Errorf("Expected variables to be {1: true}, got %v", dpll.variables)
	}
	if dpll.max_key != 2 {
		t.Errorf("Expected max_key to be 2, got %d", dpll.max_key)
	}
	if dpll.lvl != 0 {
		t.Errorf("Expected lvl to be 0, got %d", dpll.lvl)
	}
}

func TestUnsat1(t *testing.T) {
	dpll := NewDPLL()
	dpll.AddClause(Clause{[]uint{0}})
	dpll.AddClause(Clause{[]uint{1}})
	if dpll.Solve() != false {
		t.Errorf("Expected Solve() to return false, got true")
	}
}

func TestSat3(t *testing.T) {
	dpll := NewDPLL()
	dpll.AddClause(Clause{[]uint{0}})
	dpll.AddClause(Clause{[]uint{2}})
	if dpll.Solve() != true {
		t.Errorf("Expected Solve() to return true, got false")
	}
}

func TestSat4(t *testing.T) {
	dpll := NewDPLL()
	dpll.AddClause(Clause{[]uint{0, 2}})
	dpll.AddClause(Clause{[]uint{1, 2}})
	if dpll.Solve() != true {
		t.Errorf("Expected Solve() to return true, got false")
	}
	if !reflect.DeepEqual(dpll.trail, []uint{0, 2}) {
		t.Errorf("Expected trail to be [0 2], got %v", dpll.trail)
	}
}

func TestUnsat2(t *testing.T) {
	dpll := NewDPLL()
	dpll.AddClause(Clause{[]uint{0, 2}})
	dpll.AddClause(Clause{[]uint{1, 2}})
	dpll.AddClause(Clause{[]uint{3}})
	if dpll.Solve() != false {
		t.Errorf("Expected Solve() to return false, got true")
	}
}

func TestCycleOfImplications(t *testing.T) {
	dpll := NewDPLL()
	for i := uint(1); i < 8; i++ {
		lit := i * 2
		dpll.AddClause(Clause{[]uint{lit - 1, lit}})
	}
	if dpll.Solve() != true {
		t.Errorf("Expected Solve() to return true, got false")
	}
	if !reflect.DeepEqual(dpll.trail, []uint{0, 2, 4, 6, 8, 10, 12, 14}) {
		t.Errorf("Expected trail to be [0 2 4 6 8 10 12 14], got %v", dpll.trail)
	}
	if dpll.lvl != 8 {
		t.Errorf("Expected lvl to be 8, got %d", dpll.lvl)
	}
}
