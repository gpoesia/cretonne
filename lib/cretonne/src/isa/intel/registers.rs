//! Intel register descriptions.

use isa::registers::{RegBank, RegClass, RegClassData, RegInfo};

include!(concat!(env!("OUT_DIR"), "/registers-intel.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use isa::RegUnit;

    #[test]
    fn unit_encodings() {
        // The encoding of integer registers is not alphabetical.
        assert_eq!(INFO.parse_regunit("rax"), Some(0));
        assert_eq!(INFO.parse_regunit("rbx"), Some(3));
        assert_eq!(INFO.parse_regunit("rcx"), Some(1));
        assert_eq!(INFO.parse_regunit("rdx"), Some(2));
        assert_eq!(INFO.parse_regunit("rsi"), Some(6));
        assert_eq!(INFO.parse_regunit("rdi"), Some(7));
        assert_eq!(INFO.parse_regunit("rbp"), Some(5));
        assert_eq!(INFO.parse_regunit("rsp"), Some(4));
        assert_eq!(INFO.parse_regunit("r8"), Some(8));
        assert_eq!(INFO.parse_regunit("r15"), Some(15));

        assert_eq!(INFO.parse_regunit("xmm0"), Some(16));
        assert_eq!(INFO.parse_regunit("xmm15"), Some(31));
    }

    #[test]
    fn unit_names() {
        fn uname(ru: RegUnit) -> String {
            INFO.display_regunit(ru).to_string()
        }

        assert_eq!(uname(0), "%rax");
        assert_eq!(uname(3), "%rbx");
        assert_eq!(uname(1), "%rcx");
        assert_eq!(uname(2), "%rdx");
        assert_eq!(uname(6), "%rsi");
        assert_eq!(uname(7), "%rdi");
        assert_eq!(uname(5), "%rbp");
        assert_eq!(uname(4), "%rsp");
        assert_eq!(uname(8), "%r8");
        assert_eq!(uname(15), "%r15");
        assert_eq!(uname(16), "%xmm0");
        assert_eq!(uname(31), "%xmm15");
    }

    #[test]
    fn regclasses() {
        assert_eq!(GPR.intersect(GPR), Some(GPR.into()));
        assert_eq!(GPR.intersect(ABCD), Some(ABCD.into()));
        assert_eq!(GPR.intersect(FPR), None);
        assert_eq!(ABCD.intersect(GPR), Some(ABCD.into()));
        assert_eq!(ABCD.intersect(ABCD), Some(ABCD.into()));
        assert_eq!(ABCD.intersect(FPR), None);
        assert_eq!(FPR.intersect(FPR), Some(FPR.into()));
        assert_eq!(FPR.intersect(GPR), None);
        assert_eq!(FPR.intersect(ABCD), None);
    }
}
