#[doc = "Register `INCL_IPS_HW_CFG` reader"]
pub struct R(crate::R<INCL_IPS_HW_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INCL_IPS_HW_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INCL_IPS_HW_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INCL_IPS_HW_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `g_IncludeAES` reader - Generic g_IncludeAES value"]
pub type G_INCLUDE_AES_R = crate::BitReader<bool>;
#[doc = "Field `g_IncludeAESGCM` reader - Generic g_IncludeAESGCM value"]
pub type G_INCLUDE_AESGCM_R = crate::BitReader<bool>;
#[doc = "Field `g_IncludeAESXTS` reader - Generic g_IncludeAESXTS value"]
pub type G_INCLUDE_AESXTS_R = crate::BitReader<bool>;
#[doc = "Field `g_IncludeDES` reader - Generic g_IncludeDES value"]
pub type G_INCLUDE_DES_R = crate::BitReader<bool>;
#[doc = "Field `g_IncludeHASH` reader - Generic g_IncludeHASH value"]
pub type G_INCLUDE_HASH_R = crate::BitReader<bool>;
#[doc = "Field `g_IncludeChachaPoly` reader - Generic g_IncludeChachaPoly value"]
pub type G_INCLUDE_CHACHA_POLY_R = crate::BitReader<bool>;
#[doc = "Field `g_IncludeSHA3` reader - Generic g_IncludeSHA3 value"]
pub type G_INCLUDE_SHA3_R = crate::BitReader<bool>;
#[doc = "Field `g_IncludeZUC` reader - Generic g_IncludeZUC value"]
pub type G_INCLUDE_ZUC_R = crate::BitReader<bool>;
#[doc = "Field `g_IncludeSM4` reader - Generic g_IncludeSM4 value"]
pub type G_INCLUDE_SM4_R = crate::BitReader<bool>;
#[doc = "Field `g_IncludePKE` reader - Generic g_IncludePKE value"]
pub type G_INCLUDE_PKE_R = crate::BitReader<bool>;
#[doc = "Field `g_IncludeNDRNG` reader - Generic g_IncludeNDRNG value"]
pub type G_INCLUDE_NDRNG_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Generic g_IncludeAES value"]
    #[inline(always)]
    pub fn g_include_aes(&self) -> G_INCLUDE_AES_R {
        G_INCLUDE_AES_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Generic g_IncludeAESGCM value"]
    #[inline(always)]
    pub fn g_include_aesgcm(&self) -> G_INCLUDE_AESGCM_R {
        G_INCLUDE_AESGCM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Generic g_IncludeAESXTS value"]
    #[inline(always)]
    pub fn g_include_aesxts(&self) -> G_INCLUDE_AESXTS_R {
        G_INCLUDE_AESXTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generic g_IncludeDES value"]
    #[inline(always)]
    pub fn g_include_des(&self) -> G_INCLUDE_DES_R {
        G_INCLUDE_DES_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Generic g_IncludeHASH value"]
    #[inline(always)]
    pub fn g_include_hash(&self) -> G_INCLUDE_HASH_R {
        G_INCLUDE_HASH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generic g_IncludeChachaPoly value"]
    #[inline(always)]
    pub fn g_include_chacha_poly(&self) -> G_INCLUDE_CHACHA_POLY_R {
        G_INCLUDE_CHACHA_POLY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Generic g_IncludeSHA3 value"]
    #[inline(always)]
    pub fn g_include_sha3(&self) -> G_INCLUDE_SHA3_R {
        G_INCLUDE_SHA3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generic g_IncludeZUC value"]
    #[inline(always)]
    pub fn g_include_zuc(&self) -> G_INCLUDE_ZUC_R {
        G_INCLUDE_ZUC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Generic g_IncludeSM4 value"]
    #[inline(always)]
    pub fn g_include_sm4(&self) -> G_INCLUDE_SM4_R {
        G_INCLUDE_SM4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generic g_IncludePKE value"]
    #[inline(always)]
    pub fn g_include_pke(&self) -> G_INCLUDE_PKE_R {
        G_INCLUDE_PKE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Generic g_IncludeNDRNG value"]
    #[inline(always)]
    pub fn g_include_ndrng(&self) -> G_INCLUDE_NDRNG_R {
        G_INCLUDE_NDRNG_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "No Description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [incl_ips_hw_cfg](index.html) module"]
pub struct INCL_IPS_HW_CFG_SPEC;
impl crate::RegisterSpec for INCL_IPS_HW_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [incl_ips_hw_cfg::R](R) reader structure"]
impl crate::Readable for INCL_IPS_HW_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INCL_IPS_HW_CFG to value 0x0611"]
impl crate::Resettable for INCL_IPS_HW_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0611;
}
