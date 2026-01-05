use std::{collections::{HashMap, HashSet}, fs, usize};
use std::cmp::{min, max};

const FILENAME: &str = "./inputs/input.txt";

const START_CHAR: char = 'S';
const EMPTY_CHAR: char = '.';
const PIPE_CONNECTIONS: [(char, [(i32, i32); 2]); 6] = [
    ('|', [(1,0), (-1,0)]),
    ('-', [(0,1), (0,-1)]),
    ('L', [(0,1), (-1,0)]),
    ('J', [(0,-1), (-1,0)]),
    ('7', [(0,-1), (1,0)]),
    ('F', [(0,1), (1,0)])
];

const DIRECTIONS: [(i32, i32); 4] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0)
];

fn main() {
    let (
        graph,
        start_coords
    ) = parse_input();

    // println!("{:?}", graph);
    println!("Starting at: {:?}", start_coords);

    let md = max_distance(&graph, start_coords);
    println!("Part 1: {}", md);

    let area = flood_fill(&perimeter(&graph, start_coords));

    println!("area: {}", area);
}

// this isn't perfect
// it only works if the perimeter is in the correct direction
// this won't always be true
// but whatever, it only had to work once
// it's also kind of hacky but who cares
fn flood_fill (perimeter: &Vec<(i32, i32)>) -> usize {
    let mut dirs = DIRECTIONS.iter().enumerate().map(|(idx, d)| (d, idx)).collect::<HashMap<_, _>>();

    let perimeter_set: HashSet<(i32, i32)> = perimeter.clone().into_iter().collect::<HashSet<_>>();
    let mut internals: HashSet<(i32, i32)> =  HashSet::new();

    let mut step_idx = 0;
    loop {
        println!("Step: {}", step_idx);
        println!("Current area: {}", internals.len());
        if step_idx == perimeter.len() + 1 {
            break;
        }

        let node_0 = perimeter[step_idx % perimeter.len()];
        let node_1 = perimeter[(step_idx + 1) % perimeter.len()];

        step_idx += 1;

        let direction = ((node_1.0 - node_0.0).signum(), (node_1.1 - node_0.1).signum());
        let dir_idx = dirs.get(&direction).unwrap();

        let clockwise_dir = DIRECTIONS[(dir_idx + 1) % 4];
        let next_point_1 = (node_0.0 + clockwise_dir.0, node_0.1 + clockwise_dir.1);
        let next_point_2 = (node_1.0 + clockwise_dir.0, node_1.1 + clockwise_dir.1);

        if !internals.contains(&next_point_1) && !perimeter_set.contains(&next_point_1) {
            internals.insert(next_point_1);
            flood(next_point_1, &mut internals, &perimeter_set);
        }

        if !internals.contains(&next_point_2) && !perimeter_set.contains(&next_point_2) {
            internals.insert(next_point_2);
            flood(next_point_2, &mut internals, &perimeter_set);
        }

    }


    return internals.len();
}

fn flood (point: (i32, i32), point_set: &mut HashSet<(i32, i32)>, boundary_set: &HashSet<(i32, i32)>) {
    for dir_idx in 0..4 {
        let dir = DIRECTIONS[dir_idx];
        let next_point = (point.0 + dir.0, point.1 + dir.1);
        if !point_set.contains(&next_point) && !boundary_set.contains(&next_point) {
            point_set.insert(next_point);
            return flood(next_point, point_set, boundary_set);
        }
    }
}

fn area (perimeter: &Vec<(i32, i32)>) {
    let mut area = 0;
    let mut perim = perimeter.clone();

    let mut step_idx = 0;
    loop {
        let node_0 = perimeter[step_idx % perimeter.len()];
        let node_1 = perimeter[(step_idx + 1) % perimeter.len()];
        let node_2 = perimeter[(step_idx + 2) % perimeter.len()];
        // let node_3 = perimeter[(step_idx + 2) % perimeter.len()];

        if perim.iter().any(|corner| is_in_rect([node_0, node_2], *corner)) {
            continue;
        }

        let [(min_x, min_y), (max_x, max_y)] = rect([node_0, node_2]); 

        area += (max_x - min_x - 1) * (max_y - min_y - 1);
    }
}

fn rect (corners: [(i32, i32); 2]) -> [(i32, i32); 2] {
    let min_x = min(corners[0].0, corners[1].0);
    let max_x = max(corners[0].0, corners[1].0);
    let min_y = min(corners[0].1, corners[1].1);
    let max_y = max(corners[0].1, corners[1].1);

    return [(min_x, min_y), (max_x, max_y)];
}

fn is_in_rect (corners: [(i32, i32); 2], point: (i32, i32)) -> bool {
    let [(min_x, min_y), (max_x, max_y)] = rect(corners);

    if min_x < point.0 && point.0 < max_x && min_y < point.1 && point.1 < max_y {
        return true;
    }
    return false;
}

