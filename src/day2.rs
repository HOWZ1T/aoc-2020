use std::ops::Index;

#[derive(Debug)]
struct Policy {
    min: i32,
    max: i32,
    chr: char
}

impl Policy {
    fn from_string(str: &str) -> Result<Policy, &'static str> {
        let mut parts: Vec<&str> = str.trim().split(' ').collect();
        if parts.len() != 2  {
            return Err("malformed data");
        }

        let range_parts: Vec<&str> = parts[0].trim().split('-').collect();
        let min = range_parts[0].parse::<i32>().unwrap();
        let max = range_parts[1].parse::<i32>().unwrap();
        let chr = parts[1].chars().next().unwrap();

        Ok(Policy { min, max, chr })
    }

    fn is_pwd_valid(&self, pwd: &str) -> bool {
        let mut count = 0;
        for chr in pwd.chars() {
            if self.chr == chr { count += 1; }
        }

        if count < self.min || count > self.max {return false;}
        return true;
    }

    fn is_pwd_valid_corp(&self, pwd: &str) -> bool {
        let chars: Vec<char> = pwd.chars().collect();
        if (self.max as usize) > chars.len() {return false;}

        return
            (chars[(self.min-1) as usize] == self.chr
            || chars[(self.max-1) as usize] == self.chr)
            && chars[(self.min-1) as usize] != chars[(self.max-1) as usize];
    }
}

pub fn part1(input: Vec<String>) -> i32 {
    let mut count = 0;
    input.iter().for_each(|val| {
        let parts: Vec<&str> = val.trim().split(':').collect();
        let pwd = parts[1].trim();
        let policy = Policy::from_string(parts[0].trim()).unwrap();
        if policy.is_pwd_valid(pwd) { count += 1; }
    });

    return count;
}

pub fn part2(input: Vec<String>) -> i32 {
    let mut count = 0;
    input.iter().for_each(|val| {
        let parts: Vec<&str> = val.trim().split(':').collect();
        let pwd = parts[1].trim();
        let policy = Policy::from_string(parts[0].trim()).unwrap();
        if policy.is_pwd_valid_corp(pwd) {count += 1;}
    });

    return count;
}