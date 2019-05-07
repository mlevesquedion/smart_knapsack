use std::cmp;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Item {
    weight: usize,
    value: usize,
}

fn dp_knapsack(items: &[Item], capacity: usize) -> Vec<Item> {
    let mut table = vec![vec![0; capacity + 1]; items.len() + 1];

    let mut computations = 0;
    for i in 1..(items.len() + 1) {
        for j in 1..(capacity + 1) {
            let mut maximum_value = table[i - 1][j];
            let item_weight = items[i - 1].weight;
            if item_weight <= j {
                maximum_value = cmp::max(
                    maximum_value,
                    items[i - 1].value + table[i - 1][j - item_weight],
                )
            }
            table[i][j] = maximum_value;
            computations += 1;
        }
    }

    let mut items_taken = Vec::new();
    let mut remaining_capacity = capacity;
    for i in (1..items.len() + 1).rev() {
        if table[i][remaining_capacity] == table[i - 1][remaining_capacity] {
            continue;
        }
        items_taken.push(items[i - 1]);
        remaining_capacity -= items[i - 1].weight;
    }

    println!("Computations: {}", computations);
    items_taken
}

fn smart_dp_knapsack(items: &[Item], capacity: usize) -> Vec<Item> {
    let mut table = vec![vec![0; capacity + 1]; items.len() + 1];
    let mut computed = vec![vec![false; capacity + 1]; items.len() + 1];

    for i in 0..(items.len() + 1) {
        computed[i][0] = true;
    }

    for i in 0..(capacity + 1) {
        computed[0][i] = true;
    }

    let mut stack = vec![(items.len(), capacity)];
    let mut computations = 0;
    while !stack.is_empty() {
        let (i, j) = stack.pop().unwrap();
        if computed[i][j] {
            continue;
        }
        let item_weight = items[i - 1].weight;
        let without = (i - 1, j);
        if item_weight <= j {
            let with = (i - 1, j - item_weight);
            if computed[without.0][without.1] && computed[with.0][with.1] {
                table[i][j] = cmp::max(
                    table[without.0][without.1],
                    items[i - 1].value + table[with.0][with.1],
                );
                computed[i][j] = true;
                computations += 1;
            } else {
                stack.push((i, j));
                stack.push(without);
                stack.push(with);
            }
        } else {
            if computed[without.0][without.1] {
                table[i][j] = table[without.0][without.1];
                computed[i][j] = true;
                computations += 1;
            } else {
                stack.push((i, j));
                stack.push(without);
            }
        }
    }

    let mut items_taken = Vec::new();
    let mut remaining_capacity = capacity;
    for i in (1..items.len() + 1).rev() {
        if table[i][remaining_capacity] == table[i - 1][remaining_capacity] {
            continue;
        }
        items_taken.push(items[i - 1]);
        remaining_capacity -= items[i - 1].weight;
    }

    println!("Computations: {}", computations);
    items_taken
}

extern crate rand;

use rand::{thread_rng, Rng};

fn get_random_parameters() -> (usize, Vec<Item>) {
    let capacity = thread_rng().gen_range(100, 1000);
    let mut items = Vec::new();

    for _ in 0..1000 {
        let weight = thread_rng().gen_range(1, 100);
        let value = thread_rng().gen_range(1, capacity / 4);
        items.push(Item { weight, value });
    }

    (capacity, items)
}

fn main() {
    let items = vec![
        Item {
            weight: 5,
            value: 10,
        },
        Item {
            weight: 4,
            value: 40,
        },
        Item {
            weight: 6,
            value: 30,
        },
        Item {
            weight: 3,
            value: 50,
        },
        Item {
            weight: 6,
            value: 20,
        },
        Item {
            weight: 6,
            value: 20,
        },
        Item {
            weight: 6,
            value: 20,
        },
        Item {
            weight: 6,
            value: 20,
        },
        Item {
            weight: 6,
            value: 20,
        },
        Item {
            weight: 6,
            value: 20,
        },
    ];

    assert_eq!(
        dp_knapsack(&items, 10),
        vec![
            Item {
                weight: 3,
                value: 50
            },
            Item {
                weight: 4,
                value: 40
            },
        ]
    );
    assert_eq!(
        smart_dp_knapsack(&items, 10),
        vec![
            Item {
                weight: 3,
                value: 50
            },
            Item {
                weight: 4,
                value: 40
            },
        ]
    );

    let (capacity, items) = get_random_parameters();
    dp_knapsack(&items, capacity);
    smart_dp_knapsack(&items, capacity);
}
