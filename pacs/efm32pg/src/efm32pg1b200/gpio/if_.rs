#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EXT` reader - External Pin Interrupt Flag"]
pub type EXT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `EM4WU` reader - EM4 Wake Up Pin Interrupt Flag"]
pub type EM4WU_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - External Pin Interrupt Flag"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - EM4 Wake Up Pin Interrupt Flag"]
    #[inline(always)]
    pub fn em4wu(&self) -> EM4WU_R {
        EM4WU_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
