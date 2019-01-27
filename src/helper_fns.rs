pub fn balanced_delimiters(code: &str) -> bool {
    let mut type1: i32 = 0;
    let mut type2: i32 = 0;
    let mut type3: i32 = 0;
    for c in code.chars() {
        match c {
            '(' => type1+=1,
            ')' => type1-=1,
            '{' => type2+=1,
            '}' => type2-=1,
            '[' => type3+=1,
            ']' => type3-=1,
            _ => (),
        }
    }
    type1 == 0 && type2 == 0 && type3 == 0
}