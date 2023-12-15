package stack

import (
	"testing"
)

func TestStack(t *testing.T) {
	s := NewStack[int]()
	if s.Length() != 0 {
		t.Errorf("Expected length to be 0, got %d", s.Length())
	}
	s.Push(1)
	s.Push(2)
	s.Push(3)
	if s.Length() != 3 {
		t.Errorf("Expected length to be 3, got %d", s.Length())
	}
	if *s.Peek() != 3 {
		t.Errorf("Expected top of stack to be 3, got %d", *s.Peek())
	}
	if *s.Pop() != 3 {
		t.Errorf("Expected top of stack to be 3, got %d", *s.Pop())
	}
	if *s.Pop() != 2 {
		t.Errorf("Expected top of stack to be 2, got %d", *s.Pop())
	}
	if *s.Pop() != 1 {
		t.Errorf("Expected top of stack to be 1, got %d", *s.Pop())
	}
	if s.Length() != 0 {
		t.Errorf("Expected length to be 0, got %d", s.Length())
	}
	if s.Pop() != nil {
		t.Errorf("Expected top of stack to be nil, got %d", *s.Pop())
	}
}
