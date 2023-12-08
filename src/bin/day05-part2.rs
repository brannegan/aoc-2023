#![allow(clippy::single_range_in_vec_init)]
use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Range;

use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1, u64};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::{Finish, IResult, Parser};

struct FoodProd {
    seeds: Vec<Range<u64>>,
    seed_to_soil: HashMap<Range<u64>, Range<u64>>,
    soil_to_fertilizer: HashMap<Range<u64>, Range<u64>>,
    fertilizer_to_water: HashMap<Range<u64>, Range<u64>>,
    water_to_light: HashMap<Range<u64>, Range<u64>>,
    light_to_temperature: HashMap<Range<u64>, Range<u64>>,
    temperature_to_humidity: HashMap<Range<u64>, Range<u64>>,
    humidity_to_location: HashMap<Range<u64>, Range<u64>>,
}
fn parse(input: &str) -> anyhow::Result<FoodProd> {
    let (_rest, food_prod) = parse_input(input)
        .finish()
        .map_err(|e| anyhow::anyhow!("parser error: {:?}", e))?;
    Ok(food_prod)
}
fn parse_input(input: &str) -> IResult<&str, FoodProd> {
    let (i, seeds) = preceded(
        tag("seeds: "),
        separated_list1(
            space1,
            separated_pair(u64, space1, u64).map(|(start, len)| start..start + len),
        ),
    )(input)?;
    let numbers = |i| {
        map(separated_list1(space1, u64), |v| {
            (v[1]..v[1] + v[2], v[0]..v[0] + v[2])
        })(i)
    };
    let mappings = |i| separated_list1(line_ending, numbers)(i);
    let gap = |i| line_ending.and(line_ending).parse(i);

    let (i, seed_to_soil) =
        preceded(gap.and(tag("seed-to-soil map:")).and(line_ending), mappings)(i)?;
    let (i, soil_to_fertilizer) = preceded(
        gap.and(tag("soil-to-fertilizer map:")).and(line_ending),
        mappings,
    )(i)?;
    let (i, fertilizer_to_water) = preceded(
        gap.and(tag("fertilizer-to-water map:")).and(line_ending),
        mappings,
    )(i)?;
    let (i, water_to_light) = preceded(
        gap.and(tag("water-to-light map:")).and(line_ending),
        mappings,
    )(i)?;
    let (i, light_to_temperature) = preceded(
        gap.and(tag("light-to-temperature map:")).and(line_ending),
        mappings,
    )(i)?;
    let (i, temperature_to_humidity) = preceded(
        gap.and(tag("temperature-to-humidity map:"))
            .and(line_ending),
        mappings,
    )(i)?;
    let (i, humidity_to_location) = preceded(
        gap.and(tag("humidity-to-location map:")).and(line_ending),
        mappings,
    )(i)?;
    Ok((
        i,
        FoodProd {
            seeds,
            seed_to_soil: seed_to_soil.into_iter().collect(),
            soil_to_fertilizer: soil_to_fertilizer.into_iter().collect(),
            fertilizer_to_water: fertilizer_to_water.into_iter().collect(),
            water_to_light: water_to_light.into_iter().collect(),
            light_to_temperature: light_to_temperature.into_iter().collect(),
            temperature_to_humidity: temperature_to_humidity.into_iter().collect(),
            humidity_to_location: humidity_to_location.into_iter().collect(),
        },
    ))
}
fn location(seeds: &[Range<u64>], mapping: &HashMap<Range<u64>, Range<u64>>) -> Vec<Range<u64>> {
    let mut result = vec![];
    for seed in seeds {
        let mut seed_map = vec![seed.clone()];
        for src in mapping.keys() {
            let mut tmp = vec![];
            while let Some(seed) = seed_map.pop() {
                //inside
                if src.start <= seed.start && seed.end <= src.end {
                    result.push(
                        mapping[src].start + seed.start - src.start
                            ..mapping[src].start + seed.end - src.start,
                    )
                //contains
                } else if seed.start < src.start && src.end < seed.end {
                    tmp.push(seed.start..src.start);
                    result.push(mapping[src].start..mapping[src].end);
                    tmp.push(src.end..seed.end)
                //left
                } else if seed.start < src.start && src.start < seed.end && seed.end < src.end {
                    tmp.push(seed.start..src.start);
                    result.push(mapping[src].start..mapping[src].start + seed.end - src.start);
                //right
                } else if src.start < seed.start && seed.start < src.end && src.end < seed.end {
                    result.push(mapping[src].start + seed.start - src.start..mapping[src].end);
                    tmp.push(src.end..seed.end);
                //outside
                } else {
                    tmp.push(seed);
                }
            }
            seed_map = tmp.clone();
        }
        result.extend(seed_map);
    }
    result
}
fn part2(food_prod: &FoodProd) -> u64 {
    let mut loc = food_prod.seeds.clone();
    loc = location(&loc, &food_prod.seed_to_soil);
    loc = location(&loc, &food_prod.soil_to_fertilizer);
    loc = location(&loc, &food_prod.fertilizer_to_water);
    loc = location(&loc, &food_prod.water_to_light);
    loc = location(&loc, &food_prod.light_to_temperature);
    loc = location(&loc, &food_prod.temperature_to_humidity);
    loc = location(&loc, &food_prod.humidity_to_location);
    loc.into_iter().map(|range| range.start).min().unwrap()
}

