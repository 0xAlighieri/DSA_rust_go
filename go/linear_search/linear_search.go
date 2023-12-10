package main

import "fmt"

func linearSearch(haystack []int, needle int) bool {
	for i := 0; i < len(haystack); i++ {
		if haystack[i] == needle {
			return true
		}
	}
	return false
}

func main() {
	haystack := []int{1, 2, 3, 4, 5}
	needle := 3
	fmt.Println("Found:", linearSearch(haystack, needle))
}
