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
#[doc = "Field `PPUPRIV` reader - PPU Privilege Interrupt Flag"]
pub type PPUPRIV_R = crate::BitReader<bool>;
#[doc = "Field `PPUPRIV` writer - PPU Privilege Interrupt Flag"]
pub type PPUPRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `PPUINST` reader - PPU Instruction Interrupt Flag"]
pub type PPUINST_R = crate::BitReader<bool>;
#[doc = "Field `PPUINST` writer - PPU Instruction Interrupt Flag"]
pub type PPUINST_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `PPUSEC` reader - PPU Security Interrupt Flag"]
pub type PPUSEC_R = crate::BitReader<bool>;
#[doc = "Field `PPUSEC` writer - PPU Security Interrupt Flag"]
pub type PPUSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
#[doc = "Field `BMPUSEC` reader - BMPU Security Interrupt Flag"]
pub type BMPUSEC_R = crate::BitReader<bool>;
#[doc = "Field `BMPUSEC` writer - BMPU Security Interrupt Flag"]
pub type BMPUSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - PPU Privilege Interrupt Flag"]
    #[inline(always)]
    pub fn ppupriv(&self) -> PPUPRIV_R {
        PPUPRIV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PPU Instruction Interrupt Flag"]
    #[inline(always)]
    pub fn ppuinst(&self) -> PPUINST_R {
        PPUINST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - PPU Security Interrupt Flag"]
    #[inline(always)]
    pub fn ppusec(&self) -> PPUSEC_R {
        PPUSEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BMPU Security Interrupt Flag"]
    #[inline(always)]
    pub fn bmpusec(&self) -> BMPUSEC_R {
        BMPUSEC_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PPU Privilege Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ppupriv(&mut self) -> PPUPRIV_W<0> {
        PPUPRIV_W::new(self)
    }
    #[doc = "Bit 2 - PPU Instruction Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ppuinst(&mut self) -> PPUINST_W<2> {
        PPUINST_W::new(self)
    }
    #[doc = "Bit 16 - PPU Security Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ppusec(&mut self) -> PPUSEC_W<16> {
        PPUSEC_W::new(self)
    }
    #[doc = "Bit 17 - BMPU Security Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bmpusec(&mut self) -> BMPUSEC_W<17> {
        BMPUSEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Read to get status of SMU interrupts.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
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