fn main() {
    let input = read_to_string("inputs/day05-input1.txt").unwrap();
    let food_prod = parse(&input).unwrap();
    let answer = part2(&food_prod);
    println!("answer is: {answer}");
}
#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::*;

    const INPUT: &str = r#"
seeds: 79 14 55 13

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
56 93 4
"#;
    #[test]
    fn parsing() -> anyhow::Result<()> {
        let food_production = parse(INPUT.trim())?;
        assert_eq!(food_production.seeds, vec![79..93, 55..68]);
        assert!(food_production.seed_to_soil.contains_key(&(98..100)));
        assert!(food_production.seed_to_soil.contains_key(&(50..98)));
        assert_eq!(food_production.seed_to_soil[&(50..98)], 52..100);
        Ok(())
    }
    #[test]
    fn location_inside() {
        let map = HashMap::from([(18..25, 88..95), (25..95, 18..88)]);
        let seeds = vec![81..95];
        assert_eq!(location(&seeds, &map), vec![74..88]);
    }
    #[test]
    fn location_outside() {
        let map = HashMap::from([(18..25, 88..95), (25..95, 18..88)]);
        let seeds = vec![1..15];
        assert_eq!(location(&seeds, &map), vec![1..15]);
    }
    #[test]
    fn location_contains() {
        let map = HashMap::from([(10..20, 100..120), (40..60, 140..160)]);
        let seeds = vec![30..70];
        assert_eq!(location(&seeds, &map), vec![140..160, 60..70, 30..40]);
    }
    #[test]
    fn location_left() {
        let map = HashMap::from([(10..20, 100..120), (40..60, 140..160)]);
        let seeds = vec![30..50];
        assert_eq!(location(&seeds, &map), vec![140..150, 30..40]);
    }
    #[test]
    fn location_right() {
        let map = HashMap::from([(10..20, 100..120), (40..60, 140..160)]);
        let seeds = vec![50..70];
        assert_eq!(location(&seeds, &map), vec![150..160, 60..70]);
    }
    #[test]
    fn location_multimap() {
        let map = HashMap::from([(5..20, 105..120), (30..40, 130..140), (50..70, 150..170)]);
        let seeds = vec![10..60];
        let mut new_loc = location(&seeds, &map);
        new_loc.sort_by(|a, b| a.start.cmp(&b.start));

        assert_eq!(new_loc, vec![20..30, 40..50, 110..120, 130..140, 150..160]);
    }
    #[test]
    fn part2_test() -> anyhow::Result<()> {
        let food_prod = parse(INPUT.trim())?;
        assert_eq!(part2(&food_prod), 46);
        Ok(())
    }
}
