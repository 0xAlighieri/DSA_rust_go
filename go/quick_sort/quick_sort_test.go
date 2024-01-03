package quick_sort

import (
	"reflect"
	"testing"
)

func TestQuickSort(t *testing.T) {
	arr := []int{9, 3, 7, 4, 69, 420, 42}
	quickSort(arr)

	expected := []int{3, 4, 7, 9, 42, 69, 420}
	if !reflect.DeepEqual(arr, expected) {
		t.Errorf("quickSort failed: got %v, want %v", arr, expected)
	}
}