fn minimum_perimeter (perimeter: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let n_nodes = perimeter.len();
    let mut corners = vec![];

    let mut step_idx = 2;

    loop {
        if step_idx == perimeter.len() + 2 {
            break;
        }

        let node_0 = perimeter[step_idx % n_nodes];
        let node_1 = perimeter[(step_idx + 1) % n_nodes];
        let node_2 = perimeter[(step_idx + 2) % n_nodes];

        if (node_0.0 == node_1.0 && node_1.0 != node_2.0) || (node_0.1 == node_1.1 && node_1.1 != node_2.1) {
            corners.push(node_1);
        }

        step_idx += 1;
    }


    return corners;
}

fn perimeter (graph: &HashMap<(i32, i32), [(i32, i32); 2]>, start_coords: (i32, i32)) -> Vec<(i32, i32)> {
    let mut nodes = vec![];
    let mut step_idx= 0;

    let mut last_node = (i32::MAX, i32::MAX);
    let mut node = start_coords;

    loop {
        nodes.push(node.clone());

        let next_node;
        let adjacent = graph.get(&node).unwrap();
        if adjacent[0] == last_node {
            next_node = adjacent[1];
        } else {
            next_node = adjacent[0];
        }
        step_idx += 1;

        last_node = node;
        node = next_node;


        if node == start_coords {
            break;
        }
    }

    return nodes;
}

fn max_distance (graph: &HashMap<(i32, i32), [(i32, i32); 2]>, start_coords: (i32, i32)) -> usize {
    let perimeter = perimeter(graph, start_coords);

    return perimeter.len() / 2;
}

// obviously this wasn't necessary
// we know it's a loop
// we can just traverse it
// but hey, i like dijkstra's algorithm
fn dijkstra (graph: &HashMap<(i32, i32), [(i32, i32); 2]>, start_coords: (i32, i32)) -> usize {
    let mut unvisited = graph.keys().collect::<HashSet<_>>();
    let mut visited = HashSet::new();
    let mut distances = graph.keys().map(|v| (v, usize::MAX)).collect::<HashMap<&(i32, i32), usize>>();
    distances.insert(&start_coords, 0);

    loop {
        if unvisited.len() == 0 {
            break;
        }
        let node = smallest_dist_node(&unvisited, &distances);
        // println!("{:?}", node);
        // println!("Unvisited: {}", unvisited.len());

        if distances.get(&node).is_none() {
            println!("No node {:?}", node);
            break;
        }

        let dist = *distances.get(&node).unwrap();
        if dist == usize::MAX {
            break;
        }
        unvisited.remove(&node);
        
        let adjacent = graph.get(&node).unwrap();

        for adj in adjacent {
            if unvisited.contains(adj) {
                distances.insert(adj, dist + 1);
            }
        }

        visited.insert(node);
    }

    unvisited.remove(&start_coords);

    let mut max_dist = 0;
    for node in visited {
        let dist = *distances.get(&node).unwrap();
        if dist > max_dist {
            max_dist = dist;
        }
    }
    return max_dist;
}

fn smallest_dist_node (unvisited: &HashSet<&(i32, i32)>, dists: &HashMap<&(i32, i32), usize>) -> (i32, i32) {
    let mut smallest_dist = usize::MAX;
    let mut sd_node = (0, 0);

    for node in unvisited {
        let d = dists.get(node).unwrap();
        if *d < smallest_dist {
            smallest_dist = *d;
            sd_node = (node.0, node.1);
        }
    }

    return sd_node;
}

fn match_pipe (pipe: char) -> [(i32, i32); 2] {
    match pipe {
        '|' => [(1,0), (-1,0)],
        '-' => [(0,1), (0,-1)],
        'L' => [(0,1), (-1,0)],
        'J' => [(0,-1), (-1,0)],
        '7' => [(0,-1), (1,0)],
        'F' => [(0,1), (1,0)],
        _ => panic!()
    }
}

fn parse_input () -> (HashMap<(i32, i32), [(i32, i32); 2]>, (i32, i32)) {
    let mut graph = HashMap::new();
    let mut s_coords = None;

    for (row_idx, line) in fs::read_to_string(FILENAME).unwrap().lines().enumerate() {
        for (col_idx, c) in line.chars().enumerate() {
            let coords = (row_idx as i32, col_idx as i32);
            if c == START_CHAR {
                s_coords = Some(coords);
                continue;
            }

            if c == EMPTY_CHAR {
                continue;
            }

            let directions = match_pipe(c);
            let adjacent_nodes = [add_coords(coords, directions[0]), add_coords(coords, directions[1])]; 

            graph.insert(coords, adjacent_nodes);
        }
    }

    let mut start_adjacents = vec![];
    for (node, adjacent) in graph.iter() {
        if adjacent[0] == s_coords.unwrap() || adjacent[1] == s_coords.unwrap() {
            start_adjacents.push(node);
        }
    }

    graph.insert(s_coords.unwrap(), [*start_adjacents[0], *start_adjacents[1]]);

    return (graph, s_coords.unwrap())
}

fn add_coords (c1: (i32, i32), c2: (i32, i32)) -> (i32, i32) {
    return (c1.0 + c2.0, c1.1 + c2.1);
}