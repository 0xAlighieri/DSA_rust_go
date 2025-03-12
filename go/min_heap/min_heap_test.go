package min_heap

import (
	"testing"
)

func TestMinHeap(t *testing.T) {
	heap := NewMinHeap[int]()

	if heap.length != 0 {
		t.Errorf("Expected heap length to be 0, got %d", heap.length)
	}

	heap.Insert(5)
	heap.Insert(3)
	heap.Insert(69)
	heap.Insert(420)
	heap.Insert(4)
	heap.Insert(1)
	heap.Insert(8)
	heap.Insert(7)

	if heap.length != 8 {
		t.Errorf("Expected heap length to be 8, got %d", heap.length)
	}

	// Test Delete operations
	val, ok := heap.Delete()
	if !ok || val != 1 {
		t.Errorf("Expected 1, got %d (ok: %v)", val, ok)
	}

	val, ok = heap.Delete()
	if !ok || val != 3 {
		t.Errorf("Expected 3, got %d (ok: %v)", val, ok)
	}

	val, ok = heap.Delete()
	if !ok || val != 4 {
		t.Errorf("Expected 4, got %d (ok: %v)", val, ok)
	}

	val, ok = heap.Delete()
	if !ok || val != 5 {
		t.Errorf("Expected 5, got %d (ok: %v)", val, ok)
	}

	if heap.length != 4 {
		t.Errorf("Expected heap length to be 4, got %d", heap.length)
	}

	val, ok = heap.Delete()
	if !ok || val != 7 {
		t.Errorf("Expected 7, got %d (ok: %v)", val, ok)
	}

	val, ok = heap.Delete()
	if !ok || val != 8 {
		t.Errorf("Expected 8, got %d (ok: %v)", val, ok)
	}

	val, ok = heap.Delete()
	if !ok || val != 69 {
		t.Errorf("Expected 69, got %d (ok: %v)", val, ok)
	}

	val, ok = heap.Delete()
	if !ok || val != 420 {
		t.Errorf("Expected 420, got %d (ok: %v)", val, ok)
	}

	if heap.length != 0 {
		t.Errorf("Expected heap length to be 0, got %d", heap.length)
	}

	// Test Delete on empty heap
	val, ok = heap.Delete()
	if ok {
		t.Errorf("Expected Delete on empty heap to return false, got true with value %d", val)
	}
}
