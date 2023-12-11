package main

import (
	"fmt"
)

func binarySearch(haystack []int, needle int) bool {
	lo := 0
	hi := len(haystack)

	done := lo < hi
	for done {
		m := int(float64(lo + (hi-lo)/2))
		v := haystack[m]

		if v == needle {
			return true
		} else if v > needle {
			hi = m
		} else {
			lo = m + 1
		}

	}
	return false
}

func main() {
	haystack := []int{1, 2, 3, 4, 5}
	needle := 3
	fmt.Println("Found:", binarySearch(haystack, needle))
}
