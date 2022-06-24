#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0EN` writer - DAC Channel 0 Enable"]
pub type CH0EN_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 0>;
#[doc = "Field `CH0DIS` writer - DAC Channel 0 Disable"]
pub type CH0DIS_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 1>;
#[doc = "Field `CH1EN` writer - DAC Channel 1 Enable"]
pub type CH1EN_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 4>;
#[doc = "Field `CH1DIS` writer - DAC Channel 1 Disable"]
pub type CH1DIS_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 5>;
#[doc = "Field `CH0FIFOFLUSH` writer - CH0 WFIFO Flush"]
pub type CH0FIFOFLUSH_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 8>;
#[doc = "Field `CH1FIFOFLUSH` writer - CH1 WFIFO Flush"]
pub type CH1FIFOFLUSH_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 9>;
#[doc = "Field `SINEMODESTART` writer - Start Sine Wave Generation"]
pub type SINEMODESTART_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 10>;
#[doc = "Field `SINEMODESTOP` writer - Stop Sine Wave Generation"]
pub type SINEMODESTOP_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 11>;
impl W {
    #[doc = "Bit 0 - DAC Channel 0 Enable"]
    #[inline(always)]
    pub fn ch0en(&mut self) -> CH0EN_W {
        CH0EN_W::new(self)
    }
    #[doc = "Bit 1 - DAC Channel 0 Disable"]
    #[inline(always)]
    pub fn ch0dis(&mut self) -> CH0DIS_W {
        CH0DIS_W::new(self)
    }
    #[doc = "Bit 4 - DAC Channel 1 Enable"]
    #[inline(always)]
    pub fn ch1en(&mut self) -> CH1EN_W {
        CH1EN_W::new(self)
    }
    #[doc = "Bit 5 - DAC Channel 1 Disable"]
    #[inline(always)]
    pub fn ch1dis(&mut self) -> CH1DIS_W {
        CH1DIS_W::new(self)
    }
    #[doc = "Bit 8 - CH0 WFIFO Flush"]
    #[inline(always)]
    pub fn ch0fifoflush(&mut self) -> CH0FIFOFLUSH_W {
        CH0FIFOFLUSH_W::new(self)
    }
    #[doc = "Bit 9 - CH1 WFIFO Flush"]
    #[inline(always)]
    pub fn ch1fifoflush(&mut self) -> CH1FIFOFLUSH_W {
        CH1FIFOFLUSH_W::new(self)
    }
    #[doc = "Bit 10 - Start Sine Wave Generation"]
    #[inline(always)]
    pub fn sinemodestart(&mut self) -> SINEMODESTART_W {
        SINEMODESTART_W::new(self)
    }
    #[doc = "Bit 11 - Stop Sine Wave Generation"]
    #[inline(always)]
    pub fn sinemodestop(&mut self) -> SINEMODESTOP_W {
        SINEMODESTOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
