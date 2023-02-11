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
#[doc = "Field `ERASE` reader - Erase Done Interrupt enable"]
pub type ERASE_R = crate::BitReader<bool>;
#[doc = "Field `ERASE` writer - Erase Done Interrupt enable"]
pub type ERASE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `WRITE` reader - Write Done Interrupt enable"]
pub type WRITE_R = crate::BitReader<bool>;
#[doc = "Field `WRITE` writer - Write Done Interrupt enable"]
pub type WRITE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `WDATAOV` reader - write data buffer overflow irq enable"]
pub type WDATAOV_R = crate::BitReader<bool>;
#[doc = "Field `WDATAOV` writer - write data buffer overflow irq enable"]
pub type WDATAOV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `PWRUPF` reader - Flash Power Up Seq done irq enable"]
pub type PWRUPF_R = crate::BitReader<bool>;
#[doc = "Field `PWRUPF` writer - Flash Power Up Seq done irq enable"]
pub type PWRUPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `PWROFF` reader - Flash Power Off Seq done irq enable"]
pub type PWROFF_R = crate::BitReader<bool>;
#[doc = "Field `PWROFF` writer - Flash Power Off Seq done irq enable"]
pub type PWROFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Erase Done Interrupt enable"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Done Interrupt enable"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - write data buffer overflow irq enable"]
    #[inline(always)]
    pub fn wdataov(&self) -> WDATAOV_R {
        WDATAOV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash Power Up Seq done irq enable"]
    #[inline(always)]
    pub fn pwrupf(&self) -> PWRUPF_R {
        PWRUPF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flash Power Off Seq done irq enable"]
    #[inline(always)]
    pub fn pwroff(&self) -> PWROFF_R {
        PWROFF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Erase Done Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn erase(&mut self) -> ERASE_W<0> {
        ERASE_W::new(self)
    }
    #[doc = "Bit 1 - Write Done Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn write(&mut self) -> WRITE_W<1> {
        WRITE_W::new(self)
    }
    #[doc = "Bit 2 - write data buffer overflow irq enable"]
    #[inline(always)]
    #[must_use]
    pub fn wdataov(&mut self) -> WDATAOV_W<2> {
        WDATAOV_W::new(self)
    }
    #[doc = "Bit 8 - Flash Power Up Seq done irq enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwrupf(&mut self) -> PWRUPF_W<8> {
        PWRUPF_W::new(self)
    }
    #[doc = "Bit 9 - Flash Power Off Seq done irq enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwroff(&mut self) -> PWROFF_W<9> {
        PWROFF_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
