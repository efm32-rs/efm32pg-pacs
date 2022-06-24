#[doc = "Register `CC2_OC` reader"]
pub struct R(crate::R<CC2_OC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC2_OC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC2_OC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC2_OC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC2_OC` writer"]
pub struct W(crate::W<CC2_OC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC2_OC_SPEC>;
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
impl From<crate::W<CC2_OC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC2_OC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OC` reader - Output Compare Value"]
pub type OC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OC` writer - Output Compare Value"]
pub type OC_W<'a> = crate::FieldWriter<'a, u32, CC2_OC_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Output Compare Value"]
    #[inline(always)]
    pub fn oc(&self) -> OC_R {
        OC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output Compare Value"]
    #[inline(always)]
    pub fn oc(&mut self) -> OC_W {
        OC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc2_oc](index.html) module"]
pub struct CC2_OC_SPEC;
impl crate::RegisterSpec for CC2_OC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc2_oc::R](R) reader structure"]
impl crate::Readable for CC2_OC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc2_oc::W](W) writer structure"]
impl crate::Writable for CC2_OC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC2_OC to value 0"]
impl crate::Resettable for CC2_OC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
