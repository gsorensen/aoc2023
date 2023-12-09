use std::str::FromStr;

#[derive(Debug, Clone)]
struct Reading {
    data: Vec<isize>,
    zero_differences: Vec<Vec<isize>>,
}

impl FromStr for Reading {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .split_whitespace()
            .filter_map(|reading| reading.parse::<isize>().ok())
            .collect::<Vec<_>>();

        let mut zero_differences = Vec::with_capacity(data.len() - 1);
        let mut zero_diff = zero_difference(&data);
        let mut all_zero_diff = is_all_elements_zero(&zero_diff);
        zero_differences.push(zero_diff.clone());

        while !all_zero_diff {
            zero_diff = zero_difference(&zero_diff);
            all_zero_diff = is_all_elements_zero(&zero_diff);
            zero_differences.push(zero_diff.clone());
        }

        Ok(Self {
            data,
            zero_differences,
        })
    }
}

impl Reading {
    fn extrapolate(&mut self) {
        let zero_diffs = self.zero_differences.len();
        let mut current_added_value = 0;

        for idx in (0..zero_diffs).rev() {
            self.zero_differences[idx].push(current_added_value);

            if idx == 0 {
                current_added_value += *self.data.last().expect("It's here")
            } else {
                current_added_value += *self.zero_differences[idx - 1].last().expect("It's there");
            }
        }

        self.data.push(current_added_value)
    }

    fn backward_extrapolate(&mut self) {
        let zero_diffs = self.zero_differences.len();
        let mut current_added_value = 0;

        for idx in (0..zero_diffs).rev() {
            self.zero_differences[idx].insert(0, current_added_value);

            if idx == 0 {
                current_added_value = *self.data.first().expect("It's here") - current_added_value;
            } else {
                current_added_value = *self.zero_differences[idx - 1].first().expect("It's there")
                    - current_added_value;
            }
        }

        self.data.insert(0, current_added_value)
    }

    fn get_last_reading(&self) -> isize {
        *self.data.last().expect("Data can't be empty")
    }

    fn get_first_reading(&self) -> isize {
        *self.data.first().expect("Data can't be empty")
    }
}

fn zero_difference(input: &[isize]) -> Vec<isize> {
    let mut difference = Vec::with_capacity(input.len() - 1);

    for i in 1..input.len() {
        difference.push(input[i] - input[i - 1]);
    }

    difference
}

fn is_all_elements_zero(input: &[isize]) -> bool {
    input.iter().all(|element| element == &0)
}

fn main() -> Result<(), anyhow::Error> {
    let puzzle_input = std::fs::read_to_string("inputs/day9.txt")?;

    let mut readings = puzzle_input
        .split("\n")
        .filter(|line| !line.is_empty())
        .filter_map(|line| Reading::from_str(line).ok())
        .collect::<Vec<_>>();

    let mut readings_cloned = readings.clone();

    for reading in readings.iter_mut() {
        reading.extrapolate();
    }

    for reading in readings_cloned.iter_mut() {
        reading.backward_extrapolate();
    }

    let sum = readings.iter().map(|r| r.get_last_reading()).sum::<isize>();
    let backsum = readings_cloned
        .iter()
        .map(|r| r.get_first_reading())
        .sum::<isize>();

    println!("The sum of extrapolated values is {sum}");
    println!("The sum of backward extrapolated values is {backsum}");

    Ok(())
}
