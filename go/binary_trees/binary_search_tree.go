package binary_trees

type Ordered interface {
	~int | ~int8 | ~int16 | ~int32 | ~int64 |
		~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64 | ~uintptr |
		~float32 | ~float64 | ~string
}

type Node[T Ordered] struct {
	value T
	left  *Node[T]
	right *Node[T]
}

func NewBST[T Ordered](value T) *Node[T] {
	return &Node[T]{
		value: value,
		left:  nil,
		right: nil,
	}
}

func (n *Node[T]) Insert(value T) {
	if n == nil {
		*n = Node[T]{value: value, left: nil, right: nil}
		return
	}

	if value <= n.value {
		if n.left == nil {
			n.left = NewBST(value)
		} else {
			n.left.Insert(value)
		}
	} else {
		if n.right == nil {
			n.right = NewBST(value)
		} else {
			n.right.Insert(value)
		}
	}
}

func (n *Node[T]) InOrderTraversal() []T {
	var result []T
	if n == nil {
		return result
	}

	result = append(result, n.left.InOrderTraversal()...)
	result = append(result, n.value)
	result = append(result, n.right.InOrderTraversal()...)

	return result
}

func (n *Node[T]) PreOrderTraversal() []T {
	var result []T
	if n == nil {
		return result
	}

	result = append(result, n.value)
	result = append(result, n.left.PreOrderTraversal()...)
	result = append(result, n.right.PreOrderTraversal()...)

	return result
}

func (n *Node[T]) PostOrderTraversal() []T {
	var result []T
	if n == nil {
		return result
	}

	result = append(result, n.left.PostOrderTraversal()...)
	result = append(result, n.right.PostOrderTraversal()...)
	result = append(result, n.value)

	return result
}
