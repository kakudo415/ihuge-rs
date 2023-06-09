use std::ops;

use crate::{dword, uHuge, word};

impl ops::Mul for &uHuge {
    type Output = uHuge;

    fn mul(self, rhs: Self) -> Self::Output {
        let len = self.digits.len() + rhs.digits.len();
        let mut digits = vec![0; len];
        mul_nn(&mut digits, &self.digits, &rhs.digits);
        uHuge { digits }.pop_leading_zeros()
    }
}

pub(crate) fn mul_nn(acc: &mut [word], lhs: &[word], rhs: &[word]) {
    for a in acc.iter_mut() {
        *a = 0;
    }
    for (i, ld) in lhs.iter().enumerate() {
        let mut carry = 0;
        for (j, rd) in rhs.iter().enumerate() {
            (acc[i + j], carry) = pred_carrying_mul(*ld, *rd, acc[i + j], carry);
        }
        acc[i + rhs.len()] = carry;
    }
}

pub(crate) fn mul_assign_n1(acc: &mut [word], d: word) {
    let mut carry = 0;
    for a in acc {
        (*a, carry) = carrying_mul(*a, d, carry);
    }
}

// carry + pred + lhs * rhs = (ans, carry)
pub(crate) fn pred_carrying_mul(lhs: word, rhs: word, pred: word, carry: word) -> (word, word) {
    let acc: dword = carry as dword + pred as dword + lhs as dword * rhs as dword;
    (acc as word, (acc >> word::BITS) as word)
}

