package binary_trees

// fdsaf
type BTNode[T comparable] struct {
	value T
	left  *BTNode[T]
	right *BTNode[T]
}

func BTPreOrder[T comparable](curr *BTNode[T], path []T) []T {
	if curr == nil {
		return path
	}

	path = append(path, curr.value)
	path = BTPreOrder(curr.left, path)
	path = BTPreOrder(curr.right, path)

	return path
}

func BTInOrder[T comparable](curr *BTNode[T], path []T) []T {
	if curr == nil {
		return path
	}

	path = BTInOrder(curr.left, path)
	path = append(path, curr.value)
	path = BTInOrder(curr.right, path)

	return path
}

func BTPostOrder[T comparable](curr *BTNode[T], path []T) []T {
	if curr == nil {
		return path
	}

	path = BTPostOrder(curr.left, path)
	path = BTPostOrder(curr.right, path)
	path = append(path, curr.value)

	return path
}
