use std::{hash::Hash, marker::PhantomData};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use rayon::prelude::*;

macro_rules! create_id {
    ($id_name:ident) => {
        #[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash)]
        struct $id_name(u64);

        impl From<u64> for $id_name {
            fn from(value: u64) -> Self {
                Self(value)
            }
        }

        impl From<$id_name> for u64 {
            fn from(value: $id_name) -> Self {
                value.0
            }
        }
    };
}

create_id!(Seed);
create_id!(Soil);
create_id!(Fertilizer);
create_id!(Water);
create_id!(Light);
create_id!(Temperature);
create_id!(Humidity);
create_id!(Location);
struct Almanac {
    seeds: Vec<Seed>,
    seed_to_soil: Map<Seed, Soil>,
    soil_to_fertilizer: Map<Soil, Fertilizer>,
    fertilizer_to_water: Map<Fertilizer, Water>,
    water_to_light: Map<Water, Light>,
    light_to_temperature: Map<Light, Temperature>,
    temperature_to_humidity: Map<Temperature, Humidity>,
    humidity_to_location: Map<Humidity, Location>,
}

#[derive(Debug, PartialEq)]
struct MapLine {
    src_start: u64,
    dest_start: u64,
    count: u64,
}

impl MapLine {
    fn contains(&self, id: u64) -> bool {
        (self.src_start..self.src_start + self.count).contains(&id)
    }

    fn get(&self, id: u64) -> u64 {
        let offset = self.dest_start as i64 - self.src_start as i64;

        (id as i64 + offset) as u64
    }
}

struct Map<S, D>
where
    D: From<u64>,
    S: Into<u64>,
{
    _src: PhantomData<S>,
    _dest: PhantomData<D>,
    lines: Vec<MapLine>,
}

