use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum SchematicContent {
    Number(usize),
    Symbol(char),
    Empty,
}

#[derive(Debug)]
struct SchematicItem {
    content: SchematicContent,
    row: usize,
    start_col: usize,
    end_col: usize,
}

impl SchematicItem {
    fn get_number(&self) -> Option<usize> {
        match self.content {
            SchematicContent::Number(x) => Some(x),
            _ => None,
        }
    }

    fn new(row: usize, start_col: usize, content: SchematicContent) -> Self {
        let end_col = match content {
            SchematicContent::Number(x) => start_col + (x as f64).log10() as usize,
            _ => start_col,
        };

        Self {
            content,
            row,
            start_col,
            end_col,
        }
    }

    // I am sure this could be done with generics
    fn is_adjacent_to_symbol(&self, other: &SchematicItem) -> bool {
        if let SchematicContent::Symbol(_) = other.content {
            let row_adjacent = usize::abs_diff(self.row, other.row) <= 1;
            let col_adjacent = (self.start_col..=self.end_col)
                .any(|col| usize::abs_diff(col, other.start_col) <= 1);

            row_adjacent && col_adjacent
        } else {
            false
        }
    }

    fn is_adjacent_to_number(&self, other: &SchematicItem) -> bool {
        if let SchematicContent::Number(_) = other.content {
            let row_adjacent = usize::abs_diff(self.row, other.row) <= 1;
            let col_adjacent = (other.start_col..=other.end_col)
                .any(|col| usize::abs_diff(col, self.start_col) <= 1);

            row_adjacent && col_adjacent
        } else {
            false
        }
    }
}

fn parse_schematic_content(symbol: &str) -> Option<SchematicContent> {
    if symbol.is_empty() {
        return None;
    }

    if let Some(number) = usize::from_str_radix(symbol, 10).ok() {
        return Some(SchematicContent::Number(number));
    }

    if !symbol.is_empty() && !symbol.contains(".") {
        return Some(SchematicContent::Symbol(symbol.chars().next().unwrap()));
    }

    return Some(SchematicContent::Empty);
}

fn parse_initial_schematic_item(symbol: &str, row: usize, col: usize) -> Option<SchematicItem> {
    let content = parse_schematic_content(symbol)?;
    Some(SchematicItem::new(row, col, content))
}

fn parse_line(line: &str, row: usize) -> Vec<SchematicItem> {
    line.split("")
        .filter(|symbol| !symbol.is_empty())
        .enumerate()
        .filter_map(|(col, symbol)| parse_initial_schematic_item(symbol, row, col))
        .coalesce(|lhs, rhs| match (lhs.content, rhs.content) {
            // If we have consecutive SchematicContent::Number, these are part of the same number
            // and need to be combined into one
            (SchematicContent::Number(xval), SchematicContent::Number(yval)) => {
                let number_str = format!("{}{}", xval, yval);
                match usize::from_str_radix(&number_str, 10) {
                    Ok(number) => Ok(SchematicItem {
                        row,
                        start_col: lhs.start_col,
                        end_col: rhs.end_col,
                        content: SchematicContent::Number(number),
                    }),
                    Err(_) => Err((lhs, rhs)),
                }
            }
            _ => Err((lhs, rhs)),
        })
        .collect()
}

fn part1(number_items: &Vec<&SchematicItem>, symbol_items: &Vec<&SchematicItem>) -> usize {
    number_items
        .iter()
        .filter(|item| {
            symbol_items
                .iter()
                .any(|symbol_item| item.is_adjacent_to_symbol(&symbol_item))
        })
        .map(|item| match item.content {
            SchematicContent::Number(val) => val,
            _ => 0,
        })
        .sum()
}

fn part2(number_items: &Vec<&SchematicItem>, symbol_items: &Vec<&SchematicItem>) -> usize {
    symbol_items
        .iter()
        .filter_map(|item| {
            let adjacent_numbers = number_items
                .iter()
                .filter_map(|number_item| {
                    if item.is_adjacent_to_number(&number_item) {
                        number_item.get_number()
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>();

            if adjacent_numbers.len() == 2 {
                Some(adjacent_numbers.iter().fold(1, |acc, x| acc * x))
            } else {
                None
            }
        })
        .sum()
}

fn main() -> Result<(), anyhow::Error> {
    let puzzle_input = std::fs::read_to_string("inputs/day3.txt")?;

    let contents = puzzle_input
        .split("\n")
        .filter(|line| line.len() > 0)
        .enumerate()
        .map(|(row, line)| parse_line(line, row))
        .collect::<Vec<Vec<SchematicItem>>>();

    let number_items = contents
        .iter()
        .map(|row| {
            row.iter().filter(|item| match item.content {
                SchematicContent::Number(_) => true,
                _ => false,
            })
        })
        .flatten()
        .collect::<Vec<&SchematicItem>>();

    let symbol_items = contents
        .iter()
        .map(|row| {
            row.iter().filter(|item| match item.content {
                SchematicContent::Symbol(_) => true,
                _ => false,
            })
        })
        .flatten()
        .collect::<Vec<&SchematicItem>>();

    let part_sum = part1(&number_items, &symbol_items);

    let gear_ratio_sum = part2(&number_items, &symbol_items);

    println!("Part sum: {}\nGear ratio sum: {}", part_sum, gear_ratio_sum);

    Ok(())
}
