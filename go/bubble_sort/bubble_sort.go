package bubble_sort

func BubbleSort(list []int) []int {
	for i := 0; i < len(list); i++ {
		for j := 0; j < len(list)-1-i; j++ {
			if list[j] > list[j+1] {
				list[j], list[j+1] = list[j+1], list[j]
			}
		}
	}
	return list
}

func main() {
	list := []int{1, 5, 2, 4, 3}
	BubbleSort(list)
}
