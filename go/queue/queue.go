package queue

type Node[T any] struct {
	Value T
	Next  *Node[T]
}

type Queue[T any] struct {
	head   *Node[T]
	tail   *Node[T]
	length int
}

func NewQueue[T any]() *Queue[T] {
	return &Queue[T]{}
}

func (q *Queue[T]) Enqueue(value T) {
	node := Node[T]{Value: value}
	q.length++
	if q.tail == nil {
		q.head = &node
		q.tail = &node
		return
	}

	q.tail.Next = &node
	q.tail = &node
}

func (q *Queue[T]) Deque() *T {
	if q.head == nil {
		println("Cannot deque; Queue is empty")
		return nil
	}
	q.length--

	head := q.head
	q.head = q.head.Next
	head.Next = nil

	if q.length == 0 {
		q.tail = nil
	}

	return &head.Value
}

func (q *Queue[T]) Length() int {
	return q.length
}

func (q *Queue[T]) Peek() *T {
	if q.head == nil {
		println("Cannot peek; Queue is empty")
		return nil
	}
	return &q.head.Value
}
