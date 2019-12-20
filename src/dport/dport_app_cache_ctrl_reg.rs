#[doc = "Reader of register DPORT_APP_CACHE_CTRL_REG"]
pub type R = crate::R<u32, super::DPORT_APP_CACHE_CTRL_REG>;
#[doc = "Writer for register DPORT_APP_CACHE_CTRL_REG"]
pub type W = crate::W<u32, super::DPORT_APP_CACHE_CTRL_REG>;
#[doc = "Register DPORT_APP_CACHE_CTRL_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_APP_CACHE_CTRL_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_APP_DRAM_HL`"]
pub type DPORT_APP_DRAM_HL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_DRAM_HL`"]
pub struct DPORT_APP_DRAM_HL_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_DRAM_HL_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `DPORT_APP_SLAVE_REQ`"]
pub type DPORT_APP_SLAVE_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_SLAVE_REQ`"]
pub struct DPORT_APP_SLAVE_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_SLAVE_REQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `DPORT_APP_AHB_SPI_REQ`"]
pub type DPORT_APP_AHB_SPI_REQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_AHB_SPI_REQ`"]
pub struct DPORT_APP_AHB_SPI_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_AHB_SPI_REQ_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `DPORT_APP_DRAM_SPLIT`"]
pub type DPORT_APP_DRAM_SPLIT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_DRAM_SPLIT`"]
pub struct DPORT_APP_DRAM_SPLIT_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_DRAM_SPLIT_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `DPORT_APP_SINGLE_IRAM_ENA`"]
pub type DPORT_APP_SINGLE_IRAM_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_SINGLE_IRAM_ENA`"]
pub struct DPORT_APP_SINGLE_IRAM_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_SINGLE_IRAM_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DPORT_APP_CACHE_LOCK_3_EN`"]
pub type DPORT_APP_CACHE_LOCK_3_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_CACHE_LOCK_3_EN`"]
pub struct DPORT_APP_CACHE_LOCK_3_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_CACHE_LOCK_3_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DPORT_APP_CACHE_LOCK_2_EN`"]
pub type DPORT_APP_CACHE_LOCK_2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_CACHE_LOCK_2_EN`"]
pub struct DPORT_APP_CACHE_LOCK_2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_CACHE_LOCK_2_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DPORT_APP_CACHE_LOCK_1_EN`"]
pub type DPORT_APP_CACHE_LOCK_1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_CACHE_LOCK_1_EN`"]
pub struct DPORT_APP_CACHE_LOCK_1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_CACHE_LOCK_1_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `DPORT_APP_CACHE_LOCK_0_EN`"]
pub type DPORT_APP_CACHE_LOCK_0_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_CACHE_LOCK_0_EN`"]
pub struct DPORT_APP_CACHE_LOCK_0_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_CACHE_LOCK_0_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DPORT_APP_CACHE_FLUSH_DONE`"]
pub type DPORT_APP_CACHE_FLUSH_DONE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_CACHE_FLUSH_DONE`"]
pub struct DPORT_APP_CACHE_FLUSH_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_CACHE_FLUSH_DONE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `DPORT_APP_CACHE_FLUSH_ENA`"]
pub type DPORT_APP_CACHE_FLUSH_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_CACHE_FLUSH_ENA`"]
pub struct DPORT_APP_CACHE_FLUSH_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_CACHE_FLUSH_ENA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DPORT_APP_CACHE_ENABLE`"]
pub type DPORT_APP_CACHE_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_CACHE_ENABLE`"]
pub struct DPORT_APP_CACHE_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_CACHE_ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `DPORT_APP_CACHE_MODE`"]
pub type DPORT_APP_CACHE_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_APP_CACHE_MODE`"]
pub struct DPORT_APP_CACHE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_APP_CACHE_MODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dport_app_dram_hl(&self) -> DPORT_APP_DRAM_HL_R {
        DPORT_APP_DRAM_HL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dport_app_slave_req(&self) -> DPORT_APP_SLAVE_REQ_R {
        DPORT_APP_SLAVE_REQ_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dport_app_ahb_spi_req(&self) -> DPORT_APP_AHB_SPI_REQ_R {
        DPORT_APP_AHB_SPI_REQ_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dport_app_dram_split(&self) -> DPORT_APP_DRAM_SPLIT_R {
        DPORT_APP_DRAM_SPLIT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dport_app_single_iram_ena(&self) -> DPORT_APP_SINGLE_IRAM_ENA_R {
        DPORT_APP_SINGLE_IRAM_ENA_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dport_app_cache_lock_3_en(&self) -> DPORT_APP_CACHE_LOCK_3_EN_R {
        DPORT_APP_CACHE_LOCK_3_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dport_app_cache_lock_2_en(&self) -> DPORT_APP_CACHE_LOCK_2_EN_R {
        DPORT_APP_CACHE_LOCK_2_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dport_app_cache_lock_1_en(&self) -> DPORT_APP_CACHE_LOCK_1_EN_R {
        DPORT_APP_CACHE_LOCK_1_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dport_app_cache_lock_0_en(&self) -> DPORT_APP_CACHE_LOCK_0_EN_R {
        DPORT_APP_CACHE_LOCK_0_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dport_app_cache_flush_done(&self) -> DPORT_APP_CACHE_FLUSH_DONE_R {
        DPORT_APP_CACHE_FLUSH_DONE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dport_app_cache_flush_ena(&self) -> DPORT_APP_CACHE_FLUSH_ENA_R {
        DPORT_APP_CACHE_FLUSH_ENA_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dport_app_cache_enable(&self) -> DPORT_APP_CACHE_ENABLE_R {
        DPORT_APP_CACHE_ENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dport_app_cache_mode(&self) -> DPORT_APP_CACHE_MODE_R {
        DPORT_APP_CACHE_MODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn dport_app_dram_hl(&mut self) -> DPORT_APP_DRAM_HL_W {
        DPORT_APP_DRAM_HL_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn dport_app_slave_req(&mut self) -> DPORT_APP_SLAVE_REQ_W {
        DPORT_APP_SLAVE_REQ_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dport_app_ahb_spi_req(&mut self) -> DPORT_APP_AHB_SPI_REQ_W {
        DPORT_APP_AHB_SPI_REQ_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn dport_app_dram_split(&mut self) -> DPORT_APP_DRAM_SPLIT_W {
        DPORT_APP_DRAM_SPLIT_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dport_app_single_iram_ena(&mut self) -> DPORT_APP_SINGLE_IRAM_ENA_W {
        DPORT_APP_SINGLE_IRAM_ENA_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dport_app_cache_lock_3_en(&mut self) -> DPORT_APP_CACHE_LOCK_3_EN_W {
        DPORT_APP_CACHE_LOCK_3_EN_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dport_app_cache_lock_2_en(&mut self) -> DPORT_APP_CACHE_LOCK_2_EN_W {
        DPORT_APP_CACHE_LOCK_2_EN_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn dport_app_cache_lock_1_en(&mut self) -> DPORT_APP_CACHE_LOCK_1_EN_W {
        DPORT_APP_CACHE_LOCK_1_EN_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn dport_app_cache_lock_0_en(&mut self) -> DPORT_APP_CACHE_LOCK_0_EN_W {
        DPORT_APP_CACHE_LOCK_0_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn dport_app_cache_flush_done(&mut self) -> DPORT_APP_CACHE_FLUSH_DONE_W {
        DPORT_APP_CACHE_FLUSH_DONE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dport_app_cache_flush_ena(&mut self) -> DPORT_APP_CACHE_FLUSH_ENA_W {
        DPORT_APP_CACHE_FLUSH_ENA_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn dport_app_cache_enable(&mut self) -> DPORT_APP_CACHE_ENABLE_W {
        DPORT_APP_CACHE_ENABLE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn dport_app_cache_mode(&mut self) -> DPORT_APP_CACHE_MODE_W {
        DPORT_APP_CACHE_MODE_W { w: self }
    }
}
