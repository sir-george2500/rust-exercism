use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut unique_multiples = HashSet::new(); // Using HashSet to store unique multiples
    for &factor in factors {
        for i in (factor..limit).step_by(factor as usize) {
            unique_multiples.insert(i); // Insert unique multiples into the HashSet
        }
    }
    unique_multiples.iter().sum()
}
