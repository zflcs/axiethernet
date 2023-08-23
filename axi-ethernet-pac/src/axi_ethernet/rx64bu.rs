#[doc = "Register `rx64bu` reader"]
pub struct R(crate::R<RX64BU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RX64BU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RX64BU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RX64BU_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<RX64BU_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Count of 64 bytes frames received, MSM\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rx64bu](index.html) module"]
pub struct RX64BU_SPEC;
impl crate::RegisterSpec for RX64BU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rx64bu::R](R) reader structure"]
impl crate::Readable for RX64BU_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets rx64bu to value 0"]
impl crate::Resettable for RX64BU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
