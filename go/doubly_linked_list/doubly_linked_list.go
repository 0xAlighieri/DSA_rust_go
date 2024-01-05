package doubly_linked_list

type EqualsFunc[T any] func(a, b T) bool

type Node[T any] struct {
	value T
	prev  *Node[T]
	next  *Node[T]
}

type DoublyLinkedList[T any] struct {
	head   *Node[T]
	tail   *Node[T]
	length int
}

func NewDoublyLinkedList[T any]() *DoublyLinkedList[T] {
	return &DoublyLinkedList[T]{}
}

// prepend
func (l *DoublyLinkedList[T]) Prepend(value T) {
	node := &Node[T]{value: value}
	l.length++

	if l.head == nil {
		l.head = node
		l.tail = node
		return
	}
	node.next = l.head
	l.head.prev = node
	l.head = node
}

// insertAt
func (l *DoublyLinkedList[T]) InsertAt(item T, index int) {
	if index > l.length {
		// throw an error
		error := "Index out of range"
		panic(error)
	}
	if index == l.length {
		l.Append(item)
		return

	} else if index == 0 {
		l.Prepend(item)
		return
	}

	l.length++
	curr := l.getAt(index)
	node := &Node[T]{value: item}

	node.next = curr
	node.prev = curr.prev
	curr.prev = node
	if node.prev != nil {
		node.prev.next = node
	}
}

// append
func (l *DoublyLinkedList[T]) Append(item T) {
	l.length++
	node := &Node[T]{value: item}
	if l.tail == nil {
		l.head = node
		l.tail = node
		return
	}
	node.prev = l.tail
	l.tail.next = node
	l.tail = node
}

// remove
// func (l *DoublyLinkedList[T]) Remove(item T) *T {
// 	curr := l.head
// 	for i := 0; curr != nil && i < l.length; i++ {
// 		// print the value of the current node
// 		if &curr.value == &item {
// 			break
// 		}
// 		curr = curr.next
// 	}
// 	if curr == nil {
// 		return nil
// 	}
// 	return l.removeNode(curr)
// }

func (l *DoublyLinkedList[T]) Remove(item T, equals EqualsFunc[T]) *T {
	curr := l.head
	for curr != nil {
		if equals(curr.value, item) {
			return l.removeNode(curr)
		}
		curr = curr.next
	}
	return nil
}

// get
func (l *DoublyLinkedList[T]) Get(index int) T {
	if index > l.length {
		// throw an error
		error := "Index out of range"
		panic(error)
	}
	return l.getAt(index).value
}

// removeAt
func (l *DoublyLinkedList[T]) RemoveAt(index int) *T {
	node := l.getAt(index)
	if node == nil {
		return nil
	}
	return l.removeNode(node)
}

// private: removeNode
func (l *DoublyLinkedList[T]) removeNode(node *Node[T]) *T {
	l.length--

	if l.length == 0 {
		out := l.head
		l.head = nil
		l.tail = nil
		return &out.value
	}

	if node.prev != nil {
		node.prev.next = node.next
	}

	if node.next != nil {
		node.next.prev = node.prev
	}

	if node == l.head {
		l.head = node.next
	}

	if node == l.tail {
		l.tail = node.prev
	}
	node.prev = nil
	node.next = nil
	return &node.value
}

// private: getAt
func (l *DoublyLinkedList[T]) getAt(index int) *Node[T] {
	curr := l.head
	for i := 0; curr != nil && i < index; i++ {
		curr = curr.next
	}
	return curr
}
