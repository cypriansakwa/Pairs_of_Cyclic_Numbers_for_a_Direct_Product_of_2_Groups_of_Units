fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn is_coprime(a: u32, b: u32) -> bool {
    gcd(a, b) == 1
}

fn phi(mut n: u32) -> u32 {
    let mut result = n;
    let mut p = 2;
    while p * p <= n {
        if n % p == 0 {
            while n % p == 0 {
                n /= p;
            }
            result -= result / p;
        }
        p += 1;
    }
    if n > 1 {
        result -= result / n;
    }
    result
}

fn is_cyclic(n: u32) -> bool {
    if n == 1 || n == 2 || n == 4 {
        return true;
    }
    if n % 2 == 0 {
        let m = n / 2;
        return m == 1 || (m % 2 != 0 && is_prime(m));
    }
    is_prime(n)
}

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn find_cyclic_pairs(limit: u32) -> Vec<(u32, u32)> {
    let mut pairs = Vec::new();
    for n in 2..=limit {
        for m in 2..=limit {
            if is_cyclic(n) && is_cyclic(m) && is_coprime(phi(n), phi(m)) {
                pairs.push((n, m));
            }
        }
    }
    pairs
}

fn main() {
    let limit = 400; // Adjust the limit as needed
    let pairs = find_cyclic_pairs(limit);
    for (n, m) in pairs {
        println!("({}, {})", n, m);
    }
}






