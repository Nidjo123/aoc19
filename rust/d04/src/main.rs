const LOW: u32 = 134792;
const HIGH: u32 = 675810;

fn has_same_adjacent_digits(mut p: u32, just_two: bool) -> bool {
    while p != 0 {
	let (x, y, z) = (p % 10, p / 10 % 10, p / 100 % 10);
	if !just_two && x == y {
	    return true;
	} else if just_two {
	    if x == y && y != z {
		return true;
	    } else {
		while p != 0 && p % 10 == p / 10 % 10 {
		    p /= 10;
		}
	    }
	}
	p /= 10;
    }
    false
}

fn never_decreases(mut p: u32) -> bool {
    while p != 0 {
	let (x, y) = (p / 10 % 10, p % 10);
	if x > y {
	    return false;
	}
	p /= 10;
    }
    true
}

fn is_valid_password(p: u32, just_two: bool) -> bool {
    let six_digit = p > 99999 && p < 1000000;
    let within_range = p >= LOW && p <= HIGH;
    let same_adjacent_digits = has_same_adjacent_digits(p, just_two);
    let increasing = never_decreases(p);

    let conditions = [six_digit, within_range, same_adjacent_digits, increasing];
    conditions.iter()
	.all(|x| *x)
}

fn main() {
    let valid_passwords = (LOW..=HIGH).filter(|p| is_valid_password(*p, false))
	.count();
    
    println!("Part 1: {}", valid_passwords);

    let valid_passwords = (LOW..=HIGH).filter(|p| is_valid_password(*p, true))
	.count();
    println!("Part 2: {}", valid_passwords);
}
