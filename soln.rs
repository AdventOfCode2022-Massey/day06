// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 6.  
//! Ben and Bart Massey 2022

use std::io::Read;

use aoc::*;

struct RingBuffer<const N: usize, T> {
    state: [T; N],
    offset: usize,
}

impl<const N: usize, T> RingBuffer<N, T> {
    fn new(state: [T; N]) -> Self {
        RingBuffer {
            state,
            offset: 0,
        }
    }

    fn advance(&mut self, val: T) {
        self.state[self.offset] = val;
        self.offset = (self.offset + 1) % N;
    }

    fn contents(&self) -> &[T; N] {
        &self.state
    }
}

fn unique(chars: &[char; 4]) -> bool {
    for (i, c1) in chars.into_iter().enumerate() {
        for c2 in chars[i + 1..].into_iter() {
            if c1 == c2 {
                return false;
            }
        }
    }
    true
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.chars();
    match get_part() {
        Part1 => {
            let init: [char; 4] = input
                .by_ref()
                .take(4)
                .collect::<Vec<char>>()
                .try_into()
                .unwrap();
            let mut buf = RingBuffer::new(init);
            if unique(buf.contents()) {
                println!("4");
                return;
            }
            for (i, ch) in input.enumerate() {
                buf.advance(ch);
                if unique(buf.contents()) {
                    println!("{}", i + 5);
                    return;
                }
            }
            eprintln!("no start found");
        },
        Part2 => todo!(),
    }
}
