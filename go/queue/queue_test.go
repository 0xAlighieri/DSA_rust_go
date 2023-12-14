package queue

import (
	"testing"
)

func TestQueue(t *testing.T) {
	q := NewQueue[int]()
	q.Enqueue(1)
	q.Enqueue(2)
	q.Enqueue(3)
	q.Enqueue(4)
	q.Enqueue(5)
	q.Enqueue(6)
	q.Enqueue(7)
	q.Enqueue(8)
	q.Enqueue(9)
	q.Enqueue(10)

	if q.Length() != 10 {
		t.Errorf("Expected length of 10, got %d", q.Length())
	}

	if *q.Peek() != 1 {
		t.Errorf("Expected peek of 1, got %d", *q.Peek())
	}

	if *q.Deque() != 1 {
		t.Errorf("Expected deque of 1, got %d", *q.Deque())
	}

	if *q.Deque() != 2 {
		t.Errorf("Expected deque of 2, got %d", *q.Deque())
	}

	if *q.Deque() != 3 {
		t.Errorf("Expected deque of 3, got %d", *q.Deque())
	}

	if *q.Deque() != 4 {
		t.Errorf("Expected deque of 4, got %d", *q.Deque())
	}

	if *q.Deque() != 5 {
		t.Errorf("Expected deque of 5, got %d", *q.Deque())
	}

	if *q.Deque() != 6 {
		t.Errorf("Expected deque of 6, got %d", *q.Deque())
	}

	if *q.Deque() != 7 {
		t.Errorf("Expected deque of 7, got %d", *q.Deque())
	}

	if *q.Deque() != 8 {
		t.Errorf("Expected deque of 8, got %d", *q.Deque())
	}

	if *q.Deque() != 9 {
		t.Errorf("Expected deque of 9, got %d", *q.Deque())
	}

	if *q.Deque() != 10 {
		t.Errorf("Expected deque of 10, got %d", *q.Deque())
	}

	if q.Length() != 0 {
		t.Errorf("Expected length of 0, got %d", q.Length())
	}

	if q.Deque() != nil {
		t.Errorf("Expected nil deque, got %d", *q.Deque())
	}

	if q.Peek() != nil {
		t.Errorf("Expected nil peek, got %d", *q.Peek())
	}
}
