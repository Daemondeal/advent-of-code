#![allow(dead_code)]
use std::vec::Vec;

fn main() {
    let input = include_str!("input.txt");

    // println!("A: {}", solve_a(input));
    solve_b(input);
}

fn solve_a(input: &str) -> i32 {
    let layers = load_image(input, 25, 6);

    let min_index = layers.iter()
        .map(|layer| {
            layer.iter()
                .map(|h| h.iter().fold(0, |state, x| {
                    if *x == 0 {
                        return state + 1;
                    }

                    state
                }))
                .sum::<u32>()
        })
        .enumerate()
        .min_by_key(|(_, a)| *a)
        .unwrap().0;

    let ones = layers[min_index].iter()
        .map(|h| h.iter().fold(0, |state, x| if *x == 1 { state + 1 } else { state }))
        .sum::<i32>();

    let twos = layers[min_index].iter()
        .map(|h| h.iter().fold(0, |state, x| if *x == 2 { state + 1 } else { state }))
        .sum::<i32>();

    ones * twos
}

fn solve_b(input: &str) {
    let layers = load_image(input, 25, 6);
    let mut image: [[u32; 25]; 6] = [[0; 25]; 6];

    for layer in layers.iter().rev() {
        for (i, row) in layer.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell != 2 {
                    image[i][j] = *cell;
                }
            }
        }
    }

    for row in image {
        for cell in row {
            print!("{} ", if cell == 1 { "#" } else { " " });
        }
        println!();
    }
}

fn load_image(input: &str, width: usize, height: usize) -> Vec<Vec<Vec<u32>>> {
    let chars = input.chars().collect::<Vec<char>>();
    let horizontal_chunks = chars.chunks(width);
    
    let mut layers: Vec<Vec<Vec<u32>>> = vec![vec![]];
    let mut cur_layer = 0;


    for slice in horizontal_chunks {
        if layers[cur_layer].len() >= height {
            layers.push(vec![]);
            cur_layer += 1;
        }

        layers[cur_layer].push(
            slice.iter()
                .map(|x| x.to_digit(10))
                .flatten()
                .collect()
        );

    } 

    layers
}
