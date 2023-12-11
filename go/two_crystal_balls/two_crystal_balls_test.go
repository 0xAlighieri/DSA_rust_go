package main

import (
	"testing"
)

func TestTwoCrystalBalls(t *testing.T) {
	// Test case 1: Test where the break is at the beginning
	breaks := []bool{true, true, true, true, true, true, true, true, true, true, true, true, true}
	expected := 0
	actual := twoCrystalBalls(breaks)
	if actual != expected {
		t.Errorf("Test Failed: expected %v, got %v", expected, actual)
	}

	// Test case 2: Test where the break is in the middle
	breaks = []bool{false, false, false, true, true, true, true, true, true, true}
	expected = 3
	actual = twoCrystalBalls(breaks)
	if actual != expected {
		t.Errorf("Test Failed: expected %v, got %v", expected, actual)
	}

	// Test case 3: Test where there is no break
	breaks = []bool{false, false, false, false, false, false, false, false, false, false}
	expected = -1
	actual = twoCrystalBalls(breaks)
	if actual != expected {
		t.Errorf("Test Failed: expected %v, got %v", expected, actual)
	}
}
