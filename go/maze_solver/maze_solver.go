package maze_solver

type Point struct {
	X int
	Y int
}

func walk(maze []string, wall string, curr Point, end Point, seen [][]bool, path *[]Point) bool {
	dir := [][2]int{
		{-1, 0}, // left
		{1, 0},  // right
		{0, -1}, // down
		{0, 1},  // up
	}
	// 1. Base Cases

	// off the map
	if curr.X < 0 ||
		curr.X >= len(maze[0]) ||
		curr.Y < 0 ||
		curr.Y >= len(maze) {
		return false
	}

	// hit a wall
	if string(maze[curr.Y][curr.X]) == wall {
		return false
	}

	// already seen
	if seen[curr.Y][curr.X] {
		return false
	}

	// at the end
	if curr.X == end.X && curr.Y == end.Y {
		*path = append(*path, curr)
		return true
	}

	// 2. Recursive Case

	// pre-recursion
	seen[curr.Y][curr.X] = true
	*path = append(*path, curr)

	// recurse
	for i := 0; i < len(dir); i++ {
		dx, dy := dir[i][0], dir[i][1]

		if walk(maze, wall, Point{curr.X + dx, curr.Y + dy}, end, seen, path) {
			return true
		}
	}
	// post-recursion
	*path = (*path)[:len(*path)-1] // magic parentheses
	return false
}

func maze_solver(maze []string, wall string, start Point, end Point) []Point {
	seen := make([][]bool, len(maze))
	path := make([]Point, 0)
	for i := range maze {
		seen[i] = make([]bool, len(maze[0]))
	}
	walk(maze, wall, start, end, seen, &path)
	return path
}
