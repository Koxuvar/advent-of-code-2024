use std::collections::HashMap;

fn make_lists(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut list1: Vec<u32> = Vec::new();
    let mut list2: Vec<u32> = Vec::new();

    for line in input.lines() {
        let l1 = line.split("   ").nth(0).unwrap().parse::<u32>();

        let l2 = line.split("   ").nth(1).unwrap().parse::<u32>();

        list1.push(l1.unwrap());
        list2.push(l2.unwrap());
    }

    list1.sort();
    list2.sort();

    (list1, list2)
}

fn compare_lists(list1: Vec<u32>, list2: Vec<u32>) -> i32 {
    let mut sum: i32 = 0;

    if list1.len() as u32 == list2.len() as u32 {
        for (i, _num) in list1.iter().enumerate() {
            sum = sum as i32 + (list2[i] as i32 - list1[i] as i32).abs();
        }
    }

    sum
}

fn calc_list_similarity(list1: Vec<u32>, list2: Vec<u32>) -> u32 {
    let mut result: u32 = 0;

    let mut countmap: HashMap<u32, u32> = HashMap::new();
    let mut appeamap: HashMap<u32, u32> = HashMap::new();

    for i in &list1 {
        let total = list1.iter().filter(|n| *n == i).count() as u32;
        countmap.entry(*i).or_insert(total);
    }

    for i in &list2 {
        let total = list2.iter().filter(|n| *n == i).count() as u32;
        appeamap.entry(*i).or_insert(total);
    }

    for (k, v) in countmap.into_iter() {
        result += v * (k * appeamap.get(&k).unwrap_or(&0));
    }

    result
}

pub fn find_solution() {
    let contents = std::fs::read_to_string("src/inputs/01/day01.txt");
    let lists = make_lists(&contents.unwrap());

    println!(
        "Total Distance: {}",
        compare_lists(lists.0.clone(), lists.1.clone())
    );

    println!(
        "Total Similarity Score: {}",
        calc_list_similarity(lists.0, lists.1)
    );
}
