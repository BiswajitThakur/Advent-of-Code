fn main(){
    let input = include_str!("./../../input/day1");
    let answer = sum(input);
    println!("Answer: {}", answer);
}
    
fn first_last(input: &str) -> isize {
    let mut beg: usize = 0;
    let mut end: usize = input.len();
    if end < 1 {
        return 0;
    };
    end -= 1;
    while beg < end {
        if input.as_bytes()[beg].is_ascii_digit()  {
            break;
        }
        beg += 1;
    }
    if beg > end {
        return 0;
    }
    while end >= beg {
        if input.as_bytes()[end].is_ascii_digit() {
            break;
        }
        end -= 1;
    }
    let first_digit = (input.as_bytes()[beg] - b'0') as isize;
    let last_digit = (input.as_bytes()[end] - b'0') as isize;
    (10*first_digit) + last_digit
}
    
fn sum(input: &str) -> isize {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut result: isize = 0;
    for i in lines {
        result += first_last(i);
    }
    result
}
    
#[cfg(test)]
mod tests {
    use first_last;
    #[test]
    fn test_1(){
        assert_eq!(first_last(&"qgsgg7heh5"), 75);
    }
}
