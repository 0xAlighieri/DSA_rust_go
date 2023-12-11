package main

import (
	"math"
)

func twoCrystalBalls(breaks []bool) int {
	jumpAmount := int(math.Floor(math.Sqrt(float64((len(breaks))))))
	i := jumpAmount

	for ; i < len(breaks); i += jumpAmount {
		if breaks[i] {
			break
		}
	}
	i -= jumpAmount

	if i < 0 {
		i = 0
	}

	for j := 0; j <= jumpAmount && i < len(breaks); j, i = j+1, i+1 {
		if breaks[i] {
			return i
		}
	}
	return -1
}

func main() {
	breaks := []bool{
		false,
		false,
		false,
		false,
		false,
		false,
		false,
		false,
		false,
		false,
		false,
		false,
		false,
		true,
		true,
	}
	println(twoCrystalBalls(breaks))
}
