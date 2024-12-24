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

func TestBFSearch(t *testing.T) {
	// We'll structure our tests as subtests to make it clear what each test is checking
	t.Run("Search in empty tree", func(t *testing.T) {
		var tree *Node[int]
		if tree.BFSearch(5) {
			t.Error("Expected false for empty tree, got true")
		}
	})

	t.Run("Basic tree operations", func(t *testing.T) {
		// Create a tree with this structure:
		//       5
		//      / \
		//     3   7
		//    /   / \
		//   1   6   8
		tree := NewBST(5)
		tree.Insert(3)
		tree.Insert(7)
		tree.Insert(1)
		tree.Insert(6)
		tree.Insert(8)

		// Test cases to verify both positive and negative searches
		testCases := []struct {
			searchFor int
			expected  bool
			desc      string
		}{
			{5, true, "should find root"},
			{3, true, "should find left child"},
			{7, true, "should find right child"},
			{1, true, "should find deep left node"},
			{6, true, "should find deep right node"},
			{8, true, "should find rightmost node"},
			{4, false, "should not find non-existent value"},
			{9, false, "should not find value larger than all nodes"},
			{0, false, "should not find value smaller than all nodes"},
		}

		for _, tc := range testCases {
			t.Run(tc.desc, func(t *testing.T) {
				result := tree.BFSearch(tc.searchFor)
				if result != tc.expected {
					t.Errorf("BFSearch(%d) = %v, expected %v",
						tc.searchFor, result, tc.expected)
				}
			})
		}
	})

	t.Run("Tree with duplicates", func(t *testing.T) {
		// Create a tree with duplicate values
		tree := NewBST(5)
		tree.Insert(5) // Duplicate root
		tree.Insert(3)
		tree.Insert(3) // Duplicate value

		if !tree.BFSearch(3) {
			t.Error("Failed to find value that exists multiple times")
		}
	})

	t.Run("Single node tree", func(t *testing.T) {
		tree := NewBST(1)

		if !tree.BFSearch(1) {
			t.Error("Failed to find value in single node tree")
		}
		if tree.BFSearch(2) {
			t.Error("Found non-existent value in single node tree")
		}
	})
}
