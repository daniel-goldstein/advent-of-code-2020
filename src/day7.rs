use crate::utils::read_input_to_string;
use regex::Regex;
use std::collections::{HashMap, HashSet};

type BagGraph<'a> = HashMap<&'a str, HashSet<(String, u32)>>;

pub fn day7() {
    println!("DAY 7");
    let input = read_input_to_string(String::from("day7"));
    let graph = read_input(&input);
    println!("{:#?}", graph.get("shiny gold").unwrap());
    println!(
        "There are {} many bags that can contain a shiny gold bag",
        how_many_can_contain(&graph, "shiny gold")
    );
    println!(
        "There are {} bags inside a shiny gold bag",
        how_many_inside(&graph, "shiny gold")
    );
}

fn how_many_can_contain(graph: &BagGraph, bag: &str) -> usize {
    graph
        .keys()
        .filter(|parent| can_contain(graph, parent, bag))
        .count()
        - 1
}

fn how_many_inside(graph: &BagGraph, bag: &str) -> u32 {
    graph
        .get(bag)
        .unwrap()
        .iter()
        .map(|(c, count)| count + count * how_many_inside(graph, c))
        .sum()
}

fn can_contain(graph: &BagGraph, parent: &str, bag: &str) -> bool {
    *parent == *bag
        || graph
            .get(parent)
            .unwrap()
            .iter()
            .any(|(c, _)| can_contain(graph, c, bag))
}

fn read_input(input: &str) -> BagGraph {
    let mut graph: BagGraph = HashMap::new();
    let bag_regex = Regex::new(r"(\d) (\w+ \w+)").unwrap();

    for line in input.lines() {
        let rule: Vec<&str> = line.split(" bags contain ").collect();
        let bag = rule[0];
        let contained = bag_regex
            .captures_iter(rule[1])
            .map(|cap| (String::from(&cap[2]), cap[1].parse::<u32>().unwrap()))
            .collect();
        graph.insert(bag, contained);
    }

    graph
}
