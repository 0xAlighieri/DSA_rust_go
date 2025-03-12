package min_heap

type Ordered interface {
	~int | ~int8 | ~int16 | ~int32 | ~int64 |
		~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64 | ~uintptr |
		~float32 | ~float64 | ~string
}

type MinHeap[T Ordered] struct {
	heap   []T
	length int
}

func NewMinHeap[T Ordered]() *MinHeap[T] {
	return &MinHeap[T]{
		heap:   make([]T, 0),
		length: 0,
	}
}

func (h *MinHeap[T]) parent(idx int) int {
	return (idx - 1) / 2
}

func (h *MinHeap[T]) leftChild(idx int) int {
	return 2*idx + 1
}

func (h *MinHeap[T]) rightChild(idx int) int {
	return 2*idx + 2
}

func (h *MinHeap[T]) heapifyDown(idx int) {
	lIdx := h.leftChild(idx)
	rIdx := h.rightChild(idx)

	if idx >= h.length || lIdx >= h.length {
		return
	}

	lV := h.heap[lIdx]
	rV := h.heap[rIdx]
	v := h.heap[idx]

	if lV > rV && v > rV {
		h.heap[idx] = rV
		h.heap[rIdx] = v
		h.heapifyDown(rIdx)
	} else if rV > lV && v > lV {
		h.heap[idx] = lV
		h.heap[lIdx] = v
		h.heapifyDown(lIdx)
	}
}

func (h *MinHeap[T]) heapifyUp(idx int) {
	if idx == 0 {
		return
	}

	p := h.parent(idx)
	parentV := h.heap[p]
	v := h.heap[idx]

	if parentV > v {
		h.heap[idx] = parentV
		h.heap[p] = v

		h.heapifyUp(p)
	}
}

func (h *MinHeap[T]) Insert(value T) {
	h.heap = append(h.heap, value)
	h.heapifyUp(h.length)
	h.length++
}

func (h *MinHeap[T]) Delete() (T, bool) {
	if h.length == 0 {
		var zeroValue T
		return zeroValue, false
	}

	out := h.heap[0]
	h.length--
	if h.length == 0 {
		h.heap = []T{}
		return out, true
	}

	h.heap[0] = h.heap[h.length]
	h.heapifyDown(0)
	return out, true
}
