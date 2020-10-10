use std::collections::HashMap;
fn main() {
    //let vec = [A, B, C, D, E, F, G, H, I];

    let cit = vec![1, 4, 1, 4, 2, 1, 3, 5, 6];

    // Get highest number of citations
    // 6 -> 1, 5 -> 2,4 -> 4, 3 -> 5, 2 -> 6, 1 -> 9

    let cit2 = cit.clone();

    println!(
        "Max h-value is {:?}",
        extract_h_index(create_hash_map(cit2, 1))
    );
}
// Create Hashmap containing number of citations => number of elements with same or greater number of citations
fn create_hash_map(v1: Vec<u32>, n: u32) -> HashMap<usize, usize> {
    let mut h = HashMap::new();
    let v2 = v1.clone();
    v1.into_iter()
        .map(|s| {
            h.insert(
                s as usize,
                v2.clone()
                    .into_iter()
                    .filter(|x| x >= &s)
                    .collect::<Vec<u32>>()
                    .len(),
            )
        })
        .collect::<Vec<_>>();
    filter_hashmap(h)
}
// Filter out hashmap entries that have value less than key
fn filter_hashmap(h: HashMap<usize, usize>) -> HashMap<usize, usize> {
    h.into_iter()
        .filter(|(k, v)| v >= k)
        .collect::<HashMap<usize, usize>>()
}

// Convert Hashmap to vector and find max h-index
fn extract_h_index(h: HashMap<usize, usize>) -> (usize, usize) {
    let v: Vec<(&usize, &usize)> = h.iter().collect::<Vec<(&usize, &usize)>>();
    let (a, b) = *v.iter().max_by(|a, b| b.1.cmp(&a.1)).unwrap();
    (*a, *b)
}
