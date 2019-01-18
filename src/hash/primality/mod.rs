use num::bigint::{BigInt, BigUint, Sign, ToBigInt, ToBigUint};
use num_traits::cast::ToPrimitive;

mod utils;

macro_rules! bu {
  ($x:expr) => {
    //BigUint::from($x as u64)
    ToBigUint::to_biguint(&($x as u64)).unwrap()
  };
}

macro_rules! bi {
  ($x:expr) => {
    //BigInt::from($x as i64)
    //($x as i64).to_bigint()
    ToBigInt::to_bigint(&($x as i64)).unwrap()
  };
}

// Baillie-PSW probabilistic primality test:
// 1. Filter composites with small divisors.
// 2. Do Miller-Rabin base 2.
// 3. Filter squares.
// 4. Do Lucas.
#[allow(dead_code)]
pub fn is_prob_prime(n: &BigUint) -> bool {
  if has_small_prime_factor(n) {
    return false;
  }
  if !passes_miller_rabin_base_2(n) {
    return false;
  }
  if is_prob_square(n) {
    return false;
  } // FIX
  let d = choose_d(n);
  passes_lucas(n, &d)
}

#[allow(dead_code)]
fn has_small_prime_factor(n: &BigUint) -> bool {
  for &divisor in utils::SMALL_PRIMES.iter() {
    let divisor = &bu!(divisor);
    if divisor > n {
      break;
    }
    if n % divisor == bu!(0) {
      return true;
    }
  }
  false
}

#[allow(dead_code)]
fn passes_miller_rabin_base_2(n: &BigUint) -> bool {
  // write n-1 = 2^r * d
  let mut d = n - 1u64;
  let mut r = 0;
  while &d % 2u64 == bu!(0) {
    d /= 2u64;
    r += 1;
  }
  // println!("{} = 2^{} * {}", n, r, d);
  let mut x = bu!(2).modpow(&d, n);
  if x == bu!(1) || x == n - &bu!(1) {
    return true;
  }
  for _ in 0..(r - 1) {
    x = x.modpow(&bu!(2), n);
    if x == bu!(1) {
      return false;
    }
    if x == n - &bu!(1) {
      return true;
    }
  }
  false
}

fn is_prob_square(n: &BigUint) -> bool {
  // Step 1
  let zero = bu!(0);
  let one = bu!(1);
  if n & bu!(2) != zero || n & bu!(7) == bu!(5) || n & bu!(11) == bu!(8) {
    return false;
  }
  // Maybe unneccessary
  if *n == zero {
    return true;
  }

  println!("Step 2");

  // Step 2
  let copy = n.clone();
  let copy = (copy.clone() & bu!(4_294_967_295)) + (copy >> 32);
  let copy = (copy.clone() & bu!(65535)) + (copy >> 16);
  let copy = (copy.clone() & bu!(255)) + ((copy.clone() >> 8) & bu!(255)) + (copy >> 16);
  // println!("{}", n.to_u64().unwrap());
  if utils::BAD_255[copy.to_u64().unwrap() as usize] {
    return false;
  }

  println!("Step 3");

  let mut x = n.clone();
  if x.clone() & bu!(4_294_967_295) == zero {
    x >>= 32;
  }
  if x.clone() & bu!(65535) == zero {
    x >>= 16;
  }
  if x.clone() & bu!(255) == zero {
    x >>= 8;
  }
  if x.clone() & bu!(15) == zero {
    x >>= 4;
  }
  if x.clone() & bu!(3) == zero {
    x >>= 2;
  }
  if x.clone() & bu!(7) != one {
    return false;
  }

  println!("Step 4");

  // let mut r: i64 = start[((n >> 3) & bu!(1023 as u64)).to_u64().unwrap() as usize];
  // let mut t: BigInt;
  // let mut z: BigInt;
  // let zero_i = BigInt::from(0 as i8);
  // while {
  //   z = BigInt::from(x.clone()) - BigInt::from(r * r);
  //   if z == zero_i {
  //     return true;
  //   }
  //   t = z.clone() & -z.clone();
  //   r += ((z & t.clone()) >> 1).to_i64().unwrap();
  //   if r > (t.clone() >> 1).to_i64().unwrap() {
  //     r = t.to_i64().unwrap() - r;
  //   }
  //   t <= BigInt::from(1 << 33)
  // } {}
  // println!("All else fails");

  //0xC840C04048404040
  // let inbase16 = &[12, 8, 4, 0, 12, 0, 4, 0, 4, 8, 4, 0, 4, 0, 4, 0];
  // let good_mask = BigUint::from_radix_be(inbase16, 16).unwrap();
  // if good_mask << n >= zero {
  //   return false;
  // }
  true
}

