#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACMPACT` reader - Analog Comparator Active"]
pub type ACMPACT_R = crate::BitReader<bool>;
#[doc = "Field `ACMPOUT` reader - Analog Comparator Output"]
pub type ACMPOUT_R = crate::BitReader<bool>;
#[doc = "Field `APORTCONFLICT` reader - APORT Conflict Output"]
pub type APORTCONFLICT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Analog Comparator Active"]
    #[inline(always)]
    pub fn acmpact(&self) -> ACMPACT_R {
        ACMPACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator Output"]
    #[inline(always)]
    pub fn acmpout(&self) -> ACMPOUT_R {
        ACMPOUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - APORT Conflict Output"]
    #[inline(always)]
    pub fn aportconflict(&self) -> APORTCONFLICT_R {
        APORTCONFLICT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
