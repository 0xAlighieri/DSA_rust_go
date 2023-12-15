package stack

import "math"

type Node[T any] struct {
	value T
	prev  *Node[T]
}

type Stack[T any] struct {
	head   *Node[T]
	length int
}

func NewStack[T any]() *Stack[T] {
	return &Stack[T]{}
}

func (s *Stack[T]) Push(value T) {
	node := &Node[T]{value: value, prev: nil}
	s.length++
	if s.head == nil {
		s.head = node
		return
	}
	node.prev = s.head
	s.head = node
}

func (s *Stack[T]) Pop() *T {
	if s.length == 0 {
		return nil
	}
	s.length = int(math.Max(0, float64(s.length-1)))
	if s.length == 0 {
		head := s.head
		s.head = nil
		return &head.value
	}
	head := s.head
	s.head = head.prev
	return &head.value
}

func (s *Stack[T]) Length() int {
	return s.length
}

func (s *Stack[T]) Peek() *T {
	return &s.head.value
}
