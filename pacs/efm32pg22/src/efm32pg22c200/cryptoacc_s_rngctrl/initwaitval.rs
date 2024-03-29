#[doc = "Register `INITWAITVAL` reader"]
pub struct R(crate::R<INITWAITVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INITWAITVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INITWAITVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INITWAITVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INITWAITVAL` writer"]
pub struct W(crate::W<INITWAITVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INITWAITVAL_SPEC>;
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
impl From<crate::W<INITWAITVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INITWAITVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INITWAITVAL` reader - Wait counter value"]
pub type INITWAITVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INITWAITVAL` writer - Wait counter value"]
pub type INITWAITVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, INITWAITVAL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Wait counter value"]
    #[inline(always)]
    pub fn initwaitval(&self) -> INITWAITVAL_R {
        INITWAITVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Wait counter value"]
    #[inline(always)]
    #[must_use]
    pub fn initwaitval(&mut self) -> INITWAITVAL_W<0> {
        INITWAITVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [initwaitval](index.html) module"]
pub struct INITWAITVAL_SPEC;
impl crate::RegisterSpec for INITWAITVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [initwaitval::R](R) reader structure"]
impl crate::Readable for INITWAITVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [initwaitval::W](W) writer structure"]
impl crate::Writable for INITWAITVAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INITWAITVAL to value 0xffff"]
impl crate::Resettable for INITWAITVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
