package binary_trees

import (
	"reflect"
	"testing"
)

func TestBSTTraversals(t *testing.T) {
	// First, let's create our test cases in a way that makes the expected results clear
	testCases := []struct {
		name           string
		insertSequence []int
		wantInOrder    []int
		wantPreOrder   []int
		wantPostOrder  []int
	}{
		{
			name:           "Basic tree with duplicates",
			insertSequence: []int{5, 3, 7, 5, 3, 7},
			wantInOrder:    []int{3, 3, 5, 5, 7, 7},
			wantPreOrder:   []int{5, 3, 3, 5, 7, 7},
			wantPostOrder:  []int{3, 5, 3, 7, 7, 5},
		},
	}

	// Now let's run each test case
	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			// Create a new tree and insert all values
			tree := NewBST(tc.insertSequence[0])
			for _, val := range tc.insertSequence[1:] {
				tree.Insert(val)
			}

			// Test in-order traversal
			gotInOrder := tree.InOrderTraversal()
			if !reflect.DeepEqual(gotInOrder, tc.wantInOrder) {
				t.Errorf("InOrderTraversal() = %v, want %v",
					gotInOrder, tc.wantInOrder)
			}

			// Test pre-order traversal
			gotPreOrder := tree.PreOrderTraversal()
			if !reflect.DeepEqual(gotPreOrder, tc.wantPreOrder) {
				t.Errorf("PreOrderTraversal() = %v, want %v",
					gotPreOrder, tc.wantPreOrder)
			}

			// Test post-order traversal
			gotPostOrder := tree.PostOrderTraversal()
			if !reflect.DeepEqual(gotPostOrder, tc.wantPostOrder) {
				t.Errorf("PostOrderTraversal() = %v, want %v",
					gotPostOrder, tc.wantPostOrder)
			}
		})
	}
}
