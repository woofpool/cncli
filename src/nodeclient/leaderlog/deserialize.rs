use rug::float::Round;
use rug::ops::MulAssignRound;
use rug::{Float, Rational};
use serde::{Deserialize, Deserializer};
use serde::de::{Visitor, SeqAccess};
use serde_cbor::{de, Value};
use serde_json::Number;
use std::marker::PhantomData;
use std::fmt;
use crate::nodeclient::leaderlog::ledgerstate::Stake;

pub(crate) fn rational<'de, D: Deserializer<'de>>(d: D) -> Result<Rational, D::Error> {
    let n: Number = Deserialize::deserialize(d)?;
    let mut f: Float = Float::with_val(24, Float::parse(&*n.to_string()).unwrap());
    f.mul_assign_round(100, Round::Nearest);
    Ok(Rational::from((f.to_integer().unwrap(), 100)))
}

pub(crate) fn rational_optional<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Rational>, D::Error> {
    let n: Option<Number> = Deserialize::deserialize(d)?;
    match n {
        None => Ok(None),
        Some(number) => {
            let mut f: Float = Float::with_val(24, Float::parse(&*number.to_string()).unwrap());
            f.mul_assign_round(100, Round::Nearest);
            Ok(Some(Rational::from((f.to_integer().unwrap(), 100))))
        }
    }
}

pub(crate) fn cbor_hex<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
    let cbor: String = Deserialize::deserialize(d)?;
    let cbor_vec = hex::decode(cbor).unwrap();
    let value: Value = de::from_slice(&*cbor_vec).unwrap();
    match value {
        Value::Bytes(key) => Ok(key),
        _ => {
            panic!("Invalid cbor hex!")
        }
    }
}

pub(crate) fn total_active_stake<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
{
    struct SumVisitor(PhantomData<fn() -> u64>);

    impl<'de> Visitor<'de> for SumVisitor {
        /// Return type of this visitor. This visitor computes the sum of a
        /// sequence of values of type T, so the type of the sum is T.
        type Value = u64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a nonempty sequence of numbers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<u64, S::Error>
            where
                S: SeqAccess<'de>,
        {
            // Start with max equal to the first value in the seq.
            let mut sum = 0_u64;

            // Update the max while there are additional values.
            while let Some(value) = seq.next_element::<Vec<Stake>>()? {
                for stake in value.into_iter() {
                    match stake {
                        Stake::StakeKey(_stake_key) => {},
                        Stake::Lovelace(amount) => { sum += amount}
                    }
                }
            }

            Ok(sum)
        }
    }

    // Create the visitor and ask the deserializer to drive it. The
    // deserializer will call visitor.visit_seq() if a seq is present in
    // the input data.
    let visitor = SumVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}