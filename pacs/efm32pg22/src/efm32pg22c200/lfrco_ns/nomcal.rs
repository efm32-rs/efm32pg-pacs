#[doc = "Register `NOMCAL` reader"]
pub struct R(crate::R<NOMCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NOMCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NOMCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NOMCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NOMCAL` writer"]
pub struct W(crate::W<NOMCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NOMCAL_SPEC>;
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
impl From<crate::W<NOMCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NOMCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NOMCALCNT` reader - Nominal Calibration Count"]
pub type NOMCALCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `NOMCALCNT` writer - Nominal Calibration Count"]
pub type NOMCALCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NOMCAL_SPEC, u32, u32, 21, O>;
impl R {
    #[doc = "Bits 0:20 - Nominal Calibration Count"]
    #[inline(always)]
    pub fn nomcalcnt(&self) -> NOMCALCNT_R {
        NOMCALCNT_R::new(self.bits & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:20 - Nominal Calibration Count"]
    #[inline(always)]
    #[must_use]
    pub fn nomcalcnt(&mut self) -> NOMCALCNT_W<0> {
        NOMCALCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Nominal calibration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nomcal](index.html) module"]
pub struct NOMCAL_SPEC;
impl crate::RegisterSpec for NOMCAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nomcal::R](R) reader structure"]
impl crate::Readable for NOMCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nomcal::W](W) writer structure"]
impl crate::Writable for NOMCAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NOMCAL to value 0x0005_b8d8"]
impl crate::Resettable for NOMCAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0005_b8d8;
}
