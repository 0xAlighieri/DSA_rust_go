#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

pub fn walk(
    maze: &[String],
    wall: &str,
    curr: Point,
    end: Point,
    seen: &mut Vec<Vec<bool>>,
    path: &mut Vec<Point>,
) -> bool {
    let dir = vec![[-1, 0], [0, 1], [1, 0], [0, -1]];

    // 1. Base Cases

    // Off the map
    if curr.x < 0 || curr.x >= maze[0].len() as i32 || curr.y < 0 || curr.y >= maze.len() as i32 {
        return false;
    }

    // Hit a wall
    if maze[curr.y as usize].chars().nth(curr.x as usize).unwrap() == wall.chars().nth(0).unwrap() {
        return false;
    }

    // Already seen
    if seen[curr.y as usize][curr.x as usize] {
        return false;
    }

    // At the end
    if curr.x == end.x && curr.y == end.y {
        path.push(curr);
        return true;
    }

    // 2. Recursive Case

    // Pre-recursion
    seen[curr.y as usize][curr.x as usize] = true;
    path.push(curr.clone());

    // Recurse
    for d in dir {
        let dx = d[0];
        let dy = d[1];

        let next = Point {
            x: curr.x + dx,
            y: curr.y + dy,
        };

        if walk(maze, wall, next, end, seen, path) {
            return true;
        }
    }

    // Post-recursion
    path.pop();
    return false;
}

pub fn maze_solver(maze: &[String], wall: &str, start: Point, end: Point) -> Vec<Point> {
    let mut seen = vec![vec![false; maze[0].len()]; maze.len()];
    let mut path = vec![];

    walk(maze, wall, start, end, &mut seen, &mut path);

    return path;
}

// Assuming the `Point` struct and `maze_solver` function are defined as per your requirement

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maze_solver_finds_correct_path() {
        let maze = vec![
            "xxxxxxxxxx x".to_string(),
            "x        x x".to_string(),
            "x        x x".to_string(),
            "x xxxxxxxx x".to_string(),
            "x          x".to_string(),
            "x xxxxxxxxxx".to_string(),
        ];

        let start = Point { x: 10, y: 0 };
        let end = Point { x: 1, y: 5 };
        let result = maze_solver(&maze, "x", start, end);

        let expected_path = vec![
            Point { x: 10, y: 0 },
            Point { x: 10, y: 1 },
            Point { x: 10, y: 2 },
            Point { x: 10, y: 3 },
            Point { x: 10, y: 4 },
            Point { x: 9, y: 4 },
            Point { x: 8, y: 4 },
            Point { x: 7, y: 4 },
            Point { x: 6, y: 4 },
            Point { x: 5, y: 4 },
            Point { x: 4, y: 4 },
            Point { x: 3, y: 4 },
            Point { x: 2, y: 4 },
            Point { x: 1, y: 4 },
            Point { x: 1, y: 5 },
        ];

        assert_eq!(
            result, expected_path,
            "Maze solver did not find the correct path"
        );
    }
}