// carry + lhs * rhs = (ans, carry)
pub(crate) fn carrying_mul(lhs: word, rhs: word, carry: word) -> (word, word) {
    let carry = carry as dword;
    let lhs = lhs as dword;
    let rhs = rhs as dword;
    let acc = carry + lhs * rhs;
    (acc as word, (acc >> word::BITS) as word)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pred_carrying_mul_0() {
        let lhs = word::MAX;
        let rhs = word::MAX;
        let pred = word::MAX;
        let carry = word::MAX;
        let ans = (word::MAX, word::MAX);
        assert_eq!(pred_carrying_mul(lhs, rhs, pred, carry), ans);
    }

    #[test]
    fn mul_0() {
        let lhs = uHuge::from_str("FFFFFFFFFFFFFFFF").unwrap();
        let rhs = uHuge::from_str("FFFFFFFFFFFFFFFF").unwrap();
        let ans = uHuge::from_str("FFFFFFFFFFFFFFFE0000000000000001").unwrap();
        assert_eq!(&lhs * &rhs, ans);
    }

    #[test]
    fn mul_1() {
        let lhs = uHuge::from_str("FFFFFFFFFFFFFFFF").unwrap();
        let rhs =
            uHuge::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF")
                .unwrap();
        let ans = uHuge::from_str(
            "FFFFFFFFFFFFFFFEFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF0000000000000001",
        )
        .unwrap();
        assert_eq!(&lhs * &rhs, ans);
    }

    #[test]
    fn mul_2() {
        let lhs = uHuge::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF").unwrap();
        let rhs = uHuge::from_str(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
        )
        .unwrap();
        let ans = uHuge::from_str("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF000000000000000000000000000000000000000000000001").unwrap();
        assert_eq!(&lhs * &rhs, ans);
    }

    #[test]
    fn mul_3() {
        let lhs = uHuge::from_str("000000000000000100000000000000010000000000000001").unwrap();
        let rhs = uHuge::from_str(
            "00000000000000010000000000000001000000000000000100000000000000010000000000000001",
        )
        .unwrap();
        let ans = uHuge::from_str("0000000000000001000000000000000200000000000000030000000000000003000000000000000300000000000000020000000000000001").unwrap();
        assert_eq!(&lhs * &rhs, ans);
    }

    #[test]
    fn mul_random_0() {
        let lhs = uHuge::from_str("95b551bd29bcee4f6eb47816f9aa54e7169e297b0b9aafb7ec3c04b3c212dbb0aa37810837d818a4bb1efd93bb8f57155fa409a292f8c8e40b088c93e72").unwrap();
        let rhs = uHuge::from_str("b68e2e738aff247097542e6c5c54c2e20659340a53e99a2daa05becbab178c09532ead8dc870c78176a5c7a4f911cb1ad1e0f9cdccf531dd6cce5b5578c").unwrap();
        let ans = uHuge::from_str("6AC20DD9F90620AE16B2FF327AB8FB8AF40808C9F6177A047EEB08E6AA7055E4FB9DFD71A28C069360D21CC7CBE9C0E32E48A92C477081F13C5AB1B8F4F18FE6D7B872382DB40D1CA8BBE8AFFB75A47C8229C4B11AF243F00786EE706F0FF40F53E52F716EEE136C269C3927F7C612234AA275206F290452E0E458").unwrap();
        assert_eq!(&lhs * &rhs, ans);
    }

    #[test]
    fn mul_random_1() {
        let lhs = uHuge::from_str("f04ddab160a06e4aac853fdba85f07f0774d2bef099d2213bb0ddd9a8dc55898c1a013845313604dbd74cd088e4c711325d3b1fe558e2428beb291d670b").unwrap();
        let rhs = uHuge::from_str("f181775e9243c97911d58d0dedf04eeb4616a12130e2b85580bdd00af30f2766584e517b2b202d0f1fb2d4194a456ed2b97203ca307e6909ff61417485").unwrap();
        let ans = uHuge::from_str("E2B2D22925E381F019FEA22D08EB44E453D1EEDBA8FDDD119197361FFBB6D05B6A54DA70585B501E3C2EF8D3A318902052EE4E812F791E234F6EF71B87F252A4BCCE7CD426B0B34D61E1BEBF24079F2631C6E946254B55AAC2497C1EA2C33D69F639C20C48B4E248A21F585EE7E97F1A31123FFAEDB4503C284B7").unwrap();
        assert_eq!(&lhs * &rhs, ans);
    }

    #[test]
    fn mul_random_2() {
        let lhs = uHuge::from_str("87996ffc6afd5a035e5a692c3cb16254d33106a41b083cb572b9310949e81fac7eed0186e95ab638bcbc2a5498fe2e2e916f7328a8988f393cca3b2961edbadf3cd6a6a2c2100bf9089910d9c89bb9a334be8a6a9fd4e989eca0e6d49c76bf995668e212").unwrap();
        let rhs = uHuge::from_str("684abfaa193dbf853277ab252c936054948030930996f8ef106025675bf08a2b88446d428fcc3dd4bda9c6f76072974041af73b4de1d0e78842837d4c71a3b987deb01b2e3254f5e104e725bf566b54b2c8d1be2c0ca62662245ddbf3224d6fc27222d49").unwrap();
        let ans = uHuge::from_str("373DED5E7E3E3C30D34C637768927C849C73BF11834E93A650963C34BB47CFE0E9A067BDC9CBD066171E04347DD0C22E6E2C7BA1B5EAF66BB7FF03BE0293ECE2AA1D68D54115D8B061A92217C5876C888AC4E54C4E5D079B084DE43A8D97CE865D503D4737B06FBCD7573A1EDE0947B84196D304F83C02D1F7C2FACFC6DBD48AFE482B0C701355B2C5E8EC9EA2DD75B95FCAE7BC7253FB802A55DC91D01EED3324257033CD093BF0968A14CE7CD9B08EAFF756EB19CFD52C2C477021524BB29956D3A100D809A122").unwrap();
        assert_eq!(&lhs * &rhs, ans);
    }

    #[test]
    fn mul_random_3() {
        let lhs = uHuge::from_str("d57e45844c0c77317f2b29444d8132fc15418a0c2b09d3c4a344b6ff5bb12ea1de451fbf44da07159438dd190b9e9f70c8b0113fcf71171d68ed3efabda47402574a1f12e992f9d1b01f4afe99059673d7c56f2f70151fdff7b220019126e648b53424ac76736081a24e2e4ecaf2c2ea589c1924198081aceb79eb1ca25ea08307dae57a14759b1dd16dbdb44dd4784aab5d8c3479c92a11ea3a344a3be28a352a41184124358a7ddc2de8f134a44b8ef8986946898b0c844c3a3b3a7341bf6fcbad0d41f5872010").unwrap();
        let rhs = uHuge::from_str("f493f637b4d6ebe51b35d2101ebd67b62c87badedec8c2bc3e87f9a9052e2362839909a741d4704a5ff356bd62e7d689f4e5628d54a5ea88b5d3a3169a95d0ccf0974196caa29ee71adc34faead439b2caf71fde2170ee55448b0d8421a2ce6560df6775e17cdf8509a6b87c576c981db61d39337c89a2752325adf9f208a9d0e07c40baba65fee4fc276c567b44aa6fc65345cb2d32ff8751274e3e7edc31f90f2ae22a125ad2cfcf5ce931265cb4a7a70c173eb6ae36b786fef2b21f604516c0f8ebe5f43de742").unwrap();
        let ans = uHuge::from_str("CBF7BF19CF31A26D0C5E7FFDE0578AB5AD8531CB328EC0B8244806A28E0583C75B9CA9CD508F5A448D08E25276CD4D4FC7275AF51CFF5344724052D4CB71A87F57A38041EA7A30E52FD076C6C12DE8F94C921E1809320632282CC1AB04151AE074A936709B2F5FA71EA6F9299F9E4D2D8AE13B8C0B9BD7A9B793445CE3ADAC76E9DC4F7583E4050045527750381BAD42F41A23232B05E6C551FAC60C486E84FF81866BF1F26DE9C248060F7754EC5F8F56DCC0D45018601F5A9D630D133DC54FA90BD497A7764F23F40A5B38B5877F4F348FD798C728212C6B42377590BFA9F5907BF5225E2FC81B80CE2A6653D19D457ABFAE90C164DFC66BEB0FC7C003B507D367A6A6606828DF918A7C4070B4B3C7A3978088641E119CACAC648378C85A84B6C760364B9CBB05BFC0DBFCE6BB5748CA61D0EBB0D27510605662DE62FC7E768B3AAD98AE79CA03AB7724EDFF0E69CCEF8BB682848216051BB8F91E1FDBC59EC3C473387C5E8F18DE640CA7045DCA68FD63C73C25F3E0E3CBB447CCE3BBFED585E4E2490EBA61316C1E99A01E94B420").unwrap();
        assert_eq!(&lhs * &rhs, ans);
    }
}
