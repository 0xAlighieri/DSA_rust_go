package bubble_sort

import (
	"reflect"
	"testing"
)

func TestBubbleSort(t *testing.T) {
	tests := []struct {
		name     string
		list     []int
		expected []int
	}{
		{
			name:     "Sorted List",
			list:     []int{1, 2, 3, 4, 5},
			expected: []int{1, 2, 3, 4, 5},
		},
		{
			name:     "Unsorted List",
			list:     []int{5, 3, 4, 1, 2},
			expected: []int{1, 2, 3, 4, 5},
		},
		{
			name:     "Duplicate Elements",
			list:     []int{3, 1, 2, 1, 3},
			expected: []int{1, 1, 2, 3, 3},
		},
		{
			name:     "Empty List",
			list:     []int{},
			expected: []int{},
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			actual := BubbleSort(tc.list)
			if !reflect.DeepEqual(actual, tc.expected) {
				t.Errorf("Test '%s' failed: expected %v, got %v", tc.name, tc.expected, actual)
			}
		})
	}
}
