use bigdecimal::BigDecimal;

const ZERO: BigDecimal = normalize(BigDecimal::from(0));
const ONE: BigDecimal = normalize(BigDecimal::from(1));
const EXP1: BigDecimal = exp(ONE);

fn ln(x: BigDecimal) -> BigDecimal {
    if x <= ZERO {
        panic!("X must be positive, non zero");
    }

    let (integral, fractional) = split_ln(x);

    return integral + lncf(1000, fractional);
}


fn split_ln(x: BigDecimal) -> (BigDecimal, BigDecimal) {
    let integral = find_e(EXP1, x.clone());
    let fractional = (x/(ipow(EXP1, integral))) - 1;

    return (integral, fractional);
}

fn exp(x:BigDecimal) -> BigDecimal {
    return if x < ZERO {
        normalize(ONE / exp(-x))
    } else {
        let (n, x_) = scale_exp(x);
        let x_prime = taylor_exp(1000, 1, x_, 1, 1, 1);
        ipow(x_prime, n)
    }
}

fn find_e(e: BigDecimal, x: BigDecimal) -> (BigDecimal, BigDecimal) {
    var (lower, upper) = bound e x (1/e) e (-1) 1
    return contract()
}

fn normalize(x:BigDecimal) -> BigDecimal {
    x.with_prec(10).with_scale(35).round(34)
}