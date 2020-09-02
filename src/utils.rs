
pub use rand::prelude::*;
pub use rayon::prelude::*;
pub use num_bigint::{ BigUint, ToBigUint };
pub use num_traits::{ Zero, One, ToPrimitive };


/// Characters set
///
/// return letters, symbols, numbers in `CharVec`
#[inline]
pub(crate) fn _DATA() -> Vec<Vec<String>> {

    let mut letters = Vec::<String>::new();
    let mut symbols = Vec::<String>::new();
    let mut numbers = Vec::<String>::new();

    let mut charset = vec![];

    let _ = (33..127)
        .into_iter()
        .map(|x| {
            let ch = x as u8 as char;
            if ch.is_ascii_alphabetic()  { letters.push(ch.into()); }
            if ch.is_ascii_punctuation() { symbols.push(ch.into()); }
            if ch.is_ascii_digit()       { numbers.push(ch.into()); }
        })
        .collect::<()>();

    charset.push(letters);
    charset.push(symbols);
    charset.push(numbers);

    charset

}


/// Count the number of a string
#[inline]
pub(crate) fn _CNT<T: AsRef<str>>(content: T) -> (BigUint, BigUint, BigUint) {

    use std::sync::Mutex;

    let l = Mutex::new(0);
    let s = Mutex::new(0);
    let n = Mutex::new(0);

    content.as_ref().chars().collect::<Vec<_>>().par_iter().for_each(
        |x| {
            if x.is_ascii() {
                if x.is_ascii_alphabetic()  {
                    let mut temp = l.lock().unwrap();
                    *temp += 1;
                }
                if x.is_ascii_punctuation() {
                    let mut temp = s.lock().unwrap();
                    *temp += 1;
                }
                if x.is_ascii_digit()       {
                    let mut temp = n.lock().unwrap();
                    *temp += 1;
                }
            } else {
                panic!("Has non-ASCII character(s)!, the first one is: {:?}", x)
            }
        }
    );

    (l.into_inner().unwrap().to_biguint().unwrap(),
     s.into_inner().unwrap().to_biguint().unwrap(),
     n.into_inner().unwrap().to_biguint().unwrap(),)

}


/// Generate n random numbers, each one is up to `length`
#[inline]
pub(crate) fn _RAND_IDX(cnt: &BigUint, length: usize) -> Vec<usize> {

    let mut n = cnt.to_biguint().unwrap();
    let mut idxs = Vec::with_capacity(n.to_usize().unwrap());

    while !n.is_zero() {
        idxs.push(thread_rng().gen_range(0, length));
        n -= BigUint::one();
    }

    idxs

}


/// Resolve large numbers into smaller numbers
#[inline]
pub(crate) fn _DIV_UNIT(unit: &BigUint, n: &mut BigUint) -> Vec<BigUint> {

    let UNIT = unit.to_biguint().unwrap();
    let mut ret = Vec::with_capacity((n.clone() / &UNIT + BigUint::one()).to_usize().unwrap());

    loop {
        if n.clone() < UNIT {
            ret.push(n.to_biguint().unwrap());
            break;
        } else {
            *n -= UNIT.clone();
            ret.push(UNIT.clone());
        }
    }

    ret

}
