#[doc = "Register `INSENSE` reader"]
pub struct R(crate::R<INSENSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INSENSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INSENSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INSENSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INSENSE` writer"]
pub struct W(crate::W<INSENSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INSENSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INSENSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INSENSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT` reader - Interrupt Sense Enable"]
pub type INT_R = crate::BitReader<bool>;
#[doc = "Field `INT` writer - Interrupt Sense Enable"]
pub type INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, INSENSE_SPEC, bool, O>;
#[doc = "Field `EM4WU` reader - EM4WU Interrupt Sense Enable"]
pub type EM4WU_R = crate::BitReader<bool>;
#[doc = "Field `EM4WU` writer - EM4WU Interrupt Sense Enable"]
pub type EM4WU_W<'a, const O: u8> = crate::BitWriter<'a, u32, INSENSE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Interrupt Sense Enable"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EM4WU Interrupt Sense Enable"]
    #[inline(always)]
    pub fn em4wu(&self) -> EM4WU_R {
        EM4WU_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Sense Enable"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<0> {
        INT_W::new(self)
    }
    #[doc = "Bit 1 - EM4WU Interrupt Sense Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em4wu(&mut self) -> EM4WU_W<1> {
        EM4WU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Sense Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [insense](index.html) module"]
pub struct INSENSE_SPEC;
impl crate::RegisterSpec for INSENSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [insense::R](R) reader structure"]
impl crate::Readable for INSENSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [insense::W](W) writer structure"]
impl crate::Writable for INSENSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INSENSE to value 0x03"]
impl crate::Resettable for INSENSE_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
