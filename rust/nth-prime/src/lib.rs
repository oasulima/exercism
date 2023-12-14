fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    } else if num <= 3 {
        return true;
    } else if num % 2 == 0 || num % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    return true;
}

pub fn nth(n: u32) -> u32 {
    let n = n + 1;
    let mut count = 0;
    let mut num: u32 = 2;
    while count < n {
        if is_prime(num) {
            count += 1;
        }
        if count == n {
            break;
        }
        num += 1;
    }
    num
}
