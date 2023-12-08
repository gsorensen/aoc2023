use std::collections::HashMap;

use rayon::prelude::*;

fn lcm(first: usize, second: usize) -> usize {
    (first * second) / gcd(first, second)
}

fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn main() -> Result<(), anyhow::Error> {
    let puzzle_input = std::fs::read_to_string("inputs/day8.txt")?;

    let mut iter = puzzle_input.split("\n\n");

    let instructions = iter
        .nth(0)
        .expect("Instructions should be there")
        .chars()
        .map(|c| match c {
            'L' => 0,
            'R' => 1,
            _ => unreachable!("Instructions only contai nleft and right"),
        })
        .collect::<Vec<u32>>();

    let graph_data = iter
        .nth(0)
        .expect("Instructions should be there")
        .split("\n")
        .filter_map(|line| line.split_once(" = "))
        .collect::<Vec<(&str, &str)>>();

    let mut graph = HashMap::<String, (String, String)>::new();
    for (node, neighbour_str) in graph_data.into_iter() {
        let neighbour_str = neighbour_str.replace("(", "");
        let neighbour_str = neighbour_str.replace(")", "");
        let (left, right) = neighbour_str.split_once(", ").expect("Should be here");

        graph.insert(
            String::from(node),
            (String::from(left), String::from(right)),
        );
    }

    let mut steps = 0;

    let mut starts = graph
        .iter()
        .filter(|(k, _)| k.ends_with("A"))
        .collect::<Vec<_>>();

    let num_instructions = instructions.len();
    let mut count = 0;
    let mut all_at_end = false;

    let mut steps_map = HashMap::<String, usize>::new();

    let num_start = starts.len();
    let mut finished = 0;

    while !all_at_end {
        if count == num_instructions {
            count = 0;
        }

        steps += 1;

        let instruction = instructions[count];

        for start in starts.iter_mut() {
            if (*start.0).ends_with("Z") == true && steps_map.get(start.0) == None {
                steps_map.insert((*start.0).clone(), steps.clone() - 1);
                finished += 1;
            } else if (*start.0).ends_with("Z") == true {
                continue;
            } else {
                let (_, (left, right)) = *start;

                if instruction == 0 {
                    *start = (left, graph.get(left).expect("Should exist"));
                } else {
                    *start = (right, graph.get(right).expect("Should exist"));
                }
            }
        }
        count += 1;

        // Can't iterate over starts here because you modify the object you're poining at
        // in the else block, thus, the condition is true the step before you have put the element
        // in the map for the final start point
        all_at_end = finished == num_start;
    }

    let val = steps_map
        .values()
        .into_iter()
        .fold(1, |acc, x| lcm(acc, *x));

    println!("{:?}", val);
    Ok(())
}
