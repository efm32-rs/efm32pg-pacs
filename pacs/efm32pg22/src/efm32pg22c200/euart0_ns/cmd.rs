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
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 0>;
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RXDIS_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 1>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TXEN_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 2>;
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TXDIS_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 3>;
#[doc = "Field `RXBLOCKEN` writer - Receiver Block Enable"]
pub type RXBLOCKEN_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 4>;
#[doc = "Field `RXBLOCKDIS` writer - Receiver Block Disable"]
pub type RXBLOCKDIS_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 5>;
#[doc = "Field `TXTRIEN` writer - Transmitter Tristate Enable"]
pub type TXTRIEN_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 6>;
#[doc = "Field `TXTRIDIS` writer - Transmitter Tristate Disable"]
pub type TXTRIDIS_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 7>;
#[doc = "Field `CLEARTX` writer - Clear TX FIFO"]
pub type CLEARTX_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 8>;
impl W {
    #[doc = "Bit 0 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W {
        RXEN_W::new(self)
    }
    #[doc = "Bit 1 - Receiver Disable"]
    #[inline(always)]
    pub fn rxdis(&mut self) -> RXDIS_W {
        RXDIS_W::new(self)
    }
    #[doc = "Bit 2 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W {
        TXEN_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter Disable"]
    #[inline(always)]
    pub fn txdis(&mut self) -> TXDIS_W {
        TXDIS_W::new(self)
    }
    #[doc = "Bit 4 - Receiver Block Enable"]
    #[inline(always)]
    pub fn rxblocken(&mut self) -> RXBLOCKEN_W {
        RXBLOCKEN_W::new(self)
    }
    #[doc = "Bit 5 - Receiver Block Disable"]
    #[inline(always)]
    pub fn rxblockdis(&mut self) -> RXBLOCKDIS_W {
        RXBLOCKDIS_W::new(self)
    }
    #[doc = "Bit 6 - Transmitter Tristate Enable"]
    #[inline(always)]
    pub fn txtrien(&mut self) -> TXTRIEN_W {
        TXTRIEN_W::new(self)
    }
    #[doc = "Bit 7 - Transmitter Tristate Disable"]
    #[inline(always)]
    pub fn txtridis(&mut self) -> TXTRIDIS_W {
        TXTRIDIS_W::new(self)
    }
    #[doc = "Bit 8 - Clear TX FIFO"]
    #[inline(always)]
    pub fn cleartx(&mut self) -> CLEARTX_W {
        CLEARTX_W::new(self)
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
