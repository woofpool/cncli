use bigdecimal::{BigDecimal, One};

// const EXP1: BigDecimal = exp(ONE);
//const EPS: BigDecimal = BigDecimal::from_str("1000000000000000000000000").unwrap().inverse().with_prec(10).with_scale(34); // 10^24

// ipow' :: Num a => a -> Integer -> a
// ipow' x n
//   | n == 0 = 1
//   | m == 0 = let y = ipow' x d in y * y
//   | otherwise = x * ipow' x (n - 1)
//   where (d,m) = divMod n 2
fn ipow_p(x: &BigDecimal, n: i32) -> BigDecimal {
    if n == 0 {
        return BigDecimal::one();
    }
    let d = n / 2_i32;
    let m = n % 2_i32;
    if m == 0 {
        return normalize(ipow_p(x, d).square());
    }

    return normalize(x * ipow_p(x, n - 1));
}

// ipow :: Fractional a => a -> Integer -> a
// ipow x n
//   | n < 0 = 1 / ipow' x (-n)
//   | otherwise = ipow' x n
pub fn ipow(x: &BigDecimal, n: i32) -> BigDecimal {
    return if n < 0 {
        normalize(ipow_p(x, -n).inverse())
    } else {
        ipow_p(x, n)
    };
}

// fn ln(x: BigDecimal) -> BigDecimal {
//     if x <= ZERO {
//         panic!("X must be positive, non zero");
//     }
//
//     let (integral, fractional) = split_ln(x);
//
//     return integral + lncf(1000, fractional);
// }

// taylorExp :: (RealFrac a, Show a) => Int -> Int -> a -> a -> a -> a -> a
// taylorExp maxN n x lastX acc divisor
// | maxN == n = acc
// | abs nextX < eps = acc
// | otherwise = taylorExp maxN (n + 1) x nextX (acc + nextX) (divisor + 1)
// where nextX = (lastX * x) / divisor
// fn taylor_exp(max_n: i32, n: i32, x: BigDecimal, last_x: BigDecimal, acc: BigDecimal, divisor: BigDecimal) -> BigDecimal {
//     if max_n == n {
//         return acc;
//     }
//     let next_x = normalize((last_x * &x) / divisor);
//     if &next_x.abs() < eps {
//         return acc;
//     }
//
//     taylor_exp(max_n, n+1, x, next_x, acc+next_x, divisor + 1)
// }

// fn split_ln(x: BigDecimal) -> (BigDecimal, BigDecimal) {
//     let integral = find_e(EXP1, x.clone());
//     let fractional = (x/(ipow(EXP1, integral))) - 1;
//
//     return (integral, fractional);
// }

// fn exp(x:BigDecimal) -> BigDecimal {
//     return if x < ZERO {
//         normalize(ONE / exp(-x))
//     } else {
//         let (n, x_) = scale_exp(x);
//         let x_prime = taylor_exp(1000, 1, x_, ONE, ONE, ONE);
//         ipow(x_prime, n)
//     }
// }

// fn find_e(e: BigDecimal, x: BigDecimal) -> (BigDecimal, BigDecimal) {
//     var (lower, upper) = bound e x (1/e) e (-1) 1
//     return contract()
// }

fn normalize(x: BigDecimal) -> BigDecimal {
    x.with_scale(34)
}