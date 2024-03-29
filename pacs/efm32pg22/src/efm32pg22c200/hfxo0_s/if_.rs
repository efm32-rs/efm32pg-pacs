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
#[doc = "Register `IF` writer"]
pub struct W(crate::W<IF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF_SPEC>;
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
impl From<crate::W<IF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDY` reader - Ready Interrupt"]
pub type RDY_R = crate::BitReader<bool>;
#[doc = "Field `RDY` writer - Ready Interrupt"]
pub type RDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `COREBIASOPTRDY` reader - Core Bias Optimization Ready Interrupt"]
pub type COREBIASOPTRDY_R = crate::BitReader<bool>;
#[doc = "Field `COREBIASOPTRDY` writer - Core Bias Optimization Ready Interrupt"]
pub type COREBIASOPTRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `DNSERR` reader - Did Not Start Error Interrupt"]
pub type DNSERR_R = crate::BitReader<bool>;
#[doc = "Field `DNSERR` writer - Did Not Start Error Interrupt"]
pub type DNSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `COREBIASOPTERR` reader - Core Bias Optimization Error Interrupt"]
pub type COREBIASOPTERR_R = crate::BitReader<bool>;
#[doc = "Field `COREBIASOPTERR` writer - Core Bias Optimization Error Interrupt"]
pub type COREBIASOPTERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Ready Interrupt"]
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core Bias Optimization Ready Interrupt"]
    #[inline(always)]
    pub fn corebiasoptrdy(&self) -> COREBIASOPTRDY_R {
        COREBIASOPTRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 29 - Did Not Start Error Interrupt"]
    #[inline(always)]
    pub fn dnserr(&self) -> DNSERR_R {
        DNSERR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Core Bias Optimization Error Interrupt"]
    #[inline(always)]
    pub fn corebiasopterr(&self) -> COREBIASOPTERR_R {
        COREBIASOPTERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ready Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rdy(&mut self) -> RDY_W<0> {
        RDY_W::new(self)
    }
    #[doc = "Bit 1 - Core Bias Optimization Ready Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn corebiasoptrdy(&mut self) -> COREBIASOPTRDY_W<1> {
        COREBIASOPTRDY_W::new(self)
    }
    #[doc = "Bit 29 - Did Not Start Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dnserr(&mut self) -> DNSERR_W<29> {
        DNSERR_W::new(self)
    }
    #[doc = "Bit 31 - Core Bias Optimization Error Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn corebiasopterr(&mut self) -> COREBIASOPTERR_W<31> {
        COREBIASOPTERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if_::W](W) writer structure"]
impl crate::Writable for IF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
