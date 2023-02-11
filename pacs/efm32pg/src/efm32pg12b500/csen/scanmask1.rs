#[doc = "Register `SCANMASK1` reader"]
pub struct R(crate::R<SCANMASK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCANMASK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCANMASK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCANMASK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCANMASK1` writer"]
pub struct W(crate::W<SCANMASK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCANMASK1_SPEC>;
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
impl From<crate::W<SCANMASK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCANMASK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCANINPUTEN` reader - Scan Channel Mask."]
pub type SCANINPUTEN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SCANINPUTEN` writer - Scan Channel Mask."]
pub type SCANINPUTEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SCANMASK1_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Scan Channel Mask."]
    #[inline(always)]
    pub fn scaninputen(&self) -> SCANINPUTEN_R {
        SCANINPUTEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Scan Channel Mask."]
    #[inline(always)]
    #[must_use]
    pub fn scaninputen(&mut self) -> SCANINPUTEN_W<0> {
        SCANINPUTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan Channel Mask 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scanmask1](index.html) module"]
pub struct SCANMASK1_SPEC;
impl crate::RegisterSpec for SCANMASK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scanmask1::R](R) reader structure"]
impl crate::Readable for SCANMASK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scanmask1::W](W) writer structure"]
impl crate::Writable for SCANMASK1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCANMASK1 to value 0"]
impl crate::Resettable for SCANMASK1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
