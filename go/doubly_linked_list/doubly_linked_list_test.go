package doubly_linked_list

import (
	"testing"
)

func TestDoublyLinkedList(t *testing.T) {
	list := NewDoublyLinkedList[int]()

	list.Append(5)
	list.Append(7)
	list.Append(9)

	if val := list.Get(2); val != 9 {
		t.Errorf("Expected 9, got %d", val)
	}
	if val := list.RemoveAt(1); *val != 7 {
		t.Errorf("Expected 7, got %d", *val)
	}
	if list.length != 2 {
		t.Errorf("Expected length 2, got %d", list.length)
	}

	list.Append(11)
	if val := list.RemoveAt(1); *val != 9 {
		t.Errorf("Expected 9, got %d", *val)
	}
	if val := list.Remove(9); val != nil {
		t.Errorf("Expected nil, got %d", *val)
	}
	if val := list.RemoveAt(0); *val != 5 {
		t.Errorf("Expected 5, got %d", *val)
	}
	if val := list.RemoveAt(0); *val != 11 {
		t.Errorf("Expected 11, got %d", *val)
	}
	if list.length != 0 {
		t.Errorf("Expected length 0, got %d", list.length)
	}

	list.Prepend(5)
	list.Prepend(7)
	list.Prepend(9)

	if val := list.Get(2); val != 5 {
		t.Errorf("Expected 5, got %d", val)
	}
	if val := list.Get(0); val != 9 {
		t.Errorf("Expected 9, got %d", val)
	}
	val := list.Remove(9)
	if val == nil || *val != 9 {
		if val == nil {
			t.Errorf("Expected 9, got nil")
		} else {
			t.Errorf("Expected 9, got %d", *val)
		}
	}
	if list.length != 2 {
		t.Errorf("Expected length 2, got %d", list.length)
	}
	if val := list.Get(0); val != 7 {
		t.Errorf("Expected 7, got %d", val)
	}
}
