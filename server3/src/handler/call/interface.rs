use std::pin::Pin;
use std::str;
use std::io;
use futures::future::join_all;
use crate::metafs::write;
use crate::util::global::{GLOBAL};
use crate::parser;

//fn print_type_of<T>(_: &T) {
//    println!("{}", std::any::type_name::<T>())
//}

use std::cmp;
use std::fmt;
use ::std::{*,
    convert::{
        TryFrom,
    },
    ops::{
        Not,
        Sub,
    },
};
use ::num_bigint::{ // 0.2.2
    BigInt,
    BigUint,
};
use fraction::Fraction;

fn pi (precision: u64) -> String
{
    /// atan(x) = x - x^3/3 + x^5/5 - x^7/7 + x^9/9...
    fn atan (x: Fraction, precision: u64) -> Fraction
    {
        use ::num_traits::pow::pow;
        let end: BigUint =
            BigUint::from(10_u32)
                .pow(precision as u32)
        ;
        let target = Fraction::new(1.into(), end);

        let mut current_term = x.clone();
        let mut ret = Fraction::from(0);
        let mut sign = BigInt::from(1);
        let mut n = BigUint::from(1_u32);
        let mut x_pow_n = x.clone();
        let two = BigUint::from(2_u32);
        let x_square = &x * &x;

        while current_term.abs() > target {
            ret = ret + current_term;
            // eprintln!(
            //     "atan({}) ~ {}",
            //     x,
            //     ret.decimal(precision as usize),
            // );
            n += &two;
            sign = -sign;
            x_pow_n = x_pow_n * &x_square;
            current_term = &x_pow_n * Fraction::new(
                sign.clone(),
                n.clone(),
            );
        }
        ret
    }

    let precision_usize = usize::
        try_from(precision)
            .expect("Overflow")
    ;
    let pi_approx = Fraction::sub(
        Fraction::from(16) * atan(
            Fraction::new(1.into(), 5_u32.into()),
            precision
                .checked_add(2) // 16 -> 10 ^ 2
                .expect("Overflow"),
        ),
        Fraction::from(4) * atan(
            Fraction::new(1.into(), 239_u32.into()),
            precision + 1, // 4 -> 10 ^ 1
        ),
    );
    pi_approx.decimal(precision_usize)
}

mod fraction {
    #![allow(clippy::suspicious_arithmetic_impl)]
    use super::*;
    use ::num_bigint::*;
    use ::num_traits::*; // 0.2.8
    use ::core::ops::{
        Add,
        Div,
        Mul,
        Neg,
        Sub,
    };

