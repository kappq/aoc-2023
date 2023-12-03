use std::cmp::max;

use nom::{
    branch::alt, bytes::complete::tag, character::complete::u32, multi::separated_list1, IResult,
    Parser,
};

const NUM_REDS: u32 = 12;
const NUM_GREENS: u32 = 13;
const NUM_BLUES: u32 = 14;

#[derive(Debug)]
struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

struct Cubes {
    num: u32,
    color: CubeColor,
}

enum CubeColor {
    Red,
    Green,
    Blue,
}

fn main() {
    let input = include_str!("input.txt");
    let games = input
        .lines()
        .map(|line| parse_game(line).unwrap().1)
        .collect();

    println!("first star: {}", part1(&games));
    println!("second star: {}", part2(&games));
}

fn part1(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .filter_map(|game| {
            if game.max_red <= NUM_REDS
                && game.max_green <= NUM_GREENS
                && game.max_blue <= NUM_BLUES
            {
                Some(game.id)
            } else {
                None
            }
        })
        .sum()
}

fn part2(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .map(|game| game.max_red * game.max_green * game.max_blue)
        .sum()
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = u32(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, subsets) = separated_list1(tag("; "), parse_subset)(input)?;

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for subset in subsets {
        for cubes in subset {
            match cubes.color {
                CubeColor::Red => max_red = max(max_red, cubes.num),
                CubeColor::Green => max_green = max(max_green, cubes.num),
                CubeColor::Blue => max_blue = max(max_blue, cubes.num),
            }
        }
    }

    Ok((
        input,
        Game {
            id,
            max_red,
            max_green,
            max_blue,
        },
    ))
}

fn parse_subset(input: &str) -> IResult<&str, Vec<Cubes>> {
    let (input, cubes) = separated_list1(tag(", "), parse_cubes)(input)?;

    Ok((input, cubes))
}

fn parse_cubes(input: &str) -> IResult<&str, Cubes> {
    let (input, num) = u32(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, color) = alt((
        tag("red").map(|_| CubeColor::Red),
        tag("green").map(|_| CubeColor::Green),
        tag("blue").map(|_| CubeColor::Blue),
    ))(input)?;

    Ok((input, Cubes { num, color }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = input
            .lines()
            .map(|line| parse_game(line).unwrap().1)
            .collect();

        assert_eq!(part1(&games), 8);
    }

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let games = input
            .lines()
            .map(|line| parse_game(line).unwrap().1)
            .collect();

        assert_eq!(part2(&games), 2286);
    }
}
