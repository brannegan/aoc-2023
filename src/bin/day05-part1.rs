use std::collections::HashMap;
use std::fs::read_to_string;
use std::ops::Range;

use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1, u64};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::{Finish, IResult, Parser};

struct FoodProd {
    seeds: Vec<u64>,
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
    let (i, seeds) = preceded(tag("seeds: "), separated_list1(space1, u64))(input)?;
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
fn location(seed: u64, mapping: &HashMap<Range<u64>, Range<u64>>) -> u64 {
    mapping
        .keys()
        .find(|src| src.contains(&seed))
        .map(|src| mapping[src].start + seed - src.start)
        .unwrap_or(seed)
}
fn part1(food_prod: &FoodProd) -> u64 {
    food_prod
        .seeds
        .iter()
        .copied()
        .map(|seed| location(seed, &food_prod.seed_to_soil))
        .map(|soil| location(soil, &food_prod.soil_to_fertilizer))
        .map(|fert| location(fert, &food_prod.fertilizer_to_water))
        .map(|wate| location(wate, &food_prod.water_to_light))
        .map(|ligh| location(ligh, &food_prod.light_to_temperature))
        .map(|temp| location(temp, &food_prod.temperature_to_humidity))
        .map(|humi| location(humi, &food_prod.humidity_to_location))
        .min()
        .unwrap()
}

fn main() {
    let input = read_to_string("inputs/day05-input1.txt").unwrap();
    let food_prod = parse(&input).unwrap();
    let answer = part1(&food_prod);
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
        assert_eq!(food_production.seeds, vec![79, 14, 55, 13]);
        assert!(food_production.seed_to_soil.contains_key(&(98..100)));
        assert!(food_production.seed_to_soil.contains_key(&(50..98)));
        assert_eq!(food_production.seed_to_soil[&(50..98)], 52..100);
        Ok(())
    }
    #[test]
    fn location_test() {
        let map = HashMap::from([(98..100, 50..52), (50..98, 52..100)]);
        assert_eq!(location(79, &map), 81);
        assert_eq!(location(14, &map), 14);
        assert_eq!(location(55, &map), 57);
        assert_eq!(location(13, &map), 13);
    }
    #[test]
    fn part1_test() -> anyhow::Result<()> {
        let food_prod = parse(INPUT.trim())?;
        assert_eq!(part1(&food_prod), 35);
        Ok(())
    }
}
