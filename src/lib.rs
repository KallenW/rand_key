//! # Usage:
//! ```rust
//! use rand_pwd::{ RandPwd, ToRandPwd };
//! fn main() {
//!     let mut r_p = RandPwd::new(10, 2, 3); // For now, it's empty. Use method `join` to generate the password
//!     r_p.join();                           // Now `r_p` has some content, be kept in its `content` field
//!     println!("{}", r_p);                  // Print it on the screen
//!     // One possible output: 7$pA7yMCw=2DPGN
//!     // Or you can build from an existing `&str`
//!     let mut r_p = RandPwd::from("=tE)n5f`sidR>BV"); // 10 letters, 4 symbols, 1 number
//!     // You can rebuild a random password and with equivalent amount of letters, symbols and numbers. Like below
//!     r_p.join();
//!     println!("{}", r_p);
//!     // One possible output: qS`Xlyhpmg~"V8[
//!     // All the `String` and `&str` has implemented trait `ToRandPwd`
//!     // which means you can use method `to_randpwd` to convert a `String` or `&str` to `RandPwd`
//!     let mut r_p = "n4jpstv$dI,.z'K".to_randpwd().unwrap();
//!     // Panic! Has non-ASCII character(s)!
//!     // let mut r_p = RandPwd::from("🦀️🦀️🦀️");
//!     // let mut r_p = "🦀️🦀️🦀️".to_randpwd();
//! }
//! ```
//! # The `UNIT` field
//! The UNIT field is used to help process large number in concurrent way.
//!
//! If you want to generate a huge random password with 1 million letters, symbols and numbers each,
//! our program will accept such a sequence: [1M, 1M, 1M].
//! However, it takes up huge RAM(Because these numbers are represented in `BigUint`, kind of a `Vec`).
//! And the procedure is single-threaded, you can only process them one by one.
//!
//! My approach is to divide these large numbers into many small numbers,
//! and then process these small numbers in parallel,
//! so the small numbers here can be understood as `UNIT`.
//! For 1M letters, we set 1K as the unit value, so [1M] = [1K, 1K, …, 1K] (1000 ones).
//! And we just need to hand this sequence to [rayon](https://github.com/rayon-rs/rayon) for processing.
//! But the disadvantages are also obvious, if `UNIT` number is too small, like default value: 1,
//! then capcity of the `Vec` is 1M at least!
//! It will take up huge even all RAM and may harm your computer.
//! In the next version, there will be a smart `UNIT` value to fix this problem.


#![allow(non_snake_case)]


mod prelude;
use prelude::*;


/// struct `RandPwd`
#[derive(Clone, Debug)]
pub struct RandPwd {
    ltr_cnt: BigUint,
    sbl_cnt: BigUint,
    num_cnt: BigUint,
    content: String,
    UNIT: BigUint,    // TODO: - implement a smart UNIT initialization to get the best performance
    DATA: Vec<Vec<String>>,
}


/// A generic trait for converting a value to a `RandPwd`.
pub trait ToRandPwd {

    /// Converts the value of `self` to a `RandPwd`.
    fn to_randpwd(&self) -> Option<RandPwd>;

}


impl RandPwd {


    /// Return an empty instance of `Result<RandPwd, &'static str>`
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// use rand_pwd::RandPwd;
    /// use num_bigint::BigUint;
    /// let mut r_p = RandPwd::new(11, 4, 2);
    ///
    /// // If you want push a large number in it
    /// // parse the `&str` into `BigUint`
    /// use std::str::FromStr;
    ///
    /// let ltr_cnt = BigUint::from_str(&format!("{}000", usize::MAX)).unwrap();
    /// let sbl_cnt = BigUint::from_str(&format!("{}000", usize::MAX)).unwrap();
    /// let num_cnt = BigUint::from_str(&format!("{}000", usize::MAX)).unwrap();
    ///
    /// r_p = RandPwd::new(ltr_cnt, sbl_cnt, num_cnt);
    ///
    /// // You can also mix the `BigUint` with primitive type
    /// ```
    #[inline]
    pub fn new<L, S, N>(ltr_cnt: L, sbl_cnt: S, num_cnt: N) -> Self
    where L: ToBigUint,
          S: ToBigUint,
          N: ToBigUint,
    {

        RandPwd {
            ltr_cnt: ltr_cnt.to_biguint().unwrap(),
            sbl_cnt: sbl_cnt.to_biguint().unwrap(),
            num_cnt: num_cnt.to_biguint().unwrap(),
            content: String::new(),
            UNIT: BigUint::one(),
            DATA: _DATA(),
        }

    }


