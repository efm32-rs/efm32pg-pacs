#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXC` reader - TX Complete Enable"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXC` writer - TX Complete Enable"]
pub type TXC_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `TXFL` reader - TX FIFO Level Enable"]
pub type TXFL_R = crate::BitReader<bool>;
#[doc = "Field `TXFL` writer - TX FIFO Level Enable"]
pub type TXFL_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `RXFL` reader - RX FIFO Level Enable"]
pub type RXFL_R = crate::BitReader<bool>;
#[doc = "Field `RXFL` writer - RX FIFO Level Enable"]
pub type RXFL_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
#[doc = "Field `RXFULL` reader - RX FIFO Full Enable"]
pub type RXFULL_R = crate::BitReader<bool>;
#[doc = "Field `RXFULL` writer - RX FIFO Full Enable"]
pub type RXFULL_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 3>;
#[doc = "Field `RXOF` reader - RX FIFO Overflow Enable"]
pub type RXOF_R = crate::BitReader<bool>;
#[doc = "Field `RXOF` writer - RX FIFO Overflow Enable"]
pub type RXOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 4>;
#[doc = "Field `RXUF` reader - RX FIFO Underflow Enable"]
pub type RXUF_R = crate::BitReader<bool>;
#[doc = "Field `RXUF` writer - RX FIFO Underflow Enable"]
pub type RXUF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 5>;
#[doc = "Field `TXOF` reader - TX FIFO Overflow Enable"]
pub type TXOF_R = crate::BitReader<bool>;
#[doc = "Field `TXOF` writer - TX FIFO Overflow Enable"]
pub type TXOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 6>;
#[doc = "Field `TXUF` reader - TX FIFO Underflow Enable"]
pub type TXUF_R = crate::BitReader<bool>;
#[doc = "Field `TXUF` writer - TX FIFO Underflow Enable"]
pub type TXUF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 7>;
#[doc = "Field `PERR` reader - Parity Error Enable"]
pub type PERR_R = crate::BitReader<bool>;
#[doc = "Field `PERR` writer - Parity Error Enable"]
pub type PERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 8>;
#[doc = "Field `FERR` reader - Framing Error Enable"]
pub type FERR_R = crate::BitReader<bool>;
#[doc = "Field `FERR` writer - Framing Error Enable"]
pub type FERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 9>;
#[doc = "Field `MPAF` reader - Multi-Processor Addr Frame Enable"]
pub type MPAF_R = crate::BitReader<bool>;
#[doc = "Field `MPAF` writer - Multi-Processor Addr Frame Enable"]
pub type MPAF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 10>;
#[doc = "Field `LOADERR` reader - Load Error Enable"]
pub type LOADERR_R = crate::BitReader<bool>;
#[doc = "Field `LOADERR` writer - Load Error Enable"]
pub type LOADERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 11>;
#[doc = "Field `CCF` reader - Collision Check Fail Enable"]
pub type CCF_R = crate::BitReader<bool>;
#[doc = "Field `CCF` writer - Collision Check Fail Enable"]
pub type CCF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 12>;
#[doc = "Field `TXIDLE` reader - TX IDLE Enable"]
pub type TXIDLE_R = crate::BitReader<bool>;
#[doc = "Field `TXIDLE` writer - TX IDLE Enable"]
pub type TXIDLE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 13>;
#[doc = "Field `STARTF` reader - Start Frame Enable"]
pub type STARTF_R = crate::BitReader<bool>;
#[doc = "Field `STARTF` writer - Start Frame Enable"]
pub type STARTF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 18>;
#[doc = "Field `SIGF` reader - Signal Frame Enable"]
pub type SIGF_R = crate::BitReader<bool>;
#[doc = "Field `SIGF` writer - Signal Frame Enable"]
pub type SIGF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 19>;
#[doc = "Field `AUTOBAUDDONE` reader - Auto Baud Complete Enable"]
pub type AUTOBAUDDONE_R = crate::BitReader<bool>;
#[doc = "Field `AUTOBAUDDONE` writer - Auto Baud Complete Enable"]
pub type AUTOBAUDDONE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 24>;
#[doc = "Field `RXTO` reader - RX Timeout Enable"]
pub type RXTO_R = crate::BitReader<bool>;
#[doc = "Field `RXTO` writer - RX Timeout Enable"]
pub type RXTO_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 25>;
impl R {
    #[doc = "Bit 0 - TX Complete Enable"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Level Enable"]
    #[inline(always)]
    pub fn txfl(&self) -> TXFL_R {
        TXFL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FIFO Level Enable"]
    #[inline(always)]
    pub fn rxfl(&self) -> RXFL_R {
        RXFL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Full Enable"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX FIFO Overflow Enable"]
    #[inline(always)]
    pub fn rxof(&self) -> RXOF_R {
        RXOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX FIFO Underflow Enable"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX FIFO Overflow Enable"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TX FIFO Underflow Enable"]
    #[inline(always)]
    pub fn txuf(&self) -> TXUF_R {
        TXUF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Parity Error Enable"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Framing Error Enable"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Multi-Processor Addr Frame Enable"]
    #[inline(always)]
    pub fn mpaf(&self) -> MPAF_R {
        MPAF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Load Error Enable"]
    #[inline(always)]
    pub fn loaderr(&self) -> LOADERR_R {
        LOADERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Collision Check Fail Enable"]
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TX IDLE Enable"]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 18 - Start Frame Enable"]
    #[inline(always)]
    pub fn startf(&self) -> STARTF_R {
        STARTF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Signal Frame Enable"]
    #[inline(always)]
    pub fn sigf(&self) -> SIGF_R {
        SIGF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto Baud Complete Enable"]
    #[inline(always)]
    pub fn autobauddone(&self) -> AUTOBAUDDONE_R {
        AUTOBAUDDONE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RX Timeout Enable"]
    #[inline(always)]
    pub fn rxto(&self) -> RXTO_R {
        RXTO_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Complete Enable"]
    #[inline(always)]
    pub fn txc(&mut self) -> TXC_W {
        TXC_W::new(self)
    }
    #[doc = "Bit 1 - TX FIFO Level Enable"]
    #[inline(always)]
    pub fn txfl(&mut self) -> TXFL_W {
        TXFL_W::new(self)
    }
    #[doc = "Bit 2 - RX FIFO Level Enable"]
    #[inline(always)]
    pub fn rxfl(&mut self) -> RXFL_W {
        RXFL_W::new(self)
    }
    #[doc = "Bit 3 - RX FIFO Full Enable"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RXFULL_W {
        RXFULL_W::new(self)
    }
    #[doc = "Bit 4 - RX FIFO Overflow Enable"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RXOF_W {
        RXOF_W::new(self)
    }
    #[doc = "Bit 5 - RX FIFO Underflow Enable"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RXUF_W {
        RXUF_W::new(self)
    }
    #[doc = "Bit 6 - TX FIFO Overflow Enable"]
    #[inline(always)]
    pub fn txof(&mut self) -> TXOF_W {
        TXOF_W::new(self)
    }
    #[doc = "Bit 7 - TX FIFO Underflow Enable"]
    #[inline(always)]
    pub fn txuf(&mut self) -> TXUF_W {
        TXUF_W::new(self)
    }
    #[doc = "Bit 8 - Parity Error Enable"]
    #[inline(always)]
    pub fn perr(&mut self) -> PERR_W {
        PERR_W::new(self)
    }
    #[doc = "Bit 9 - Framing Error Enable"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FERR_W {
        FERR_W::new(self)
    }
    #[doc = "Bit 10 - Multi-Processor Addr Frame Enable"]
    #[inline(always)]
    pub fn mpaf(&mut self) -> MPAF_W {
        MPAF_W::new(self)
    }
    #[doc = "Bit 11 - Load Error Enable"]
    #[inline(always)]
    pub fn loaderr(&mut self) -> LOADERR_W {
        LOADERR_W::new(self)
    }
    #[doc = "Bit 12 - Collision Check Fail Enable"]
    #[inline(always)]
    pub fn ccf(&mut self) -> CCF_W {
        CCF_W::new(self)
    }
    #[doc = "Bit 13 - TX IDLE Enable"]
    #[inline(always)]
    pub fn txidle(&mut self) -> TXIDLE_W {
        TXIDLE_W::new(self)
    }
    #[doc = "Bit 18 - Start Frame Enable"]
    #[inline(always)]
    pub fn startf(&mut self) -> STARTF_W {
        STARTF_W::new(self)
    }
    #[doc = "Bit 19 - Signal Frame Enable"]
    #[inline(always)]
    pub fn sigf(&mut self) -> SIGF_W {
        SIGF_W::new(self)
    }
    #[doc = "Bit 24 - Auto Baud Complete Enable"]
    #[inline(always)]
    pub fn autobauddone(&mut self) -> AUTOBAUDDONE_W {
        AUTOBAUDDONE_W::new(self)
    }
    #[doc = "Bit 25 - RX Timeout Enable"]
    #[inline(always)]
    pub fn rxto(&mut self) -> RXTO_W {
        RXTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
