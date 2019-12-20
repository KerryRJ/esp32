#[doc = "Reader of register I2S_PLC_CONF1_REG(i)"]
pub type R = crate::R<u32, super::I2S_PLC_CONF1_REGI>;
#[doc = "Writer for register I2S_PLC_CONF1_REG(i)"]
pub type W = crate::W<u32, super::I2S_PLC_CONF1_REGI>;
#[doc = "Register I2S_PLC_CONF1_REG(i) `reset()`'s with value 0"]
impl crate::ResetValue for super::I2S_PLC_CONF1_REGI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_SLIDE_WIN_LEN`"]
pub type I2S_SLIDE_WIN_LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_SLIDE_WIN_LEN`"]
pub struct I2S_SLIDE_WIN_LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_SLIDE_WIN_LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `I2S_BAD_OLA_WIN2_PARA`"]
pub type I2S_BAD_OLA_WIN2_PARA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_BAD_OLA_WIN2_PARA`"]
pub struct I2S_BAD_OLA_WIN2_PARA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_BAD_OLA_WIN2_PARA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `I2S_BAD_OLA_WIN2_PARA_SHIFT`"]
pub type I2S_BAD_OLA_WIN2_PARA_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_BAD_OLA_WIN2_PARA_SHIFT`"]
pub struct I2S_BAD_OLA_WIN2_PARA_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_BAD_OLA_WIN2_PARA_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `I2S_BAD_CEF_ATTEN_PARA_SHIFT`"]
pub type I2S_BAD_CEF_ATTEN_PARA_SHIFT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_BAD_CEF_ATTEN_PARA_SHIFT`"]
pub struct I2S_BAD_CEF_ATTEN_PARA_SHIFT_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_BAD_CEF_ATTEN_PARA_SHIFT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `I2S_BAD_CEF_ATTEN_PARA`"]
pub type I2S_BAD_CEF_ATTEN_PARA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2S_BAD_CEF_ATTEN_PARA`"]
pub struct I2S_BAD_CEF_ATTEN_PARA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_BAD_CEF_ATTEN_PARA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn i2s_slide_win_len(&self) -> I2S_SLIDE_WIN_LEN_R {
        I2S_SLIDE_WIN_LEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2s_bad_ola_win2_para(&self) -> I2S_BAD_OLA_WIN2_PARA_R {
        I2S_BAD_OLA_WIN2_PARA_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn i2s_bad_ola_win2_para_shift(&self) -> I2S_BAD_OLA_WIN2_PARA_SHIFT_R {
        I2S_BAD_OLA_WIN2_PARA_SHIFT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn i2s_bad_cef_atten_para_shift(&self) -> I2S_BAD_CEF_ATTEN_PARA_SHIFT_R {
        I2S_BAD_CEF_ATTEN_PARA_SHIFT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn i2s_bad_cef_atten_para(&self) -> I2S_BAD_CEF_ATTEN_PARA_R {
        I2S_BAD_CEF_ATTEN_PARA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn i2s_slide_win_len(&mut self) -> I2S_SLIDE_WIN_LEN_W {
        I2S_SLIDE_WIN_LEN_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn i2s_bad_ola_win2_para(&mut self) -> I2S_BAD_OLA_WIN2_PARA_W {
        I2S_BAD_OLA_WIN2_PARA_W { w: self }
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn i2s_bad_ola_win2_para_shift(&mut self) -> I2S_BAD_OLA_WIN2_PARA_SHIFT_W {
        I2S_BAD_OLA_WIN2_PARA_SHIFT_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn i2s_bad_cef_atten_para_shift(&mut self) -> I2S_BAD_CEF_ATTEN_PARA_SHIFT_W {
        I2S_BAD_CEF_ATTEN_PARA_SHIFT_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn i2s_bad_cef_atten_para(&mut self) -> I2S_BAD_CEF_ATTEN_PARA_W {
        I2S_BAD_CEF_ATTEN_PARA_W { w: self }
    }
}
