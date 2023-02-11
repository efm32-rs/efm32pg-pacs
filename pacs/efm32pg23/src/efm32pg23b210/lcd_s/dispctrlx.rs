#[doc = "Register `DISPCTRLX` reader"]
pub struct R(crate::R<DISPCTRLX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DISPCTRLX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DISPCTRLX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DISPCTRLX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DISPCTRLX` writer"]
pub struct W(crate::W<DISPCTRLX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DISPCTRLX_SPEC>;
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
impl From<crate::W<DISPCTRLX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DISPCTRLX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISPLAYDIV` reader - Display Divider"]
pub type DISPLAYDIV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DISPLAYDIV` writer - Display Divider"]
pub type DISPLAYDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DISPCTRLX_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Display Divider"]
    #[inline(always)]
    pub fn displaydiv(&self) -> DISPLAYDIV_R {
        DISPLAYDIV_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Display Divider"]
    #[inline(always)]
    #[must_use]
    pub fn displaydiv(&mut self) -> DISPLAYDIV_W<0> {
        DISPLAYDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dispctrlx](index.html) module"]
pub struct DISPCTRLX_SPEC;
impl crate::RegisterSpec for DISPCTRLX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dispctrlx::R](R) reader structure"]
impl crate::Readable for DISPCTRLX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dispctrlx::W](W) writer structure"]
impl crate::Writable for DISPCTRLX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DISPCTRLX to value 0"]
impl crate::Resettable for DISPCTRLX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