    /// Return the content of random password in `&str`
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// use rand_pwd::RandPwd;
    /// let r_p = RandPwd::new(10, 2, 3);
    /// assert_eq!("", r_p.val())
    /// ```
    #[inline]
    pub fn val(&self) -> &str {
        &self.content
    }


    /// Change the content of `RandPwd`, depend on the name of operation you passed.
    /// There's two operations: **update** and **check**
    ///
    /// update means just replace the value you've passed and update the counts field
    ///
    /// check means if the counts field of new value doesn't match the old one, it will panic!
    /// if checking passed, the old one will be replaced
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// // update
    /// use rand_pwd::RandPwd;
    /// use num_traits::ToPrimitive;
    /// use num_bigint::BigUint;
    /// let mut r_p = RandPwd::new(10, 2, 3);
    /// r_p.set_val("123456", "update");
    /// assert_eq!(*r_p.get_cnt("ltr").unwrap(), BigUint::from(0_usize));
    /// assert_eq!(*r_p.get_cnt("sbl").unwrap(), BigUint::from(0_usize));
    /// assert_eq!(*r_p.get_cnt("num").unwrap(), BigUint::from(6_usize));
    ///
    /// // check
    /// let mut r_p = RandPwd::new(10, 2, 3);
    /// // r_p.set_val("123456", "check"); // Will panic
    /// ```
    #[inline]
    pub fn set_val(&mut self, val: &str, op: &str) {

        match op {

            "update" => {
                self.ltr_cnt = _CNT(val).0.to_biguint().unwrap();
                self.sbl_cnt = _CNT(val).1.to_biguint().unwrap();
                self.num_cnt = _CNT(val).2.to_biguint().unwrap();

                self.content = val.into();
            },

            "check" => {
                if (self.ltr_cnt.to_usize().unwrap(),
                    self.sbl_cnt.to_usize().unwrap(),
                    self.num_cnt.to_usize().unwrap(),) == _CNT(val) {

                    self.content = val.into();
                } else {
                    panic!("The fields of {:?} is not right", val);
                }
            },

            _ => (),
        }
    }


    /// Return the value of `UNIT`
    /// # Example
    ///
    /// Basic Usage:
    /// ```
    /// use rand_pwd::RandPwd;
    /// let r_p = RandPwd::new(10, 2, 3); // The default value of unit is 1
    /// assert_eq!(r_p.unit(), 1);
    /// ```
    #[inline]
    pub fn unit(&self) -> &BigUint {
        &self.UNIT
    }


    /// The value of UNIT is inversely proportional to memory overhead
    /// In order to reduce the memory overhead, raise the value of `UNIT`
    #[inline]
    pub fn set_unit(&mut self, val: impl ToBigUint) {
        self.UNIT = val.to_biguint().unwrap();
    }


    /// Return the shared reference of `DATA`
    #[inline]
    pub fn data(&self) -> &Vec<Vec<String>> {
        &self.DATA
    }

    /// Set the user-defined data
    // #[inline]
    // pub fn set_data<T: AsRef<str>>(&mut self, val: &[T]) -> result::Result<(), Box<dyn Error>> {
    //
    //     self.DATA = val.iter().map(|x| x.as_ref().to_string()).collect::<Vec<_>>();
    //
    //     Ok(())
    //
    // }


