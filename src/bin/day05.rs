use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

use rayon::prelude::*;

struct AlmanacEntry {
    src_start: usize,
    dest_start: usize,
    range_length: usize,
}

impl AlmanacEntry {
    fn dest_mapping(&self, element: usize) -> Option<usize> {
        if element < self.src_start || element >= self.src_start + self.range_length {
            None
        } else {
            Some(self.dest_start + (element - self.src_start))
        }
    }
}

impl TryFrom<&str> for AlmanacEntry {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut iter = value.split_whitespace();
        let end = match iter.next() {
            Some(val) => usize::from_str_radix(val, 10).ok(),
            _ => None,
        };

        let start = match iter.next() {
            Some(val) => usize::from_str_radix(val, 10).ok(),
            _ => None,
        };

        let length = match iter.next() {
            Some(val) => usize::from_str_radix(val, 10).ok(),
            _ => None,
        };

        match (start, end, length) {
            (Some(start), Some(end), Some(length)) => Ok(Self {
                src_start: start,
                dest_start: end,
                range_length: length,
            }),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Failed to parse entry")),
        }
    }
}

struct AlmanacCategory {
    entries: Vec<AlmanacEntry>,
}

impl AlmanacCategory {
    fn get_dest_mapping(&self, input: &usize) -> usize {
        match self
            .entries
            .iter()
            .filter_map(|entry| entry.dest_mapping(*input))
            .nth(0)
        {
            Some(mapping) => mapping,
            None => *input,
        }
    }
}

impl FromIterator<AlmanacEntry> for AlmanacCategory {
    fn from_iter<T: IntoIterator<Item = AlmanacEntry>>(iter: T) -> Self {
        let mut entries = vec![];

        for entry in iter {
            entries.push(entry);
        }

        AlmanacCategory { entries }
    }
}

impl TryFrom<&str> for AlmanacCategory {
    type Error = std::io::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(value
            .split("\n")
            .skip(1)
            .filter_map(|entry| AlmanacEntry::try_from(entry).ok())
            .collect::<AlmanacCategory>())
    }
}

struct Almanac {
    seed_input: Vec<usize>,
    seed_to_soil: AlmanacCategory,
    soil_to_fertiliser: AlmanacCategory,
    fertiliser_to_water: AlmanacCategory,
    water_to_light: AlmanacCategory,
    light_to_temperature: AlmanacCategory,
    temperature_to_humidity: AlmanacCategory,
    humidity_to_location: AlmanacCategory,
}

impl FromStr for Almanac {
    type Err = std::io::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut iter = value.split("\n\n").into_iter();

        let seed_input = match iter.nth(0) {
            Some(seeds_str) => Ok(seeds_str
                .split_whitespace()
                .filter_map(|seed_str| usize::from_str_radix(seed_str, 10).ok())
                .collect::<Vec<usize>>()),
            None => Err(Error::new(ErrorKind::InvalidInput, "Couldn't parse seeds")),
        }?;

        // These unwraps would fail, but input is well-defined for this small exercise
        let seed_to_soil = AlmanacCategory::try_from(iter.next().unwrap())?;
        let soil_to_fertiliser = AlmanacCategory::try_from(iter.next().unwrap())?;
        let fertiliser_to_water = AlmanacCategory::try_from(iter.next().unwrap())?;
        let water_to_light = AlmanacCategory::try_from(iter.next().unwrap())?;
        let light_to_temperature = AlmanacCategory::try_from(iter.next().unwrap())?;
        let temperature_to_humidity = AlmanacCategory::try_from(iter.next().unwrap())?;
        let humidity_to_location = AlmanacCategory::try_from(iter.next().unwrap())?;

        Ok(Self {
            seed_input,
            seed_to_soil,
            soil_to_fertiliser,
            fertiliser_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
}

impl Almanac {
    // Might be a way to define a common function that takes an iterator as input
    fn get_lowest_location(&self) -> usize {
        self.seed_input
            .par_iter()
            .map(|seed| self.seed_to_soil.get_dest_mapping(seed))
            .map(|soil| self.soil_to_fertiliser.get_dest_mapping(&soil))
            .map(|fertiliser| self.fertiliser_to_water.get_dest_mapping(&fertiliser))
            .map(|water| self.water_to_light.get_dest_mapping(&water))
            .map(|light| self.light_to_temperature.get_dest_mapping(&light))
            .map(|temperature| self.temperature_to_humidity.get_dest_mapping(&temperature))
            .map(|humidity| self.humidity_to_location.get_dest_mapping(&humidity))
            .min()
            .unwrap_or(0)
    }

    fn get_lowest_location_from_seed_range(&self) -> usize {
        self.seed_input
            .par_chunks(2)
            .flat_map(|chunk| chunk[0]..chunk[0] + chunk[1])
            .map(|seed| self.seed_to_soil.get_dest_mapping(&seed))
            .map(|soil| self.soil_to_fertiliser.get_dest_mapping(&soil))
            .map(|fertiliser| self.fertiliser_to_water.get_dest_mapping(&fertiliser))
            .map(|water| self.water_to_light.get_dest_mapping(&water))
            .map(|light| self.light_to_temperature.get_dest_mapping(&light))
            .map(|temperature| self.temperature_to_humidity.get_dest_mapping(&temperature))
            .map(|humidity| self.humidity_to_location.get_dest_mapping(&humidity))
            .min()
            .unwrap_or(0)
    }
}

fn main() -> Result<(), std::io::Error> {
    let puzzle_input = std::fs::read_to_string("inputs/day5.txt")?;

    let almanac = Almanac::from_str(&puzzle_input)?;
    let low_location = almanac.get_lowest_location();
    let low_location_range = almanac.get_lowest_location_from_seed_range();

    println!(
        "Low location: {}\nLow location part 2 {}",
        low_location, low_location_range
    );
    Ok(())
}
