// helpers

pub fn is_it_pow(input: usize, mut candidate: usize) -> bool {
    let original_candidiate = candidate;
    if input < candidate {
        return false;
    }
    loop {
        if input == candidate {
            return true;
        }
        candidate *= original_candidiate;
        if input < candidate {
            return false;
        }
    }
}
