#[doc = "Register `USART1_CLKROUTE` reader"]
pub struct R(crate::R<USART1_CLKROUTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USART1_CLKROUTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USART1_CLKROUTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USART1_CLKROUTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USART1_CLKROUTE` writer"]
pub struct W(crate::W<USART1_CLKROUTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USART1_CLKROUTE_SPEC>;
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
impl From<crate::W<USART1_CLKROUTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USART1_CLKROUTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PORT` reader - SCLK port select register"]
pub type PORT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PORT` writer - SCLK port select register"]
pub type PORT_W<'a> = crate::FieldWriter<'a, u32, USART1_CLKROUTE_SPEC, u8, u8, 2, 0>;
#[doc = "Field `PIN` reader - SCLK pin select register"]
pub type PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN` writer - SCLK pin select register"]
pub type PIN_W<'a> = crate::FieldWriter<'a, u32, USART1_CLKROUTE_SPEC, u8, u8, 4, 16>;
impl R {
    #[doc = "Bits 0:1 - SCLK port select register"]
    #[inline(always)]
    pub fn port(&self) -> PORT_R {
        PORT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - SCLK pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SCLK port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PORT_W {
        PORT_W::new(self)
    }
    #[doc = "Bits 16:19 - SCLK pin select register"]
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
#[doc = "SCLK port/pin select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usart1_clkroute](index.html) module"]
pub struct USART1_CLKROUTE_SPEC;
impl crate::RegisterSpec for USART1_CLKROUTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usart1_clkroute::R](R) reader structure"]
impl crate::Readable for USART1_CLKROUTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usart1_clkroute::W](W) writer structure"]
impl crate::Writable for USART1_CLKROUTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USART1_CLKROUTE to value 0"]
impl crate::Resettable for USART1_CLKROUTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
