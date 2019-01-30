// helpers

pub fn is_it_pow(input: usize, mut candidate: usize) -> bool {
    let original_candidate = candidate;
    if input < candidate {
        return false;
    }
    loop {
        if input == candidate {
            return true;
        }
        candidate *= original_candidate;
        if input < candidate {
            return false;
        }
    }
}