impl<S, D> Map<S, D>
where
    D: From<u64> + Copy,
    S: Into<u64> + Copy,
{
    fn get(&self, src: S) -> D {
        self.lines
            .iter()
            .find(|line| line.contains(src.into()))
            .map(|line| D::from(line.get(src.into())))
            .unwrap_or(D::from(src.into()))
    }

    fn new(lines: Vec<MapLine>) -> Self {
        Self {
            _src: Default::default(),
            _dest: Default::default(),
            lines,
        }
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> color_eyre::Result<Almanac> {
    parsers::parse_input(input)
}

#[aoc(day5, part1)]
pub fn part1(almanac: &Almanac) -> u64 {
    almanac
        .seeds
        .iter()
        .map(|&seed| almanac.seed_to_soil.get(seed))
        .map(|soil| almanac.soil_to_fertilizer.get(soil))
        .map(|fertilizer| almanac.fertilizer_to_water.get(fertilizer))
        .map(|water| almanac.water_to_light.get(water))
        .map(|light| almanac.light_to_temperature.get(light))
        .map(|temp| almanac.temperature_to_humidity.get(temp))
        .map(|humdity| almanac.humidity_to_location.get(humdity))
        .map(|loc| loc.0)
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2(almanac: &Almanac) -> u64 {
    let seeds: Vec<Seed> = almanac
        .seeds
        .iter()
        .chunks(2)
        .into_iter()
        .flat_map(|mut chnk| (chnk.next().unwrap().0..).take(chnk.next().unwrap().0 as usize))
        .map(|seed| seed.into())
        .collect();

    seeds
        .par_iter()
        .map(|&seed| almanac.seed_to_soil.get(seed))
        .map(|soil| almanac.soil_to_fertilizer.get(soil))
        .map(|fertilizer| almanac.fertilizer_to_water.get(fertilizer))
        .map(|water| almanac.water_to_light.get(water))
        .map(|light| almanac.light_to_temperature.get(light))
        .map(|temp| almanac.temperature_to_humidity.get(temp))
        .map(|humdity| almanac.humidity_to_location.get(humdity))
        .map(|loc| loc.0)
        .min()
        .unwrap()
}

mod parsers {
    use nom::{
        bytes::complete::tag,
        character,
        character::complete::{newline, space0, space1},
        combinator::{all_consuming, eof, map},
        multi::separated_list1,
        sequence::{preceded, terminated, tuple},
        Finish, IResult,
    };

    use crate::day5::{
        Almanac, Fertilizer, Humidity, Light, Location, Map, MapLine, Seed, Soil, Temperature,
        Water,
    };

    pub(crate) fn parse_input(input: &str) -> color_eyre::Result<Almanac> {
        let (_, almanac) = all_consuming(almanac)(input)
            .map_err(|e| e.to_owned())
            .finish()?;
        Ok(almanac)
    }

    fn almanac(input: &str) -> IResult<&str, Almanac> {
        map(
            tuple((
                terminated(terminated(seeds, newline), newline),
                terminated(terminated(seed_to_soil, newline), newline),
                terminated(terminated(soil_to_fertilizer, newline), newline),
                terminated(terminated(fertilizer_to_water, newline), newline),
                terminated(terminated(water_to_light, newline), newline),
                terminated(terminated(light_to_temperature, newline), newline),
                terminated(terminated(temperature_to_humidity, newline), newline),
                terminated(humidity_to_location, eof),
            )),
            |(
                seeds,
                seed_to_soil,
                soil_to_fertilizer,
                fertilizer_to_water,
                water_to_light,
                light_to_temperature,
                temperature_to_humidity,
                humidity_to_location,
            )| {
                Almanac {
                    seeds,

                    seed_to_soil,
                    soil_to_fertilizer,
                    fertilizer_to_water,
                    water_to_light,
                    light_to_temperature,
                    temperature_to_humidity,
                    humidity_to_location,
                }
            },
        )(input)
    }

    fn seeds(input: &str) -> IResult<&str, Vec<Seed>> {
        map(
            preceded(
                terminated(tag("seeds:"), space1),
                separated_list1(space1, character::complete::u64),
            ),
            |ids| ids.iter().copied().map(Seed::from).collect(),
        )(input)
    }

    macro_rules! map_parser {
        ($parser_name:ident, $tag:expr, $source:ty, $dest:ty) => {
            fn $parser_name(input: &str) -> IResult<&str, Map<$source, $dest>> {
                let (input, lines) = preceded(
                    terminated(
                        terminated(tag($tag), space1),
                        terminated(tag("map:"), newline),
                    ),
                    separated_list1(newline, map_line),
                )(input)?;

                Ok((input, Map::new(lines)))
            }
        };
    }

    map_parser!(seed_to_soil, "seed-to-soil", Seed, Soil);

    map_parser!(soil_to_fertilizer, "soil-to-fertilizer", Soil, Fertilizer);

    map_parser!(
        fertilizer_to_water,
        "fertilizer-to-water",
        Fertilizer,
        Water
    );

    map_parser!(water_to_light, "water-to-light", Water, Light);

    map_parser!(
        light_to_temperature,
        "light-to-temperature",
        Light,
        Temperature
    );

    map_parser!(
        temperature_to_humidity,
        "temperature-to-humidity",
        Temperature,
        Humidity
    );

    map_parser!(
        humidity_to_location,
        "humidity-to-location",
        Humidity,
        Location
    );

    fn map_line(input: &str) -> IResult<&str, MapLine> {
        map(
            tuple((
                terminated(character::complete::u64, space0),
                terminated(character::complete::u64, space0),
                terminated(character::complete::u64, space0),
            )),
            |(dest, src, count)| MapLine {
                src_start: src,
                dest_start: dest,
                count,
            },
        )(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_map_line() {
            let line = "50 98 2";
            let (_, actual) = map_line(line).unwrap();
            let expected = MapLine {
                src_start: 98,
                dest_start: 50,
                count: 2,
            };
            assert_eq!(expected, actual);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn parse_input() {
        let _ = input_generator(INPUT).unwrap();
    }

    #[test]
    fn part1_sample_input() {
        let almanac = input_generator(INPUT).unwrap();

        let actual = part1(&almanac);

        assert_eq!(35, actual);
    }

    #[test]
    fn part2_sample_input() {
        let almanac = input_generator(INPUT).unwrap();

        let actual = part2(&almanac);

        assert_eq!(46, actual);
    }
}
