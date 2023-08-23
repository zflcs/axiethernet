#[doc = "Register `txmcu` reader"]
pub struct R(crate::R<TXMCU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXMCU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXMCU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXMCU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<TXMCU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Multiple Collision frames Transmitted OK, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txmcu](index.html) module"]
pub struct TXMCU_SPEC;
impl crate::RegisterSpec for TXMCU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txmcu::R](R) reader structure"]
impl crate::Readable for TXMCU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets txmcu to value 0"]
impl crate::Resettable for TXMCU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
