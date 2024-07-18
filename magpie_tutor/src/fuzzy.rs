//! Simple implementation for simple fuzzy sorting

use core::f32;
use std::cmp::{max, min};
use std::collections::VecDeque;

pub struct FuzzyRes<'a, T> {
    pub rank: f32,
    pub data: &'a T,
}

/// Return `(best, rest)`, `rest` won't contain `best`
pub fn fuzzy_rank<'a, T, F>(
    value: &str,
    vec: Vec<&'a T>,
    mut f: F,
) -> (FuzzyRes<'a, T>, Vec<FuzzyRes<'a, T>>)
where
    F: FnMut(&T) -> &str,
{
    let mut out: VecDeque<FuzzyRes<T>> = VecDeque::new();

    for v in vec {
        let res = FuzzyRes {
            rank: lev(f(v).to_lowercase().as_str(), value.to_lowercase().as_str()),
            data: v,
        };

        let index = out
            .binary_search_by(|x: &FuzzyRes<T>| (x.rank.total_cmp(&res.rank).reverse()))
            .unwrap_or_else(|e| e);

        out.insert(index, res);
    }

    let mut out = out.into_iter();

    (out.next().unwrap(), out.collect())
}

/// Normalize levenshtein distance
///
/// https://github.com/TheAlgorithms/Rust/blob/master/src/string/levenshtein_distance.rs
fn lev(string1: &str, string2: &str) -> f32 {
    if string1.is_empty() {
        return string2.len() as f32;
    }
    let l1 = string1.len();
    let mut prev_dist: Vec<usize> = (0..=l1).collect();

    for (row, c2) in string2.chars().enumerate() {
        // we'll keep a reference to matrix[i-1][j-1] (top-left cell)
        let mut prev_substitution_cost = prev_dist[0];
        // diff with empty string, since `row` starts at 0, it's `row + 1`
        prev_dist[0] = row + 1;

        for (col, c1) in string1.chars().enumerate() {
            // "on the left" in the matrix (i.e. the value we just computed)
            let deletion_cost = prev_dist[col] + 1;
            // "on the top" in the matrix (means previous)
            let insertion_cost = prev_dist[col + 1] + 1;
            let substitution_cost = if c1 == c2 {
                // last char is the same on both ends, so the min_distance is left unchanged from matrix[i-1][i+1]
                prev_substitution_cost
            } else {
                // substitute the last character
                prev_substitution_cost + 1
            };
            // save the old value at (i-1, j-1)
            prev_substitution_cost = prev_dist[col + 1];
            prev_dist[col + 1] = min(substitution_cost, min(deletion_cost, insertion_cost));
        }
    }
    let max = max(string1.len(), string2.len());
    (max - prev_dist[l1]) as f32 / max as f32
}