package maze_solver

import (
	"fmt"
	"reflect"
	"testing"
)

// drawPath in Go
func drawPath(data []string, path []Point) []string {
	var result []string
	for _, row := range data {
		bytes := []byte(row)
		result = append(result, string(bytes))
	}

	for _, p := range path {
		if p.Y < len(result) && p.X < len(result[p.Y]) {
			bytes := []byte(result[p.Y])
			bytes[p.X] = '*'
			result[p.Y] = string(bytes)
		}
	}
	return result
}

func TestMazeSolver(t *testing.T) {
	maze := []string{
		"xxxxxxxxxx x",
		"x        x x",
		"x        x x",
		"x xxxxxxxx x",
		"x          x",
		"x xxxxxxxxxx",
	}

	mazeResult := []Point{
		{X: 10, Y: 0},
		{X: 10, Y: 1},
		{X: 10, Y: 2},
		{X: 10, Y: 3},
		{X: 10, Y: 4},
		{X: 9, Y: 4},
		{X: 8, Y: 4},
		{X: 7, Y: 4},
		{X: 6, Y: 4},
		{X: 5, Y: 4},
		{X: 4, Y: 4},
		{X: 3, Y: 4},
		{X: 2, Y: 4},
		{X: 1, Y: 4},
		{X: 1, Y: 5},
	}

	// Assuming maze_solver is implemented
	result := maze_solver(maze, "x", Point{X: 10, Y: 0}, Point{X: 1, Y: 5})
	fmt.Println(result)
	if !reflect.DeepEqual(drawPath(maze, result), drawPath(maze, mazeResult)) {
		t.Errorf("Maze paths did not match")
	}
}
