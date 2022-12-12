/// In modular arithmetic, gets the product of number a and b,
/// for a specific modulus.
///
/// Source: https://en.wikipedia.org/wiki/Modular_arithmetic#Properties
pub fn multiplication (a: u64, b: u64, modulus: u64) -> u64 {
    let mut a = a;
    let mut b = b;

    if a >= modulus {
        a %= modulus;
    }

    if b >= modulus {
        b %= modulus;
    }

    let x = a;
    let c = x * b / modulus;
    let r: i64 = (a * b - c * modulus) as i64 % modulus as i64;

    if r < 0 {
        return (r + modulus as i64) as u64;
    }

    r as u64
}

/// In modular arithmetic, elevate n to the specified exponent,
/// for a specific modulus.
///
/// Source: https://en.wikipedia.org/wiki/Modular_arithmetic#Properties
pub fn power(n: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;
    let mut a = n;
    let mut b = exponent;

    while b > 0 {
        if b & 1 != 0 {
            result = multiplication(result, a, modulus);
        }
        b = b >> 1;
        a = multiplication(a, a, modulus);
    }

    result
}
