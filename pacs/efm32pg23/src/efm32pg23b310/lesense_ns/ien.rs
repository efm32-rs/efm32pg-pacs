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
#[doc = "Field `CH0` reader - Channel"]
pub type CH0_R = crate::BitReader<bool>;
#[doc = "Field `CH0` writer - Channel"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH1` reader - Channel"]
pub type CH1_R = crate::BitReader<bool>;
#[doc = "Field `CH1` writer - Channel"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH2` reader - Channel"]
pub type CH2_R = crate::BitReader<bool>;
#[doc = "Field `CH2` writer - Channel"]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH3` reader - Channel"]
pub type CH3_R = crate::BitReader<bool>;
#[doc = "Field `CH3` writer - Channel"]
pub type CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH4` reader - Channel"]
pub type CH4_R = crate::BitReader<bool>;
#[doc = "Field `CH4` writer - Channel"]
pub type CH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH5` reader - Channel"]
pub type CH5_R = crate::BitReader<bool>;
#[doc = "Field `CH5` writer - Channel"]
pub type CH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH6` reader - Channel"]
pub type CH6_R = crate::BitReader<bool>;
#[doc = "Field `CH6` writer - Channel"]
pub type CH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH7` reader - Channel"]
pub type CH7_R = crate::BitReader<bool>;
#[doc = "Field `CH7` writer - Channel"]
pub type CH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH8` reader - Channel"]
pub type CH8_R = crate::BitReader<bool>;
#[doc = "Field `CH8` writer - Channel"]
pub type CH8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH9` reader - Channel"]
pub type CH9_R = crate::BitReader<bool>;
#[doc = "Field `CH9` writer - Channel"]
pub type CH9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH10` reader - Channel"]
pub type CH10_R = crate::BitReader<bool>;
#[doc = "Field `CH10` writer - Channel"]
pub type CH10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH11` reader - Channel"]
pub type CH11_R = crate::BitReader<bool>;
#[doc = "Field `CH11` writer - Channel"]
pub type CH11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH12` reader - Channel"]
pub type CH12_R = crate::BitReader<bool>;
#[doc = "Field `CH12` writer - Channel"]
pub type CH12_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH13` reader - Channel"]
pub type CH13_R = crate::BitReader<bool>;
#[doc = "Field `CH13` writer - Channel"]
pub type CH13_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH14` reader - Channel"]
pub type CH14_R = crate::BitReader<bool>;
#[doc = "Field `CH14` writer - Channel"]
pub type CH14_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CH15` reader - Channel"]
pub type CH15_R = crate::BitReader<bool>;
#[doc = "Field `CH15` writer - Channel"]
pub type CH15_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `SCANDONE` reader - Scan Complete"]
pub type SCANDONE_R = crate::BitReader<bool>;
#[doc = "Field `SCANDONE` writer - Scan Complete"]
pub type SCANDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `DEC` reader - Decoder"]
pub type DEC_R = crate::BitReader<bool>;
#[doc = "Field `DEC` writer - Decoder"]
pub type DEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `RESWL` reader - Result Watermark Level"]
pub type RESWL_R = crate::BitReader<bool>;
#[doc = "Field `RESWL` writer - Result Watermark Level"]
pub type RESWL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `RESOF` reader - Result Overflow"]
pub type RESOF_R = crate::BitReader<bool>;
#[doc = "Field `RESOF` writer - Result Overflow"]
pub type RESOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CNTOF` reader - Counter Overflow"]
pub type CNTOF_R = crate::BitReader<bool>;
#[doc = "Field `CNTOF` writer - Counter Overflow"]
pub type CNTOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `RESUF` reader - Result Underflow"]
pub type RESUF_R = crate::BitReader<bool>;
#[doc = "Field `RESUF` writer - Result Underflow"]
pub type RESUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Channel"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Channel"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Channel"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Channel"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Channel"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Scan Complete"]
    #[inline(always)]
    pub fn scandone(&self) -> SCANDONE_R {
        SCANDONE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Decoder"]
    #[inline(always)]
    pub fn dec(&self) -> DEC_R {
        DEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Result Watermark Level"]
    #[inline(always)]
    pub fn reswl(&self) -> RESWL_R {
        RESWL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Result Overflow"]
    #[inline(always)]
    pub fn resof(&self) -> RESOF_R {
        RESOF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Counter Overflow"]
    #[inline(always)]
    pub fn cntof(&self) -> CNTOF_R {
        CNTOF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Result Underflow"]
    #[inline(always)]
    pub fn resuf(&self) -> RESUF_R {
        RESUF_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 4 - Channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<4> {
        CH4_W::new(self)
    }
    #[doc = "Bit 5 - Channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<5> {
        CH5_W::new(self)
    }
    #[doc = "Bit 6 - Channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<6> {
        CH6_W::new(self)
    }
    #[doc = "Bit 7 - Channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<7> {
        CH7_W::new(self)
    }
    #[doc = "Bit 8 - Channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> CH8_W<8> {
        CH8_W::new(self)
    }
    #[doc = "Bit 9 - Channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> CH9_W<9> {
        CH9_W::new(self)
    }
    #[doc = "Bit 10 - Channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> CH10_W<10> {
        CH10_W::new(self)
    }
    #[doc = "Bit 11 - Channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> CH11_W<11> {
        CH11_W::new(self)
    }
    #[doc = "Bit 12 - Channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> CH12_W<12> {
        CH12_W::new(self)
    }
    #[doc = "Bit 13 - Channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> CH13_W<13> {
        CH13_W::new(self)
    }
    #[doc = "Bit 14 - Channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> CH14_W<14> {
        CH14_W::new(self)
    }
    #[doc = "Bit 15 - Channel"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> CH15_W<15> {
        CH15_W::new(self)
    }
    #[doc = "Bit 16 - Scan Complete"]
    #[inline(always)]
    #[must_use]
    pub fn scandone(&mut self) -> SCANDONE_W<16> {
        SCANDONE_W::new(self)
    }
    #[doc = "Bit 17 - Decoder"]
    #[inline(always)]
    #[must_use]
    pub fn dec(&mut self) -> DEC_W<17> {
        DEC_W::new(self)
    }
    #[doc = "Bit 18 - Result Watermark Level"]
    #[inline(always)]
    #[must_use]
    pub fn reswl(&mut self) -> RESWL_W<18> {
        RESWL_W::new(self)
    }
    #[doc = "Bit 19 - Result Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn resof(&mut self) -> RESOF_W<19> {
        RESOF_W::new(self)
    }
    #[doc = "Bit 20 - Counter Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn cntof(&mut self) -> CNTOF_W<20> {
        CNTOF_W::new(self)
    }
    #[doc = "Bit 21 - Result Underflow"]
    #[inline(always)]
    #[must_use]
    pub fn resuf(&mut self) -> RESUF_W<21> {
        RESUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enables\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