    /// Returns the length of this `RandPwd`, in both bytes and [char]s.
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// use rand_pwd::RandPwd;
    /// let mut r_p = RandPwd::new(10, 2, 3);
    /// r_p.join();
    /// assert_eq!(r_p.len(), 15);
    /// ```
    ///
    #[inline]
    pub fn len(&self) -> usize {
        self.content.len()
    }


    /// Returns true if this `RandPwd` has a length of zero, and false otherwise.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }


    /// Get count of `RandPwd`
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// use rand_pwd::RandPwd;
    /// use num_traits::ToPrimitive;
    /// let r_p = RandPwd::new(10, 2, 3);
    /// assert_eq!(r_p.get_cnt("ltr").unwrap().to_usize().unwrap(), 10);
    /// assert_eq!(r_p.get_cnt("sbl").unwrap().to_usize().unwrap(), 2);
    /// assert_eq!(r_p.get_cnt("num").unwrap().to_usize().unwrap(), 3);
    /// ```
    #[inline]
    pub fn get_cnt(&self, kind: &str) -> Option<&BigUint> {

        match kind {

            "ltr" => Some(&self.ltr_cnt),
            "sbl" => Some(&self.sbl_cnt),
            "num" => Some(&self.num_cnt),

            _   => None,
        }
    }


    /// Change the count of letters, symbols or numbers of `RandPwd`
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// use rand_pwd::*;
    /// let mut r_p = RandPwd::new(10, 2, 3);
    ///
    /// // Set the letter's count
    /// r_p.set_cnt("ltr", 0);
    /// r_p.join();
    /// println!("{}", r_p.val());
    /// // Output: *029(
    ///
    /// // Set the symbol's count
    /// r_p.set_cnt("sbl", 0);
    /// r_p.join();
    /// println!("{}", r_p.val());
    /// // Output: nz1MriAl0j5on
    ///
    /// // Set the number's count
    /// r_p.set_cnt("num", 0);
    /// r_p.join();
    /// println!("{}", r_p.val());
    /// // Output: +iQiQGSXl(nv
    /// ```
    #[inline]
    pub fn set_cnt<T: ToBigUint>(&mut self, kind: &str, val: T) -> Option<()> {

        match kind {

            "ltr" => self.ltr_cnt = val.to_biguint()?,
            "sbl" => self.sbl_cnt = val.to_biguint()?,
            "num" => self.num_cnt = val.to_biguint()?,

            _     => (),
        }

        Some(())
    }


    /// Generate the password for `RandPwd`
    /// # Example
    ///
    /// Basic usage:
    /// ```
    /// use rand_pwd::RandPwd;
    /// let mut r_p = RandPwd::new(10, 2, 3);
    /// r_p.join();
    /// println!("{}", r_p);
    /// ```
    #[inline]
    pub fn join(&mut self) {

        let mut inner_r_p = self.clone();
        let unit = &inner_r_p.UNIT;
        let data = &inner_r_p.DATA;

        // TODO: - Improve readability
        let mut PWD
                 =
        vec![(&mut inner_r_p.ltr_cnt, &data[0]),
             (&mut inner_r_p.sbl_cnt, &data[1]),
             (&mut inner_r_p.num_cnt, &data[2]),]
            .into_iter()
            .map(|(bignum, data)| {
                _DIV_UNIT(unit, bignum)
                    .par_iter()
                    .map(|cnt| {
                        _RAND_IDX(cnt, data.len())
                            .iter()
                            // TODO: - Remove this `clone` which can cause huge overhead of both memory and CPU
                            .map(|idx| data[*idx].clone())
                            .collect::<String>()
                    })
                    .collect()
            })
            .collect::<Vec<Vec<_>>>()
            .concat()
            .join("");

        // This is absolutely safe, because they are all ASCII characters except control ones.
        let bytes = unsafe { PWD.as_bytes_mut() };
        bytes.shuffle(&mut thread_rng());
        self.content = bytes.par_iter().map(|s| *s as char).collect::<String>();

    }

}
