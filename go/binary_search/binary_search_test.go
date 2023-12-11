package main

import "testing"

func TestLinearSearch(t *testing.T) {
	haystack := []int{1, 2, 3, 4, 5}
	needle := 3
	want := true
	got := binarySearch(haystack, needle)
	if got != want {
		t.Errorf("binarySearch() = %v, want %v", got, want)
	}
}
