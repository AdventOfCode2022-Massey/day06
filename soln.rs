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

fn unique<const N: usize>(chars: &[char; N]) -> bool {
    let h: std::collections::HashSet<char> = chars
        .iter()
        .copied()
        .collect();
    h.len() == N
}

fn solve<const N: usize>() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.chars();
    let init: [char; N] = input
        .by_ref()
        .take(N)
        .collect::<Vec<char>>()
        .try_into()
        .unwrap();
    let mut buf = RingBuffer::new(init);
    if unique(buf.contents()) {
        println!("{}", N);
        return;
    }
    for (i, ch) in input.enumerate() {
        buf.advance(ch);
        if unique(buf.contents()) {
            println!("{}", i + N + 1);
            return;
        }
    }
    eprintln!("no start found");
}

fn main() {
    match get_part() {
        Part1 => solve::<4>(),
        Part2 => solve::<14>(),
    }
}
