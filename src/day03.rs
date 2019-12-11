// Day 03 AOC 2019


type Coord = (i32, i32);
type LineCoords = Vec<Coord>;


#[aoc_generator(day3)]
fn get_coords(input: &str) -> Vec<LineCoords> {
    let mut coords: Vec<LineCoords> = vec!();

    for line in input.lines() {

        let mut coord: Coord = (0, 0);
        let mut line_coords: LineCoords = vec!();
        let line_moves: Vec<&str> = line.trim().split(",").collect();

        for mv in line_moves {

            // Move amount
            let amount = remove_first(mv).parse::<i32>().unwrap();

            // Get all coords for move
            let mut coord_range: Vec<Coord> =
                match mv.chars().next().unwrap() {
                    'U' => (coord.1 + 1 .. coord.1 + amount + 1).map(|v| (coord.0, v)).collect(),

                    'D' => (coord.1 - amount .. coord.1).rev().map(|v| (coord.0, v)).collect(),

                    'R' => (coord.0 + 1 .. coord.0 + amount + 1).map(|v| (v, coord.1)).collect(),

                    'L' => (coord.0 - amount .. coord.0).rev().map(|v| (v, coord.1)).collect(),

                    _ => panic!("Day 03 - I don't know what this move is {}", mv),
                };

            // Add to line coords
            line_coords.append(&mut coord_range);

            // Take last coord
            coord = *line_coords.last().unwrap();
        }

        coords.push(line_coords);
    }

    coords
}


#[aoc(day3, part1)]
fn part1(coords: &Vec<LineCoords>) -> Option<i32> {
    let intersections = get_intersections(&coords);

    intersections
        .iter()
        .map(|(a, b)| a.abs() + b.abs())
        .min()
}


#[aoc(day3, part2)]
fn part2(coords: &Vec<LineCoords>) -> Option<i32> {
    let intersections = get_intersections(&coords);
    let mut per_line_steps: Vec<Vec<usize>> = vec!();

    for line in coords.iter() {
        let mut steps: Vec<usize> = vec!();
        
        for i in intersections.iter() {
            steps.push((line.iter().position(|v| v == i).unwrap()) + 1);
        }

        per_line_steps.push(steps);
    }

    let line1 = per_line_steps.get(0).unwrap();
    let line2 = per_line_steps.get(1).unwrap();

    let mut combined: Vec<i32> = vec!();

    for (a, b) in line1.iter().zip(line2.iter()) {
        combined.push((a.clone() + b.clone()) as i32);
    }

    combined.into_iter().min()
}


fn get_intersections(lines: &Vec<LineCoords>) -> LineCoords {
    let mut flat_coords: LineCoords = vec!();
    let mut repeated_coords: LineCoords = vec!();

    for line_coords in lines.iter() {
        
        // Remove duplicates from line
        let mut unique_coords = vec!();
        for coord in line_coords.iter() {
            if !unique_coords.contains(coord) {
                unique_coords.push(*coord);
            }
        }

        // Check uniques and sort them properly
        for coord in unique_coords.iter() {
            if flat_coords.contains(coord) {
                repeated_coords.push(*coord);
            } else {
                flat_coords.push(*coord);
            }
        }
    }

    repeated_coords
}


fn remove_first(s: &str) -> &str {
    s.chars().next().map(|c| &s[c.len_utf8()..]).unwrap()
}


#[cfg(test)]
mod tests {
    use super::{get_coords, part1, part2};

    #[test]
    fn day3_part1() {
        let coords1 = get_coords("R8,U5,L5,D3\nU7,R6,D4,L4");
        let coords2 = get_coords("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83");
        let coords3 = get_coords("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        
        assert_eq!(part1(&coords1), Some(6));
        assert_eq!(part1(&coords2), Some(159));
        assert_eq!(part1(&coords3), Some(135));
    }
    
    #[test]
    fn day3_part2() {
        let coords1 = get_coords("R8,U5,L5,D3\nU7,R6,D4,L4");
        let coords2 = get_coords("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83");
        let coords3 = get_coords("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        assert_eq!(part2(&coords1), Some(30));
        assert_eq!(part2(&coords2), Some(610));
        assert_eq!(part2(&coords3), Some(410));
    }
}
