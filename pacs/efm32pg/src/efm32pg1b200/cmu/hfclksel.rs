#[doc = "Register `HFCLKSEL` writer"]
pub struct W(crate::W<HFCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFCLKSEL_SPEC>;
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
impl From<crate::W<HFCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HFCLK Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HF_AW {
    #[doc = "1: Select HFRCO as HFCLK"]
    HFRCO = 1,
    #[doc = "2: Select HFXO as HFCLK"]
    HFXO = 2,
    #[doc = "3: Select LFRCO as HFCLK"]
    LFRCO = 3,
    #[doc = "4: Select LFXO as HFCLK"]
    LFXO = 4,
}
impl From<HF_AW> for u8 {
    #[inline(always)]
    fn from(variant: HF_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `HF` writer - HFCLK Select"]
pub type HF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HFCLKSEL_SPEC, u8, HF_AW, 3, O>;
impl<'a, const O: u8> HF_W<'a, O> {
    #[doc = "Select HFRCO as HFCLK"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(HF_AW::HFRCO)
    }
    #[doc = "Select HFXO as HFCLK"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(HF_AW::HFXO)
    }
    #[doc = "Select LFRCO as HFCLK"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(HF_AW::LFRCO)
    }
    #[doc = "Select LFXO as HFCLK"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(HF_AW::LFXO)
    }
}
impl W {
    #[doc = "Bits 0:2 - HFCLK Select"]
    #[inline(always)]
    #[must_use]
    pub fn hf(&mut self) -> HF_W<0> {
        HF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Frequency Clock Select Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfclksel](index.html) module"]
pub struct HFCLKSEL_SPEC;
impl crate::RegisterSpec for HFCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hfclksel::W](W) writer structure"]
impl crate::Writable for HFCLKSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HFCLKSEL to value 0"]
impl crate::Resettable for HFCLKSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
