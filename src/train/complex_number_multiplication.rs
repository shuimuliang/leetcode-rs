/*
https://leetcode.com/problems/complex-number-multiplication
https://github.com/rust-num/num-complex
*/

use core::ops::Mul;

struct Complex {
    /// Real portion of the complex number
    re: i32,
    /// Imaginary portion of the complex number
    im: i32,
}

impl Complex {
    fn new(re: i32, im: i32) -> Self {
        Complex { re, im }
    }
}

impl Mul<Complex> for Complex {
    type Output = Self;

    #[inline]
    fn mul(self, other: Self) -> Self::Output {
        let re = self.re * other.re - self.im * other.im;
        let im = self.re * other.im + self.im * other.re;
        Self::Output::new(re, im)
    }
}

fn deserialize(s: &str) -> Result<Complex, ()> {
    let mut a = s.split('+');
    let re_str = a.next().unwrap();
    let im_str = a.next().unwrap();
    let im_str_raw = &im_str[..im_str.len() - 1];
    Ok(Complex {
        re: re_str.parse::<i32>().unwrap(),
        im: im_str_raw.parse::<i32>().unwrap(),
    })
}

fn serialize(t: &Complex) -> String {
    format!("{}+{}i", t.re, t.im)
}

struct Solution;

impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        let num1_c: Complex = deserialize(&num1).unwrap();
        let num2_c: Complex = deserialize(&num2).unwrap();
        let num3_c = num1_c * num2_c;
        serialize(&num3_c)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let num1 = "1+1i".to_string();
        let num2 = "1+1i".to_string();
        let res = Solution::complex_number_multiply(num1, num2);
        assert_eq!(res, "0+2i");
    }

    #[test]
    fn test_case_2() {
        let num1 = "1+-1i".to_string();
        let num2 = "1+-1i".to_string();
        let res = Solution::complex_number_multiply(num1, num2);
        assert_eq!(res, "0+-2i");
    }
}
