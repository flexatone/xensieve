
mod util {

    fn gcd(mut n: u64, mut m: u64) -> u64 {
        // not sure if assert is best way to handle this
        assert!(n != 0 && m != 0);
        while m != 0 {
            if m < n {
                let t = m;
                m = n;
                n = t;
            }
            m = m % n;
        }
        n
    }

    fn lcm(a: u64, b: u64) -> u64 {
        a * b / gcd(a, b)
    }

    fn meziriac(a: u64, b: u64) -> u64 {
        let mut g: u64 = 1;
        if b == 1 {
            g = 1;
        } else if a == b {
            g = 0;
        } else {
            while g < u64::MAX {
                if ((g * a) % b) == 1 {
                    break
                }
                g += 1;
            }
        }
        g
    }

    fn mod_inv(a: i64, b: i64) -> Option<i64> {
        let (mut a, mut b) = (a, b);
        let (mut current_inv, mut next_inv) = (0, 1);
        let modulus = b;

        if b == 1 {
            return Some(1);
        }
        while a > 1 {
            let temp_inv = current_inv - a / b * next_inv;
            let remainder = a % b;
            (a, b) = (b, remainder);
            current_inv = next_inv;
            next_inv = temp_inv;
        }
        if current_inv < 0 {
            current_inv += modulus;
        }
        if a == 1 {
            Some(current_inv)
        } else {
            // No modular inverse exists; a and b not coprime
            None
        }
    }

    #[cfg(test)] // only compile when running cargo test
    mod tests {
        use super::*; // bring code in outer into scope
        // use crate::util::*;

        #[test]
        fn test_gcd_a() {
            assert_eq!(gcd(14, 15), 1);
        }

        #[test]
        fn test_gcd_b() {
            assert_eq!(gcd(12, 8), 4);
        }

        #[test]
        fn test_gcd_c() {
            let a = 2 * 3 * 5 * 11 * 17;
            let b = 3 * 7 * 11 * 13 * 19;
            assert_eq!(gcd(a, b), 3 * 11);
        }

        #[test]
        #[should_panic]
        fn test_gcd_d() {
            gcd(12, 0);
        }

        #[test]
        #[should_panic]
        fn test_gcd_e() {
            gcd(0, 3);
        }

        #[test]
        fn test_lcm_a() {
            assert_eq!(lcm(12, 8), 24);
        }

        #[test]
        fn test_lcm_b() {
            assert_eq!(lcm(3, 4), 12);
        }

        #[test]
        #[should_panic]
        fn test_lcm_c() {
            // as gcd panics on 0, this does as well
            assert_eq!(lcm(3, 0), 0);
        }

        #[test]
        fn test_meziriac_a() {
            assert_eq!(meziriac(1, 1), 1);
            assert_eq!(meziriac(10, 1), 1);
            assert_eq!(meziriac(10, 10), 0);
            assert_eq!(meziriac(12, 12), 0);
            assert_eq!(meziriac(3, 11), 4);
            assert_eq!(meziriac(20, 9), 5);
        }

        #[test]
        fn test_mod_inv_a() {
            assert_eq!(mod_inv(3, 11), Some(10));
            assert_eq!(mod_inv(20, 9), Some(9));
        }
    }

}


// NOTE: not clear how to test private library methods