// find first D in [5, -7, 9, ...] for which Jacobi symbol (D/n) = -1
fn choose_d(n: &BigUint) -> BigInt {
  let n_signed = &BigInt::from_biguint(Sign::Plus, n.clone());
  let mut d = bi!(5);
  while jacobi_symbol(&d, n_signed) != -1 {
    if d > bi!(0) {
      d += bi!(2);
    } else {
      d -= bi!(2);
    }
    d *= bi!(-1);
  }
  d
}

// Jacobi symbol (a/n)
fn jacobi_symbol(a: &BigInt, n: &BigInt) -> i64 {
  // unfortunately cannot be written as large match block since BigUint::from is not pattern
  if n == &bi!(1) {
    return 1;
  }
  if a == &bi!(0) {
    return 0;
  } else if a == &bi!(1) {
    return 1;
  } else if a == &bi!(2) {
    let n_mod_8 = n % 8;
    if n_mod_8 == bi!(3) || n_mod_8 == bi!(5) {
      return -1;
    } else if n_mod_8 == bi!(1) || n_mod_8 == bi!(7) {
      return 1;
    }
  } else if *a < bi!(0) {
    // return (-1)^((n-1)/2) (-a/n)
    let j = jacobi_symbol(&(a * &bi!(-1)), n);
    let exp_mod_2 = ((n - bi!(1)) / bi!(2)) % 2;
    if exp_mod_2 == bi!(0) {
      return j;
    } else {
      return -j;
    }
  }
  if a % 2 == bi!(0) {
    return jacobi_symbol(&bi!(2), n) * jacobi_symbol(&(a / &bi!(2)), n);
  } else if &(a % n) != a {
    return jacobi_symbol(&(a % n), n);
  }
  0
}

#[allow(dead_code)]
fn passes_lucas(_n: &BigUint, _d: &BigInt) -> bool {
  // let p = 1;
  // let q = (1 - d) / 4;
  false
}

#[cfg(test)]
mod tests {
  use super::*;

  fn isqrt(n: usize) -> usize {
    n == 0 && return n;
    let mut s = (n as f64).sqrt() as usize;
    s = (s + n / s) >> 1;
    if s * s > n {
      s - 1
    } else {
      s
    }
  }

  fn perfect_sqrt(n: usize) -> isize {
    match n & 0xf {
      0 | 1 | 4 | 9 => {
        let t = isqrt(n);
        if t * t == n {
          t as isize
        } else {
          -1
        }
      }
      _ => -1,
    }
  }
  #[test]
  fn test_is_square() {
    let mut squares = vec![];
    for i in 0..200 {
      if perfect_sqrt(i) >= 0 {
        squares.push(true);
      } else {
        squares.push(false);
      }
    }
    for i in 0..squares.len() {
      println!("{}", i);
      let val = bu!(i);
      assert!(is_prob_square(&val) == squares[i]);
    }
  }
  #[test]
  fn test_small_prime_factor() {
    let n_prime = bu!(233u64);
    let n_composite = bu!(50_621u64);
    let n_composite_large = bu!(104_927u64);

    assert!(n_composite == bu!(223u64) * bu!(227u64));
    assert!(n_composite_large == bu!(317u64) * bu!(331u64));

    assert!(!has_small_prime_factor(&n_prime));
    assert!(has_small_prime_factor(&n_composite));
    assert!(!has_small_prime_factor(&n_composite_large));
  }

  #[test]
  fn test_miller_rabin() {
    assert!(passes_miller_rabin_base_2(&bu!(13u64)));
    assert!(!passes_miller_rabin_base_2(&bu!(65u64)));
  }
}
