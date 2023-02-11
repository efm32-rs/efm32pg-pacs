#[doc = "Register `SCHED1` reader"]
pub struct R(crate::R<SCHED1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCHED1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCHED1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCHED1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCHED1` writer"]
pub struct W(crate::W<SCHED1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCHED1_SPEC>;
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
impl From<crate::W<SCHED1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCHED1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRESCALE` reader - Prescale"]
pub type PRESCALE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PRESCALE` writer - Prescale"]
pub type PRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCHED1_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Prescale"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Prescale"]
    #[inline(always)]
    #[must_use]
    pub fn prescale(&mut self) -> PRESCALE_W<0> {
        PRESCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scheduling\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sched1](index.html) module"]
pub struct SCHED1_SPEC;
impl crate::RegisterSpec for SCHED1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sched1::R](R) reader structure"]
impl crate::Readable for SCHED1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sched1::W](W) writer structure"]
impl crate::Writable for SCHED1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCHED1 to value 0"]
impl crate::Resettable for SCHED1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