    #[derive(
        Debug,
        Clone,
        PartialEq, Eq,
        Ord,
    )]
    pub
    struct Fraction {
        pub
        numerator: BigInt,

        pub
        denominator: BigUint,
    }

    impl From<i32> for Fraction {
        #[inline]
        fn from (x: i32) -> Self
        {
            Self::new(x.into(), 1_u32.into())
        }
    }

    impl PartialOrd for Fraction {
        fn partial_cmp (
            self: &'_ Self,
            other: &'_ Self,
        ) -> Option<cmp::Ordering>
        {
            (self - other)
                .numerator
                .partial_cmp(&BigInt::zero())
        }
    }

    impl Fraction {
        pub
        fn new (
            numerator: BigInt,
            denominator: BigUint,
        ) -> Self
        {
            assert!(denominator.is_zero().not(), "Division by zero");
            let mut ret = Self {
                numerator,
                denominator,
            };
            ret.simplify();
            ret
        }

        pub
        fn simplify (self: &'_ mut Self)
        {
            let (sign, abs) = self.numerator.split();
            let gcd = gcd(
                &abs,
                &self.denominator,
            );
            self.numerator = BigInt::from_biguint(
                sign,
                abs / &gcd,
            );
            self.denominator /= gcd;
        }

        pub
        fn inverse (self: &'_ Self) -> Self
        {
            if let Some(numerator) = self.numerator.to_biguint() {
                Fraction::new(
                    self.denominator.to_bigint()
                        .unwrap() // why ???
                    ,
                    numerator,
                )
            } else {
                Fraction::new(
                    BigInt::from_biguint(
                        self.numerator.sign(),
                        self.denominator.clone(),
                    ),
                    self.numerator
                        .clone()
                        .neg()
                        .to_biguint()
                        .unwrap(),
                )
            }
        }

        pub
        fn abs (self: &'_ Self) -> Self
        {
            Self {
                numerator: self.numerator.abs(),
                denominator: self.denominator.clone(),
            }
        }

        pub
        fn decimal (
            self: &'_ Self,
            precision: usize,
        ) -> String
        {
            use ::core::fmt::Write;
            use ::num_integer::Integer;
            let mut ret = String::new();
            let Self {
                numerator,
                denominator,
            } = self.clone();
            let (sign, mut numerator) = numerator.split();
            if let Sign::Minus = sign {
                ret.push('-');
            }
            let base = BigUint::from(10_u32);
            let (q, r) = numerator.div_mod_floor(&denominator);
            write!(&mut ret, "{}", q).unwrap();
            if r.is_zero() {
                return ret;
            } else {
                ret.reserve(1 + precision);
                ret.push('.');
            }
            numerator = r * &base;
            for _ in 0 .. precision {
                let (q, r) = numerator.div_mod_floor(&denominator);
                write!(&mut ret, "{}", q).unwrap();
                if r.is_zero() { break; }
                numerator = r * &base;
            }
            ret
        }
    }

    macro_rules! derive_op {(
        impl $Op:ident for Fraction {
            type Output = Fraction;

            fn $op:ident (&$self:tt, &$other:tt) -> Self::Output
            $body:block
        }
    ) => (
        impl<'a> $Op for &'a Fraction {
            type Output = Fraction;

            fn $op ($self: &'a Fraction, $other: &'a Fraction) -> Self::Output
            $body
        }

        impl<'a> $Op<&'a Fraction> for Fraction {
            type Output = Fraction;

            #[inline]
            fn $op ($self: Fraction, $other: &'a Fraction) -> Self::Output
            {
                $Op::$op(&$self, $other)
            }
        }

        impl<'a> $Op<Fraction> for &'a Fraction {
            type Output = Fraction;

            #[inline]
            fn $op ($self: &'a Fraction, $other: Fraction) -> Self::Output
            {
                $Op::$op($self, &$other)
            }
        }

        impl $Op for Fraction {
            type Output = Fraction;

            #[inline]
            fn $op ($self: Fraction, $other: Fraction) -> Self::Output
            {
                $Op::$op(&$self, &$other)
            }
        }
    )}

    derive_op! {
        impl Add for Fraction {
            type Output = Fraction;

            fn add (&self, &other) -> Self::Output
            {
                let lhs = {
                    let (sign, abs) = self.numerator.split();
                    BigInt::from_biguint(
                        sign,
                        abs * &other.denominator,
                    )
                };
                let rhs = {
                    let (sign, abs) = other.numerator.split();
                    BigInt::from_biguint(
                        sign,
                        abs * &self.denominator,
                    )
                };
                Fraction::new(
                    lhs + rhs,
                    &self.denominator * &other.denominator,
                )
            }
        }
    }

    derive_op! {
        impl Sub for Fraction {
            type Output = Fraction;

            fn sub (&self, &other) -> Self::Output
            {
                let lhs = {
                    let (sign, abs) = self.numerator.split();
                    BigInt::from_biguint(
                        sign,
                        abs * &other.denominator,
                    )
                };
                let rhs = {
                    let (sign, abs) = other.numerator.split();
                    BigInt::from_biguint(
                        sign,
                        abs * &self.denominator,
                    )
                };
                Fraction::new(
                    lhs - rhs,
                    &self.denominator * &other.denominator,
                )
            }
        }
    }

    derive_op! {
        impl Mul for Fraction {
            type Output = Fraction;

            fn mul (&self, &other) -> Self::Output
            {
                Fraction::new(
                    &self.numerator * &other.numerator,
                    &self.denominator * &other.denominator,
                )
            }
        }
    }

    derive_op! {
        impl Div for Fraction {
            type Output = Fraction;

            fn div (&self, &other) -> Self::Output
            {
                self * other.inverse()
            }
        }
    }

    impl fmt::Display for Fraction {
        fn fmt (
            self: &'_ Self,
            stream: &'_ mut fmt::Formatter<'_>,
        ) -> fmt::Result
        {
            write!(stream,
                "{} / {}",
                self.numerator,
                self.denominator,
            )
        }
    }

    fn gcd (a: &'_ BigUint, b: &'_ BigUint) -> BigUint
    {
        let mut a = a.clone();
        let mut b = b.clone();
        while b.is_zero().not() {
            let r = a % &b;
            a = b;
            b = r;
        }
        a
    }

    trait SignSplit {
        fn split (self: &'_ Self) -> (Sign, BigUint);
    }
    impl SignSplit for BigInt {
        fn split (self: &'_ BigInt) -> (Sign, BigUint)
        {
            fn to_biguint_lossy (this: &'_ BigInt) -> BigUint
            {
                this.to_biguint()
                    .unwrap_or_else(||
                        this.neg()
                            .to_biguint()
                            .unwrap()
                    )
            }
            (self.sign(), to_biguint_lossy(self))
        }
    }
}

pub async fn run2(data: &[u8]) -> &[u8] {
    let x = pi(1000);
    println!("x is {}", x);

    return "ok".as_bytes();
}

pub async fn run(data: &[u8]) -> &[u8] {
    let x = pi(1000);
    println!("x is {}", x);


    let newline : &[u8] = &[0x0a];
    let res:Vec<u8> = [data, newline].concat();

    let mut rec_data = parser::record::Record::new(data.to_vec());
    rec_data.syslog_parse();

    let mut file_write: Vec<u8> = "test".as_bytes().to_vec();
    let file_write_field = rec_data.field.get("appname");
    if let Some(v) = file_write_field {
        file_write = v.clone();
    }
    let file_write_str = str::from_utf8(&file_write).unwrap_or_else(|_| "test");

    if let Ok(mut slb) = GLOBAL.lock() {
        let buf = slb.buffer.get_mut(file_write_str);
        //println!("res len is {}", res.len());
        if buf == Option::None {
            slb.insert(file_write_str.to_string(), res.clone());
            return "ok".as_bytes();
        }

        let bufdata = buf.unwrap();
        //print_type_of(&bufdata);

        println!("bufdata len is {}, capacity is {}, buffer/file is {}", bufdata.len(), bufdata.capacity(), file_write_str);
        if bufdata.len() >= 10000 {
            println!("bufdata len is: {}, write to file: {}!", bufdata.len(), file_write_str);
            let write_op = write::write(file_write_str, &bufdata);

            let mut fut: Vec<Pin<Box<dyn warp::Future<Output = ()>>>> = Vec::new();
            fut.push(Box::pin(write_op));

            join_all(fut).await;
            slb.buffer.clear();
        } else {
            bufdata.extend(res.clone());
        }
    }

    return "ok".as_bytes();
}

#[derive(Debug)]
struct IOerr;
impl warp::reject::Reject for IOerr {}

pub async fn run_elastic_doc(data: serde_json::Value, index: &str, doc_type: &str) ->  Result<&'static [u8], warp::Rejection> {
    let file_write: Vec<u8> = "test".as_bytes().to_vec();
    let file_write_str = str::from_utf8(&file_write).unwrap_or_else(|_| "test");

    println!("Index: {}, Type: {}, Data: {:?}", index, doc_type, data);
            let write_op = write::write(file_write_str, "test".as_bytes());

    //        let mut fut: Vec<Pin<Box<dyn warp::Future<Output = ()>>>> = Vec::new();
    //        fut.push(Box::pin(write_op));

    //        join_all(fut).await;
    //        //slb.buffer.clear();
    ////return "ok".as_bytes();

    //if 1 == 2 {
    //    return Err(warp::reject::custom(IOerr));
    //}

    Ok("ok".as_bytes())
}
