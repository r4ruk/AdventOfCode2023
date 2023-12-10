use std::collections::HashMap;

/// function calculating the lowest common multiplier from a set of numbers
pub fn lcm(numbers: Vec<i128>)-> i128  {
    let mut lcm_factors = HashMap::new();

    for num in numbers {
        let factors = prime_factors(num);

        for (prime, power) in factors {
            let max_power = lcm_factors.entry(prime).or_insert(power);
            *max_power = (*max_power).max(power);
        }
    }

    let mut lcm = 1;

    for (prime, power) in lcm_factors {
        lcm *= prime.pow(power as u32);
    }

    lcm
}

/// function returns a hashmap of prime factors and its corresponding power for the given number
pub fn prime_factors(n: i128) -> HashMap<i128, i128> {
    let mut factors = HashMap::new();
    let mut num = n;
    let mut divisor = 2;

    while num > 1 {
        while num % divisor == 0 {
            *factors.entry(divisor).or_insert(0) += 1;
            num /= divisor;
        }
        divisor += 1;
    }

    factors
}