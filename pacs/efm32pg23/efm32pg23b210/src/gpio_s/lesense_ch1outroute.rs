#[doc = "Register `LESENSE_CH1OUTROUTE` reader"]
pub struct R(crate::R<LESENSE_CH1OUTROUTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LESENSE_CH1OUTROUTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LESENSE_CH1OUTROUTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LESENSE_CH1OUTROUTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LESENSE_CH1OUTROUTE` writer"]
pub struct W(crate::W<LESENSE_CH1OUTROUTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LESENSE_CH1OUTROUTE_SPEC>;
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
impl From<crate::W<LESENSE_CH1OUTROUTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LESENSE_CH1OUTROUTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORT` reader - CH1OUT port select register"]
pub type PORT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORT` writer - CH1OUT port select register"]
pub type PORT_W<'a> = crate::FieldWriter<'a, u32, LESENSE_CH1OUTROUTE_SPEC, u8, u8, 2, 0>;
#[doc = "Field `PIN` reader - CH1OUT pin select register"]
pub type PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN` writer - CH1OUT pin select register"]
pub type PIN_W<'a> = crate::FieldWriter<'a, u32, LESENSE_CH1OUTROUTE_SPEC, u8, u8, 4, 16>;
impl R {
    #[doc = "Bits 0:1 - CH1OUT port select register"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - CH1OUT pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CH1OUT port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W {
        PORT_W::new(self)
    }
    #[doc = "Bits 16:19 - CH1OUT pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PIN_W {
        PIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CH1OUT port/pin select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lesense_ch1outroute](index.html) module"]
pub struct LESENSE_CH1OUTROUTE_SPEC;
impl crate::RegisterSpec for LESENSE_CH1OUTROUTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lesense_ch1outroute::R](R) reader structure"]
impl crate::Readable for LESENSE_CH1OUTROUTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lesense_ch1outroute::W](W) writer structure"]
impl crate::Writable for LESENSE_CH1OUTROUTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LESENSE_CH1OUTROUTE to value 0"]
impl crate::Resettable for LESENSE_CH1OUTROUTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
