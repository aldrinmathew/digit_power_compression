use core::fmt;
use std::collections::HashMap;

struct ExponentValue {
    exponent: u8,
    value: u64,
}

struct NumberMap {
    number: u8,
    powers: Vec<ExponentValue>,
}

impl fmt::Display for ExponentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("PowerPair { exp: ")?;
        f.write_str(self.exponent.to_string().as_str())?;
        f.write_str(", value: ")?;
        f.write_str(self.value.to_string().as_str())?;
        f.write_str(" }")?;
        Ok(())
    }
}

impl fmt::Display for NumberMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NumValues { num: ")?;
        f.write_str(self.number.to_string().as_str())?;
        f.write_str(", powers: vec![")?;
        for it in self.powers.iter() {
            it.fmt(f)?;
            f.write_str(", ")?;
        }
        f.write_str("] }")?;
        Ok(())
    }
}

fn main() {
    let mut num_list: Vec<NumberMap> = vec![];
    let mut hashed_powers = HashMap::<u64, (u8, u8)>::new();
    for i in 1..255u8 {
        let mut num_val = NumberMap {
            number: i,
            powers: vec![],
        };
        for j in 1..255u8 {
            let pow_val = (i as u64).checked_pow(j as u32);
            if pow_val.is_none() {
                break;
            }
            num_val.powers.push(ExponentValue {
                exponent: j,
                value: pow_val.unwrap(),
            });
            hashed_powers.insert(pow_val.unwrap(), (i, j));
            if i == 1 {
                break;
            }
        }
        num_list.push(num_val);
    }
    let mut unfound_values = Vec::<u64>::new();
    let mut max_cmp_factor = 0usize;
    let mut total_cmp_factor = 0f64;
    let start = 0u64;
    let end = 10000000u64;
    println!("Calculating {} -> {}", start, end);
    for candidate in start..end {
        let res = find_pairs(candidate, &hashed_powers);
        if res.is_empty() {
            unfound_values.push(candidate);
        } else {
            if res.len() > max_cmp_factor {
                max_cmp_factor = res.len();
            }
            total_cmp_factor += res.len() as f64 / 4f64;
        }
        print!(
            "\r{:.2} %",
            ((candidate - start) as f64 / (end - start) as f64) * 100f64
        );
    }
    println!("");
    println!("Maximum compression factor: {} / 4", max_cmp_factor);
    println!(
        "Average compression factor: {}",
        (total_cmp_factor) / ((end - start) as f64)
    );
    if unfound_values.is_empty() {
        println!("All values successfully processed!");
    } else {
        print!("Could not find sequence for ");
        for it in unfound_values.iter() {
            print!("{}, ", it);
        }
        println!("");
    }
}

fn find_pairs<'a>(num: u64, lookup: &'a HashMap<u64, (u8, u8)>) -> Vec<(u8, u8)> {
    if num == 0 {
        return vec![(0, 0)];
    }
    if lookup.contains_key(&num) {
        return vec![lookup[&num]];
    }
    let mut res = Vec::<(u8, u8)>::new();
    let mut shift;
    let mut num_backup = num;
    let mut value = num;
    while num_backup != 0 {
        shift = 0;
        while value != 0 && !lookup.contains_key(&value) {
            value /= 10;
            shift += 1;
        }
        if value == 0 {
            return vec![];
        }
        res.push(lookup[&value]);
        num_backup = num_backup - (value * 10u64.pow(shift));
        if num_backup == 0 && shift > 0 {
            res.push((0, shift as u8));
        } else if shift > 0 && num_backup < 10u64.pow(shift - 1) {
            let mut temp_pow = 1u8;
            while (shift - (temp_pow as u32)) > 0
                && num_backup < 10u64.pow(shift - (temp_pow as u32))
            {
                temp_pow += 1;
            }
            res.push((0, temp_pow - 1));
        }
        value = num_backup;
    }
    return res;
}
