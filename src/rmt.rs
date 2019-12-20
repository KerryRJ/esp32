#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 32usize],
    #[doc = "0x20 - RMT_CH0CONF0_REG"]
    pub rmt_ch0conf0_reg: RMT_CH0CONF0_REG,
    #[doc = "0x24 - RMT_CH0CONF1_REG"]
    pub rmt_ch0conf1_reg: RMT_CH0CONF1_REG,
    #[doc = "0x28 - RMT_CH1CONF0_REG"]
    pub rmt_ch1conf0_reg: RMT_CH1CONF0_REG,
    #[doc = "0x2c - RMT_CH1CONF1_REG"]
    pub rmt_ch1conf1_reg: RMT_CH1CONF1_REG,
    #[doc = "0x30 - RMT_CH2CONF0_REG"]
    pub rmt_ch2conf0_reg: RMT_CH2CONF0_REG,
    #[doc = "0x34 - RMT_CH2CONF1_REG"]
    pub rmt_ch2conf1_reg: RMT_CH2CONF1_REG,
    #[doc = "0x38 - RMT_CH3CONF0_REG"]
    pub rmt_ch3conf0_reg: RMT_CH3CONF0_REG,
    #[doc = "0x3c - RMT_CH3CONF1_REG"]
    pub rmt_ch3conf1_reg: RMT_CH3CONF1_REG,
    #[doc = "0x40 - RMT_CH4CONF0_REG"]
    pub rmt_ch4conf0_reg: RMT_CH4CONF0_REG,
    #[doc = "0x44 - RMT_CH4CONF1_REG"]
    pub rmt_ch4conf1_reg: RMT_CH4CONF1_REG,
    #[doc = "0x48 - RMT_CH5CONF0_REG"]
    pub rmt_ch5conf0_reg: RMT_CH5CONF0_REG,
    #[doc = "0x4c - RMT_CH5CONF1_REG"]
    pub rmt_ch5conf1_reg: RMT_CH5CONF1_REG,
    #[doc = "0x50 - RMT_CH6CONF0_REG"]
    pub rmt_ch6conf0_reg: RMT_CH6CONF0_REG,
    #[doc = "0x54 - RMT_CH6CONF1_REG"]
    pub rmt_ch6conf1_reg: RMT_CH6CONF1_REG,
    #[doc = "0x58 - RMT_CH7CONF0_REG"]
    pub rmt_ch7conf0_reg: RMT_CH7CONF0_REG,
    #[doc = "0x5c - RMT_CH7CONF1_REG"]
    pub rmt_ch7conf1_reg: RMT_CH7CONF1_REG,
    #[doc = "0x60 - RMT_CH0STATUS_REG"]
    pub rmt_ch0status_reg: RMT_CH0STATUS_REG,
    #[doc = "0x64 - RMT_CH1STATUS_REG"]
    pub rmt_ch1status_reg: RMT_CH1STATUS_REG,
    #[doc = "0x68 - RMT_CH2STATUS_REG"]
    pub rmt_ch2status_reg: RMT_CH2STATUS_REG,
    #[doc = "0x6c - RMT_CH3STATUS_REG"]
    pub rmt_ch3status_reg: RMT_CH3STATUS_REG,
    #[doc = "0x70 - RMT_CH4STATUS_REG"]
    pub rmt_ch4status_reg: RMT_CH4STATUS_REG,
    #[doc = "0x74 - RMT_CH5STATUS_REG"]
    pub rmt_ch5status_reg: RMT_CH5STATUS_REG,
    #[doc = "0x78 - RMT_CH6STATUS_REG"]
    pub rmt_ch6status_reg: RMT_CH6STATUS_REG,
    #[doc = "0x7c - RMT_CH7STATUS_REG"]
    pub rmt_ch7status_reg: RMT_CH7STATUS_REG,
    #[doc = "0x80 - RMT_CH0ADDR_REG"]
    pub rmt_ch0addr_reg: RMT_CH0ADDR_REG,
    #[doc = "0x84 - RMT_CH1ADDR_REG"]
    pub rmt_ch1addr_reg: RMT_CH1ADDR_REG,
    #[doc = "0x88 - RMT_CH2ADDR_REG"]
    pub rmt_ch2addr_reg: RMT_CH2ADDR_REG,
    #[doc = "0x8c - RMT_CH3ADDR_REG"]
    pub rmt_ch3addr_reg: RMT_CH3ADDR_REG,
    #[doc = "0x90 - RMT_CH4ADDR_REG"]
    pub rmt_ch4addr_reg: RMT_CH4ADDR_REG,
    #[doc = "0x94 - RMT_CH5ADDR_REG"]
    pub rmt_ch5addr_reg: RMT_CH5ADDR_REG,
    #[doc = "0x98 - RMT_CH6ADDR_REG"]
    pub rmt_ch6addr_reg: RMT_CH6ADDR_REG,
    #[doc = "0x9c - RMT_CH7ADDR_REG"]
    pub rmt_ch7addr_reg: RMT_CH7ADDR_REG,
    #[doc = "0xa0 - RMT_INT_RAW_REG"]
    pub rmt_int_raw_reg: RMT_INT_RAW_REG,
    #[doc = "0xa4 - RMT_INT_ST_REG"]
    pub rmt_int_st_reg: RMT_INT_ST_REG,
    #[doc = "0xa8 - RMT_INT_ENA_REG"]
    pub rmt_int_ena_reg: RMT_INT_ENA_REG,
    #[doc = "0xac - RMT_INT_CLR_REG"]
    pub rmt_int_clr_reg: RMT_INT_CLR_REG,
    #[doc = "0xb0 - RMT_CH0CARRIER_DUTY_REG"]
    pub rmt_ch0carrier_duty_reg: RMT_CH0CARRIER_DUTY_REG,
    #[doc = "0xb4 - RMT_CH1CARRIER_DUTY_REG"]
    pub rmt_ch1carrier_duty_reg: RMT_CH1CARRIER_DUTY_REG,
    #[doc = "0xb8 - RMT_CH2CARRIER_DUTY_REG"]
    pub rmt_ch2carrier_duty_reg: RMT_CH2CARRIER_DUTY_REG,
    #[doc = "0xbc - RMT_CH3CARRIER_DUTY_REG"]
    pub rmt_ch3carrier_duty_reg: RMT_CH3CARRIER_DUTY_REG,
    #[doc = "0xc0 - RMT_CH4CARRIER_DUTY_REG"]
    pub rmt_ch4carrier_duty_reg: RMT_CH4CARRIER_DUTY_REG,
    #[doc = "0xc4 - RMT_CH5CARRIER_DUTY_REG"]
    pub rmt_ch5carrier_duty_reg: RMT_CH5CARRIER_DUTY_REG,
    #[doc = "0xc8 - RMT_CH6CARRIER_DUTY_REG"]
    pub rmt_ch6carrier_duty_reg: RMT_CH6CARRIER_DUTY_REG,
    #[doc = "0xcc - RMT_CH7CARRIER_DUTY_REG"]
    pub rmt_ch7carrier_duty_reg: RMT_CH7CARRIER_DUTY_REG,
    #[doc = "0xd0 - RMT_CH0_TX_LIM_REG"]
    pub rmt_ch0_tx_lim_reg: RMT_CH0_TX_LIM_REG,
    #[doc = "0xd4 - RMT_CH1_TX_LIM_REG"]
    pub rmt_ch1_tx_lim_reg: RMT_CH1_TX_LIM_REG,
    #[doc = "0xd8 - RMT_CH2_TX_LIM_REG"]
    pub rmt_ch2_tx_lim_reg: RMT_CH2_TX_LIM_REG,
    #[doc = "0xdc - RMT_CH3_TX_LIM_REG"]
    pub rmt_ch3_tx_lim_reg: RMT_CH3_TX_LIM_REG,
    #[doc = "0xe0 - RMT_CH4_TX_LIM_REG"]
    pub rmt_ch4_tx_lim_reg: RMT_CH4_TX_LIM_REG,
    #[doc = "0xe4 - RMT_CH5_TX_LIM_REG"]
    pub rmt_ch5_tx_lim_reg: RMT_CH5_TX_LIM_REG,
    #[doc = "0xe8 - RMT_CH6_TX_LIM_REG"]
    pub rmt_ch6_tx_lim_reg: RMT_CH6_TX_LIM_REG,
    #[doc = "0xec - RMT_CH7_TX_LIM_REG"]
    pub rmt_ch7_tx_lim_reg: RMT_CH7_TX_LIM_REG,
    #[doc = "0xf0 - RMT_APB_CONF_REG"]
    pub rmt_apb_conf_reg: RMT_APB_CONF_REG,
    _reserved53: [u8; 8usize],
    #[doc = "0xfc - RMT_DATE_REG"]
    pub rmt_date_reg: RMT_DATE_REG,
}
#[doc = "RMT_CH0CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch0conf0_reg](rmt_ch0conf0_reg) module"]
pub type RMT_CH0CONF0_REG = crate::Reg<u32, _RMT_CH0CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH0CONF0_REG;
#[doc = "`read()` method returns [rmt_ch0conf0_reg::R](rmt_ch0conf0_reg::R) reader structure"]
impl crate::Readable for RMT_CH0CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch0conf0_reg::W](rmt_ch0conf0_reg::W) writer structure"]
impl crate::Writable for RMT_CH0CONF0_REG {}
#[doc = "RMT_CH0CONF0_REG"]
pub mod rmt_ch0conf0_reg;
#[doc = "RMT_CH0CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch0conf1_reg](rmt_ch0conf1_reg) module"]
pub type RMT_CH0CONF1_REG = crate::Reg<u32, _RMT_CH0CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH0CONF1_REG;
#[doc = "`read()` method returns [rmt_ch0conf1_reg::R](rmt_ch0conf1_reg::R) reader structure"]
impl crate::Readable for RMT_CH0CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch0conf1_reg::W](rmt_ch0conf1_reg::W) writer structure"]
impl crate::Writable for RMT_CH0CONF1_REG {}
#[doc = "RMT_CH0CONF1_REG"]
pub mod rmt_ch0conf1_reg;
#[doc = "RMT_CH1CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch1conf0_reg](rmt_ch1conf0_reg) module"]
pub type RMT_CH1CONF0_REG = crate::Reg<u32, _RMT_CH1CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH1CONF0_REG;
#[doc = "`read()` method returns [rmt_ch1conf0_reg::R](rmt_ch1conf0_reg::R) reader structure"]
impl crate::Readable for RMT_CH1CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch1conf0_reg::W](rmt_ch1conf0_reg::W) writer structure"]
impl crate::Writable for RMT_CH1CONF0_REG {}
#[doc = "RMT_CH1CONF0_REG"]
pub mod rmt_ch1conf0_reg;
#[doc = "RMT_CH1CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch1conf1_reg](rmt_ch1conf1_reg) module"]
pub type RMT_CH1CONF1_REG = crate::Reg<u32, _RMT_CH1CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH1CONF1_REG;
#[doc = "`read()` method returns [rmt_ch1conf1_reg::R](rmt_ch1conf1_reg::R) reader structure"]
impl crate::Readable for RMT_CH1CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch1conf1_reg::W](rmt_ch1conf1_reg::W) writer structure"]
impl crate::Writable for RMT_CH1CONF1_REG {}
#[doc = "RMT_CH1CONF1_REG"]
pub mod rmt_ch1conf1_reg;
#[doc = "RMT_CH2CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch2conf0_reg](rmt_ch2conf0_reg) module"]
pub type RMT_CH2CONF0_REG = crate::Reg<u32, _RMT_CH2CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH2CONF0_REG;
#[doc = "`read()` method returns [rmt_ch2conf0_reg::R](rmt_ch2conf0_reg::R) reader structure"]
impl crate::Readable for RMT_CH2CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch2conf0_reg::W](rmt_ch2conf0_reg::W) writer structure"]
impl crate::Writable for RMT_CH2CONF0_REG {}
#[doc = "RMT_CH2CONF0_REG"]
pub mod rmt_ch2conf0_reg;
#[doc = "RMT_CH2CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch2conf1_reg](rmt_ch2conf1_reg) module"]
pub type RMT_CH2CONF1_REG = crate::Reg<u32, _RMT_CH2CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH2CONF1_REG;
#[doc = "`read()` method returns [rmt_ch2conf1_reg::R](rmt_ch2conf1_reg::R) reader structure"]
impl crate::Readable for RMT_CH2CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch2conf1_reg::W](rmt_ch2conf1_reg::W) writer structure"]
impl crate::Writable for RMT_CH2CONF1_REG {}
#[doc = "RMT_CH2CONF1_REG"]
pub mod rmt_ch2conf1_reg;
#[doc = "RMT_CH3CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch3conf0_reg](rmt_ch3conf0_reg) module"]
pub type RMT_CH3CONF0_REG = crate::Reg<u32, _RMT_CH3CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH3CONF0_REG;
#[doc = "`read()` method returns [rmt_ch3conf0_reg::R](rmt_ch3conf0_reg::R) reader structure"]
impl crate::Readable for RMT_CH3CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch3conf0_reg::W](rmt_ch3conf0_reg::W) writer structure"]
impl crate::Writable for RMT_CH3CONF0_REG {}
#[doc = "RMT_CH3CONF0_REG"]
pub mod rmt_ch3conf0_reg;
#[doc = "RMT_CH3CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch3conf1_reg](rmt_ch3conf1_reg) module"]
pub type RMT_CH3CONF1_REG = crate::Reg<u32, _RMT_CH3CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH3CONF1_REG;
#[doc = "`read()` method returns [rmt_ch3conf1_reg::R](rmt_ch3conf1_reg::R) reader structure"]
impl crate::Readable for RMT_CH3CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch3conf1_reg::W](rmt_ch3conf1_reg::W) writer structure"]
impl crate::Writable for RMT_CH3CONF1_REG {}
#[doc = "RMT_CH3CONF1_REG"]
pub mod rmt_ch3conf1_reg;
#[doc = "RMT_CH4CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch4conf0_reg](rmt_ch4conf0_reg) module"]
pub type RMT_CH4CONF0_REG = crate::Reg<u32, _RMT_CH4CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH4CONF0_REG;
#[doc = "`read()` method returns [rmt_ch4conf0_reg::R](rmt_ch4conf0_reg::R) reader structure"]
impl crate::Readable for RMT_CH4CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch4conf0_reg::W](rmt_ch4conf0_reg::W) writer structure"]
impl crate::Writable for RMT_CH4CONF0_REG {}
#[doc = "RMT_CH4CONF0_REG"]
pub mod rmt_ch4conf0_reg;
#[doc = "RMT_CH4CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch4conf1_reg](rmt_ch4conf1_reg) module"]
pub type RMT_CH4CONF1_REG = crate::Reg<u32, _RMT_CH4CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH4CONF1_REG;
#[doc = "`read()` method returns [rmt_ch4conf1_reg::R](rmt_ch4conf1_reg::R) reader structure"]
impl crate::Readable for RMT_CH4CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch4conf1_reg::W](rmt_ch4conf1_reg::W) writer structure"]
impl crate::Writable for RMT_CH4CONF1_REG {}
#[doc = "RMT_CH4CONF1_REG"]
pub mod rmt_ch4conf1_reg;
#[doc = "RMT_CH5CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch5conf0_reg](rmt_ch5conf0_reg) module"]
pub type RMT_CH5CONF0_REG = crate::Reg<u32, _RMT_CH5CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH5CONF0_REG;
#[doc = "`read()` method returns [rmt_ch5conf0_reg::R](rmt_ch5conf0_reg::R) reader structure"]
impl crate::Readable for RMT_CH5CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch5conf0_reg::W](rmt_ch5conf0_reg::W) writer structure"]
impl crate::Writable for RMT_CH5CONF0_REG {}
#[doc = "RMT_CH5CONF0_REG"]
pub mod rmt_ch5conf0_reg;
#[doc = "RMT_CH5CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch5conf1_reg](rmt_ch5conf1_reg) module"]
pub type RMT_CH5CONF1_REG = crate::Reg<u32, _RMT_CH5CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH5CONF1_REG;
#[doc = "`read()` method returns [rmt_ch5conf1_reg::R](rmt_ch5conf1_reg::R) reader structure"]
impl crate::Readable for RMT_CH5CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch5conf1_reg::W](rmt_ch5conf1_reg::W) writer structure"]
impl crate::Writable for RMT_CH5CONF1_REG {}
#[doc = "RMT_CH5CONF1_REG"]
pub mod rmt_ch5conf1_reg;
#[doc = "RMT_CH6CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch6conf0_reg](rmt_ch6conf0_reg) module"]
pub type RMT_CH6CONF0_REG = crate::Reg<u32, _RMT_CH6CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH6CONF0_REG;
#[doc = "`read()` method returns [rmt_ch6conf0_reg::R](rmt_ch6conf0_reg::R) reader structure"]
impl crate::Readable for RMT_CH6CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch6conf0_reg::W](rmt_ch6conf0_reg::W) writer structure"]
impl crate::Writable for RMT_CH6CONF0_REG {}
#[doc = "RMT_CH6CONF0_REG"]
pub mod rmt_ch6conf0_reg;
#[doc = "RMT_CH6CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch6conf1_reg](rmt_ch6conf1_reg) module"]
pub type RMT_CH6CONF1_REG = crate::Reg<u32, _RMT_CH6CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH6CONF1_REG;
#[doc = "`read()` method returns [rmt_ch6conf1_reg::R](rmt_ch6conf1_reg::R) reader structure"]
impl crate::Readable for RMT_CH6CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch6conf1_reg::W](rmt_ch6conf1_reg::W) writer structure"]
impl crate::Writable for RMT_CH6CONF1_REG {}
#[doc = "RMT_CH6CONF1_REG"]
pub mod rmt_ch6conf1_reg;
#[doc = "RMT_CH7CONF0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch7conf0_reg](rmt_ch7conf0_reg) module"]
pub type RMT_CH7CONF0_REG = crate::Reg<u32, _RMT_CH7CONF0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH7CONF0_REG;
#[doc = "`read()` method returns [rmt_ch7conf0_reg::R](rmt_ch7conf0_reg::R) reader structure"]
impl crate::Readable for RMT_CH7CONF0_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch7conf0_reg::W](rmt_ch7conf0_reg::W) writer structure"]
impl crate::Writable for RMT_CH7CONF0_REG {}
#[doc = "RMT_CH7CONF0_REG"]
pub mod rmt_ch7conf0_reg;
#[doc = "RMT_CH7CONF1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch7conf1_reg](rmt_ch7conf1_reg) module"]
pub type RMT_CH7CONF1_REG = crate::Reg<u32, _RMT_CH7CONF1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH7CONF1_REG;
#[doc = "`read()` method returns [rmt_ch7conf1_reg::R](rmt_ch7conf1_reg::R) reader structure"]
impl crate::Readable for RMT_CH7CONF1_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch7conf1_reg::W](rmt_ch7conf1_reg::W) writer structure"]
impl crate::Writable for RMT_CH7CONF1_REG {}
#[doc = "RMT_CH7CONF1_REG"]
pub mod rmt_ch7conf1_reg;
#[doc = "RMT_CH0STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch0status_reg](rmt_ch0status_reg) module"]
pub type RMT_CH0STATUS_REG = crate::Reg<u32, _RMT_CH0STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH0STATUS_REG;
#[doc = "`read()` method returns [rmt_ch0status_reg::R](rmt_ch0status_reg::R) reader structure"]
impl crate::Readable for RMT_CH0STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch0status_reg::W](rmt_ch0status_reg::W) writer structure"]
impl crate::Writable for RMT_CH0STATUS_REG {}
#[doc = "RMT_CH0STATUS_REG"]
pub mod rmt_ch0status_reg;
#[doc = "RMT_CH1STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch1status_reg](rmt_ch1status_reg) module"]
pub type RMT_CH1STATUS_REG = crate::Reg<u32, _RMT_CH1STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH1STATUS_REG;
#[doc = "`read()` method returns [rmt_ch1status_reg::R](rmt_ch1status_reg::R) reader structure"]
impl crate::Readable for RMT_CH1STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch1status_reg::W](rmt_ch1status_reg::W) writer structure"]
impl crate::Writable for RMT_CH1STATUS_REG {}
#[doc = "RMT_CH1STATUS_REG"]
pub mod rmt_ch1status_reg;
#[doc = "RMT_CH2STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch2status_reg](rmt_ch2status_reg) module"]
pub type RMT_CH2STATUS_REG = crate::Reg<u32, _RMT_CH2STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH2STATUS_REG;
#[doc = "`read()` method returns [rmt_ch2status_reg::R](rmt_ch2status_reg::R) reader structure"]
impl crate::Readable for RMT_CH2STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch2status_reg::W](rmt_ch2status_reg::W) writer structure"]
impl crate::Writable for RMT_CH2STATUS_REG {}
#[doc = "RMT_CH2STATUS_REG"]
pub mod rmt_ch2status_reg;
#[doc = "RMT_CH3STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch3status_reg](rmt_ch3status_reg) module"]
pub type RMT_CH3STATUS_REG = crate::Reg<u32, _RMT_CH3STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH3STATUS_REG;
#[doc = "`read()` method returns [rmt_ch3status_reg::R](rmt_ch3status_reg::R) reader structure"]
impl crate::Readable for RMT_CH3STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch3status_reg::W](rmt_ch3status_reg::W) writer structure"]
impl crate::Writable for RMT_CH3STATUS_REG {}
#[doc = "RMT_CH3STATUS_REG"]
pub mod rmt_ch3status_reg;
#[doc = "RMT_CH4STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch4status_reg](rmt_ch4status_reg) module"]
pub type RMT_CH4STATUS_REG = crate::Reg<u32, _RMT_CH4STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH4STATUS_REG;
#[doc = "`read()` method returns [rmt_ch4status_reg::R](rmt_ch4status_reg::R) reader structure"]
impl crate::Readable for RMT_CH4STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch4status_reg::W](rmt_ch4status_reg::W) writer structure"]
impl crate::Writable for RMT_CH4STATUS_REG {}
#[doc = "RMT_CH4STATUS_REG"]
pub mod rmt_ch4status_reg;
#[doc = "RMT_CH5STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch5status_reg](rmt_ch5status_reg) module"]
pub type RMT_CH5STATUS_REG = crate::Reg<u32, _RMT_CH5STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH5STATUS_REG;
#[doc = "`read()` method returns [rmt_ch5status_reg::R](rmt_ch5status_reg::R) reader structure"]
impl crate::Readable for RMT_CH5STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch5status_reg::W](rmt_ch5status_reg::W) writer structure"]
impl crate::Writable for RMT_CH5STATUS_REG {}
#[doc = "RMT_CH5STATUS_REG"]
pub mod rmt_ch5status_reg;
#[doc = "RMT_CH6STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch6status_reg](rmt_ch6status_reg) module"]
pub type RMT_CH6STATUS_REG = crate::Reg<u32, _RMT_CH6STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH6STATUS_REG;
#[doc = "`read()` method returns [rmt_ch6status_reg::R](rmt_ch6status_reg::R) reader structure"]
impl crate::Readable for RMT_CH6STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch6status_reg::W](rmt_ch6status_reg::W) writer structure"]
impl crate::Writable for RMT_CH6STATUS_REG {}
#[doc = "RMT_CH6STATUS_REG"]
pub mod rmt_ch6status_reg;
#[doc = "RMT_CH7STATUS_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch7status_reg](rmt_ch7status_reg) module"]
pub type RMT_CH7STATUS_REG = crate::Reg<u32, _RMT_CH7STATUS_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH7STATUS_REG;
#[doc = "`read()` method returns [rmt_ch7status_reg::R](rmt_ch7status_reg::R) reader structure"]
impl crate::Readable for RMT_CH7STATUS_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch7status_reg::W](rmt_ch7status_reg::W) writer structure"]
impl crate::Writable for RMT_CH7STATUS_REG {}
#[doc = "RMT_CH7STATUS_REG"]
pub mod rmt_ch7status_reg;
#[doc = "RMT_CH0ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch0addr_reg](rmt_ch0addr_reg) module"]
pub type RMT_CH0ADDR_REG = crate::Reg<u32, _RMT_CH0ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH0ADDR_REG;
#[doc = "`read()` method returns [rmt_ch0addr_reg::R](rmt_ch0addr_reg::R) reader structure"]
impl crate::Readable for RMT_CH0ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch0addr_reg::W](rmt_ch0addr_reg::W) writer structure"]
impl crate::Writable for RMT_CH0ADDR_REG {}
#[doc = "RMT_CH0ADDR_REG"]
pub mod rmt_ch0addr_reg;
#[doc = "RMT_CH1ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch1addr_reg](rmt_ch1addr_reg) module"]
pub type RMT_CH1ADDR_REG = crate::Reg<u32, _RMT_CH1ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH1ADDR_REG;
#[doc = "`read()` method returns [rmt_ch1addr_reg::R](rmt_ch1addr_reg::R) reader structure"]
impl crate::Readable for RMT_CH1ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch1addr_reg::W](rmt_ch1addr_reg::W) writer structure"]
impl crate::Writable for RMT_CH1ADDR_REG {}
#[doc = "RMT_CH1ADDR_REG"]
pub mod rmt_ch1addr_reg;
#[doc = "RMT_CH2ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch2addr_reg](rmt_ch2addr_reg) module"]
pub type RMT_CH2ADDR_REG = crate::Reg<u32, _RMT_CH2ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH2ADDR_REG;
#[doc = "`read()` method returns [rmt_ch2addr_reg::R](rmt_ch2addr_reg::R) reader structure"]
impl crate::Readable for RMT_CH2ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch2addr_reg::W](rmt_ch2addr_reg::W) writer structure"]
impl crate::Writable for RMT_CH2ADDR_REG {}
#[doc = "RMT_CH2ADDR_REG"]
pub mod rmt_ch2addr_reg;
#[doc = "RMT_CH3ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch3addr_reg](rmt_ch3addr_reg) module"]
pub type RMT_CH3ADDR_REG = crate::Reg<u32, _RMT_CH3ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH3ADDR_REG;
#[doc = "`read()` method returns [rmt_ch3addr_reg::R](rmt_ch3addr_reg::R) reader structure"]
impl crate::Readable for RMT_CH3ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch3addr_reg::W](rmt_ch3addr_reg::W) writer structure"]
impl crate::Writable for RMT_CH3ADDR_REG {}
#[doc = "RMT_CH3ADDR_REG"]
pub mod rmt_ch3addr_reg;
#[doc = "RMT_CH4ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch4addr_reg](rmt_ch4addr_reg) module"]
pub type RMT_CH4ADDR_REG = crate::Reg<u32, _RMT_CH4ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH4ADDR_REG;
#[doc = "`read()` method returns [rmt_ch4addr_reg::R](rmt_ch4addr_reg::R) reader structure"]
impl crate::Readable for RMT_CH4ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch4addr_reg::W](rmt_ch4addr_reg::W) writer structure"]
impl crate::Writable for RMT_CH4ADDR_REG {}
#[doc = "RMT_CH4ADDR_REG"]
pub mod rmt_ch4addr_reg;
#[doc = "RMT_CH5ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch5addr_reg](rmt_ch5addr_reg) module"]
pub type RMT_CH5ADDR_REG = crate::Reg<u32, _RMT_CH5ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH5ADDR_REG;
#[doc = "`read()` method returns [rmt_ch5addr_reg::R](rmt_ch5addr_reg::R) reader structure"]
impl crate::Readable for RMT_CH5ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch5addr_reg::W](rmt_ch5addr_reg::W) writer structure"]
impl crate::Writable for RMT_CH5ADDR_REG {}
#[doc = "RMT_CH5ADDR_REG"]
pub mod rmt_ch5addr_reg;
#[doc = "RMT_CH6ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch6addr_reg](rmt_ch6addr_reg) module"]
pub type RMT_CH6ADDR_REG = crate::Reg<u32, _RMT_CH6ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH6ADDR_REG;
#[doc = "`read()` method returns [rmt_ch6addr_reg::R](rmt_ch6addr_reg::R) reader structure"]
impl crate::Readable for RMT_CH6ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch6addr_reg::W](rmt_ch6addr_reg::W) writer structure"]
impl crate::Writable for RMT_CH6ADDR_REG {}
#[doc = "RMT_CH6ADDR_REG"]
pub mod rmt_ch6addr_reg;
#[doc = "RMT_CH7ADDR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch7addr_reg](rmt_ch7addr_reg) module"]
pub type RMT_CH7ADDR_REG = crate::Reg<u32, _RMT_CH7ADDR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH7ADDR_REG;
#[doc = "`read()` method returns [rmt_ch7addr_reg::R](rmt_ch7addr_reg::R) reader structure"]
impl crate::Readable for RMT_CH7ADDR_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch7addr_reg::W](rmt_ch7addr_reg::W) writer structure"]
impl crate::Writable for RMT_CH7ADDR_REG {}
#[doc = "RMT_CH7ADDR_REG"]
pub mod rmt_ch7addr_reg;
#[doc = "RMT_INT_RAW_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_int_raw_reg](rmt_int_raw_reg) module"]
pub type RMT_INT_RAW_REG = crate::Reg<u32, _RMT_INT_RAW_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_INT_RAW_REG;
#[doc = "`read()` method returns [rmt_int_raw_reg::R](rmt_int_raw_reg::R) reader structure"]
impl crate::Readable for RMT_INT_RAW_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_int_raw_reg::W](rmt_int_raw_reg::W) writer structure"]
impl crate::Writable for RMT_INT_RAW_REG {}
#[doc = "RMT_INT_RAW_REG"]
pub mod rmt_int_raw_reg;
#[doc = "RMT_INT_ST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_int_st_reg](rmt_int_st_reg) module"]
pub type RMT_INT_ST_REG = crate::Reg<u32, _RMT_INT_ST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_INT_ST_REG;
#[doc = "`read()` method returns [rmt_int_st_reg::R](rmt_int_st_reg::R) reader structure"]
impl crate::Readable for RMT_INT_ST_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_int_st_reg::W](rmt_int_st_reg::W) writer structure"]
impl crate::Writable for RMT_INT_ST_REG {}
#[doc = "RMT_INT_ST_REG"]
pub mod rmt_int_st_reg;
#[doc = "RMT_INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_int_ena_reg](rmt_int_ena_reg) module"]
pub type RMT_INT_ENA_REG = crate::Reg<u32, _RMT_INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_INT_ENA_REG;
#[doc = "`read()` method returns [rmt_int_ena_reg::R](rmt_int_ena_reg::R) reader structure"]
impl crate::Readable for RMT_INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_int_ena_reg::W](rmt_int_ena_reg::W) writer structure"]
impl crate::Writable for RMT_INT_ENA_REG {}
#[doc = "RMT_INT_ENA_REG"]
pub mod rmt_int_ena_reg;
#[doc = "RMT_INT_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_int_clr_reg](rmt_int_clr_reg) module"]
pub type RMT_INT_CLR_REG = crate::Reg<u32, _RMT_INT_CLR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_INT_CLR_REG;
#[doc = "`read()` method returns [rmt_int_clr_reg::R](rmt_int_clr_reg::R) reader structure"]
impl crate::Readable for RMT_INT_CLR_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_int_clr_reg::W](rmt_int_clr_reg::W) writer structure"]
impl crate::Writable for RMT_INT_CLR_REG {}
#[doc = "RMT_INT_CLR_REG"]
pub mod rmt_int_clr_reg;
#[doc = "RMT_CH0CARRIER_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch0carrier_duty_reg](rmt_ch0carrier_duty_reg) module"]
pub type RMT_CH0CARRIER_DUTY_REG = crate::Reg<u32, _RMT_CH0CARRIER_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH0CARRIER_DUTY_REG;
#[doc = "`read()` method returns [rmt_ch0carrier_duty_reg::R](rmt_ch0carrier_duty_reg::R) reader structure"]
impl crate::Readable for RMT_CH0CARRIER_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch0carrier_duty_reg::W](rmt_ch0carrier_duty_reg::W) writer structure"]
impl crate::Writable for RMT_CH0CARRIER_DUTY_REG {}
#[doc = "RMT_CH0CARRIER_DUTY_REG"]
pub mod rmt_ch0carrier_duty_reg;
#[doc = "RMT_CH1CARRIER_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch1carrier_duty_reg](rmt_ch1carrier_duty_reg) module"]
pub type RMT_CH1CARRIER_DUTY_REG = crate::Reg<u32, _RMT_CH1CARRIER_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH1CARRIER_DUTY_REG;
#[doc = "`read()` method returns [rmt_ch1carrier_duty_reg::R](rmt_ch1carrier_duty_reg::R) reader structure"]
impl crate::Readable for RMT_CH1CARRIER_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch1carrier_duty_reg::W](rmt_ch1carrier_duty_reg::W) writer structure"]
impl crate::Writable for RMT_CH1CARRIER_DUTY_REG {}
#[doc = "RMT_CH1CARRIER_DUTY_REG"]
pub mod rmt_ch1carrier_duty_reg;
#[doc = "RMT_CH2CARRIER_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch2carrier_duty_reg](rmt_ch2carrier_duty_reg) module"]
pub type RMT_CH2CARRIER_DUTY_REG = crate::Reg<u32, _RMT_CH2CARRIER_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH2CARRIER_DUTY_REG;
#[doc = "`read()` method returns [rmt_ch2carrier_duty_reg::R](rmt_ch2carrier_duty_reg::R) reader structure"]
impl crate::Readable for RMT_CH2CARRIER_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch2carrier_duty_reg::W](rmt_ch2carrier_duty_reg::W) writer structure"]
impl crate::Writable for RMT_CH2CARRIER_DUTY_REG {}
#[doc = "RMT_CH2CARRIER_DUTY_REG"]
pub mod rmt_ch2carrier_duty_reg;
#[doc = "RMT_CH3CARRIER_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch3carrier_duty_reg](rmt_ch3carrier_duty_reg) module"]
pub type RMT_CH3CARRIER_DUTY_REG = crate::Reg<u32, _RMT_CH3CARRIER_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH3CARRIER_DUTY_REG;
#[doc = "`read()` method returns [rmt_ch3carrier_duty_reg::R](rmt_ch3carrier_duty_reg::R) reader structure"]
impl crate::Readable for RMT_CH3CARRIER_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch3carrier_duty_reg::W](rmt_ch3carrier_duty_reg::W) writer structure"]
impl crate::Writable for RMT_CH3CARRIER_DUTY_REG {}
#[doc = "RMT_CH3CARRIER_DUTY_REG"]
pub mod rmt_ch3carrier_duty_reg;
#[doc = "RMT_CH4CARRIER_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch4carrier_duty_reg](rmt_ch4carrier_duty_reg) module"]
pub type RMT_CH4CARRIER_DUTY_REG = crate::Reg<u32, _RMT_CH4CARRIER_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH4CARRIER_DUTY_REG;
#[doc = "`read()` method returns [rmt_ch4carrier_duty_reg::R](rmt_ch4carrier_duty_reg::R) reader structure"]
impl crate::Readable for RMT_CH4CARRIER_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch4carrier_duty_reg::W](rmt_ch4carrier_duty_reg::W) writer structure"]
impl crate::Writable for RMT_CH4CARRIER_DUTY_REG {}
#[doc = "RMT_CH4CARRIER_DUTY_REG"]
pub mod rmt_ch4carrier_duty_reg;
#[doc = "RMT_CH5CARRIER_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch5carrier_duty_reg](rmt_ch5carrier_duty_reg) module"]
pub type RMT_CH5CARRIER_DUTY_REG = crate::Reg<u32, _RMT_CH5CARRIER_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH5CARRIER_DUTY_REG;
#[doc = "`read()` method returns [rmt_ch5carrier_duty_reg::R](rmt_ch5carrier_duty_reg::R) reader structure"]
impl crate::Readable for RMT_CH5CARRIER_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch5carrier_duty_reg::W](rmt_ch5carrier_duty_reg::W) writer structure"]
impl crate::Writable for RMT_CH5CARRIER_DUTY_REG {}
#[doc = "RMT_CH5CARRIER_DUTY_REG"]
pub mod rmt_ch5carrier_duty_reg;
#[doc = "RMT_CH6CARRIER_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch6carrier_duty_reg](rmt_ch6carrier_duty_reg) module"]
pub type RMT_CH6CARRIER_DUTY_REG = crate::Reg<u32, _RMT_CH6CARRIER_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH6CARRIER_DUTY_REG;
#[doc = "`read()` method returns [rmt_ch6carrier_duty_reg::R](rmt_ch6carrier_duty_reg::R) reader structure"]
impl crate::Readable for RMT_CH6CARRIER_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch6carrier_duty_reg::W](rmt_ch6carrier_duty_reg::W) writer structure"]
impl crate::Writable for RMT_CH6CARRIER_DUTY_REG {}
#[doc = "RMT_CH6CARRIER_DUTY_REG"]
pub mod rmt_ch6carrier_duty_reg;
#[doc = "RMT_CH7CARRIER_DUTY_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch7carrier_duty_reg](rmt_ch7carrier_duty_reg) module"]
pub type RMT_CH7CARRIER_DUTY_REG = crate::Reg<u32, _RMT_CH7CARRIER_DUTY_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH7CARRIER_DUTY_REG;
#[doc = "`read()` method returns [rmt_ch7carrier_duty_reg::R](rmt_ch7carrier_duty_reg::R) reader structure"]
impl crate::Readable for RMT_CH7CARRIER_DUTY_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch7carrier_duty_reg::W](rmt_ch7carrier_duty_reg::W) writer structure"]
impl crate::Writable for RMT_CH7CARRIER_DUTY_REG {}
#[doc = "RMT_CH7CARRIER_DUTY_REG"]
pub mod rmt_ch7carrier_duty_reg;
#[doc = "RMT_CH0_TX_LIM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch0_tx_lim_reg](rmt_ch0_tx_lim_reg) module"]
pub type RMT_CH0_TX_LIM_REG = crate::Reg<u32, _RMT_CH0_TX_LIM_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH0_TX_LIM_REG;
#[doc = "`read()` method returns [rmt_ch0_tx_lim_reg::R](rmt_ch0_tx_lim_reg::R) reader structure"]
impl crate::Readable for RMT_CH0_TX_LIM_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch0_tx_lim_reg::W](rmt_ch0_tx_lim_reg::W) writer structure"]
impl crate::Writable for RMT_CH0_TX_LIM_REG {}
#[doc = "RMT_CH0_TX_LIM_REG"]
pub mod rmt_ch0_tx_lim_reg;
#[doc = "RMT_CH1_TX_LIM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch1_tx_lim_reg](rmt_ch1_tx_lim_reg) module"]
pub type RMT_CH1_TX_LIM_REG = crate::Reg<u32, _RMT_CH1_TX_LIM_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH1_TX_LIM_REG;
#[doc = "`read()` method returns [rmt_ch1_tx_lim_reg::R](rmt_ch1_tx_lim_reg::R) reader structure"]
impl crate::Readable for RMT_CH1_TX_LIM_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch1_tx_lim_reg::W](rmt_ch1_tx_lim_reg::W) writer structure"]
impl crate::Writable for RMT_CH1_TX_LIM_REG {}
#[doc = "RMT_CH1_TX_LIM_REG"]
pub mod rmt_ch1_tx_lim_reg;
#[doc = "RMT_CH2_TX_LIM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch2_tx_lim_reg](rmt_ch2_tx_lim_reg) module"]
pub type RMT_CH2_TX_LIM_REG = crate::Reg<u32, _RMT_CH2_TX_LIM_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH2_TX_LIM_REG;
#[doc = "`read()` method returns [rmt_ch2_tx_lim_reg::R](rmt_ch2_tx_lim_reg::R) reader structure"]
impl crate::Readable for RMT_CH2_TX_LIM_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch2_tx_lim_reg::W](rmt_ch2_tx_lim_reg::W) writer structure"]
impl crate::Writable for RMT_CH2_TX_LIM_REG {}
#[doc = "RMT_CH2_TX_LIM_REG"]
pub mod rmt_ch2_tx_lim_reg;
#[doc = "RMT_CH3_TX_LIM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch3_tx_lim_reg](rmt_ch3_tx_lim_reg) module"]
pub type RMT_CH3_TX_LIM_REG = crate::Reg<u32, _RMT_CH3_TX_LIM_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH3_TX_LIM_REG;
#[doc = "`read()` method returns [rmt_ch3_tx_lim_reg::R](rmt_ch3_tx_lim_reg::R) reader structure"]
impl crate::Readable for RMT_CH3_TX_LIM_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch3_tx_lim_reg::W](rmt_ch3_tx_lim_reg::W) writer structure"]
impl crate::Writable for RMT_CH3_TX_LIM_REG {}
#[doc = "RMT_CH3_TX_LIM_REG"]
pub mod rmt_ch3_tx_lim_reg;
#[doc = "RMT_CH4_TX_LIM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch4_tx_lim_reg](rmt_ch4_tx_lim_reg) module"]
pub type RMT_CH4_TX_LIM_REG = crate::Reg<u32, _RMT_CH4_TX_LIM_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH4_TX_LIM_REG;
#[doc = "`read()` method returns [rmt_ch4_tx_lim_reg::R](rmt_ch4_tx_lim_reg::R) reader structure"]
impl crate::Readable for RMT_CH4_TX_LIM_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch4_tx_lim_reg::W](rmt_ch4_tx_lim_reg::W) writer structure"]
impl crate::Writable for RMT_CH4_TX_LIM_REG {}
#[doc = "RMT_CH4_TX_LIM_REG"]
pub mod rmt_ch4_tx_lim_reg;
#[doc = "RMT_CH5_TX_LIM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch5_tx_lim_reg](rmt_ch5_tx_lim_reg) module"]
pub type RMT_CH5_TX_LIM_REG = crate::Reg<u32, _RMT_CH5_TX_LIM_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH5_TX_LIM_REG;
#[doc = "`read()` method returns [rmt_ch5_tx_lim_reg::R](rmt_ch5_tx_lim_reg::R) reader structure"]
impl crate::Readable for RMT_CH5_TX_LIM_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch5_tx_lim_reg::W](rmt_ch5_tx_lim_reg::W) writer structure"]
impl crate::Writable for RMT_CH5_TX_LIM_REG {}
#[doc = "RMT_CH5_TX_LIM_REG"]
pub mod rmt_ch5_tx_lim_reg;
#[doc = "RMT_CH6_TX_LIM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch6_tx_lim_reg](rmt_ch6_tx_lim_reg) module"]
pub type RMT_CH6_TX_LIM_REG = crate::Reg<u32, _RMT_CH6_TX_LIM_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH6_TX_LIM_REG;
#[doc = "`read()` method returns [rmt_ch6_tx_lim_reg::R](rmt_ch6_tx_lim_reg::R) reader structure"]
impl crate::Readable for RMT_CH6_TX_LIM_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch6_tx_lim_reg::W](rmt_ch6_tx_lim_reg::W) writer structure"]
impl crate::Writable for RMT_CH6_TX_LIM_REG {}
#[doc = "RMT_CH6_TX_LIM_REG"]
pub mod rmt_ch6_tx_lim_reg;
#[doc = "RMT_CH7_TX_LIM_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_ch7_tx_lim_reg](rmt_ch7_tx_lim_reg) module"]
pub type RMT_CH7_TX_LIM_REG = crate::Reg<u32, _RMT_CH7_TX_LIM_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_CH7_TX_LIM_REG;
#[doc = "`read()` method returns [rmt_ch7_tx_lim_reg::R](rmt_ch7_tx_lim_reg::R) reader structure"]
impl crate::Readable for RMT_CH7_TX_LIM_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_ch7_tx_lim_reg::W](rmt_ch7_tx_lim_reg::W) writer structure"]
impl crate::Writable for RMT_CH7_TX_LIM_REG {}
#[doc = "RMT_CH7_TX_LIM_REG"]
pub mod rmt_ch7_tx_lim_reg;
#[doc = "RMT_APB_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_apb_conf_reg](rmt_apb_conf_reg) module"]
pub type RMT_APB_CONF_REG = crate::Reg<u32, _RMT_APB_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_APB_CONF_REG;
#[doc = "`read()` method returns [rmt_apb_conf_reg::R](rmt_apb_conf_reg::R) reader structure"]
impl crate::Readable for RMT_APB_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_apb_conf_reg::W](rmt_apb_conf_reg::W) writer structure"]
impl crate::Writable for RMT_APB_CONF_REG {}
#[doc = "RMT_APB_CONF_REG"]
pub mod rmt_apb_conf_reg;
#[doc = "RMT_DATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rmt_date_reg](rmt_date_reg) module"]
pub type RMT_DATE_REG = crate::Reg<u32, _RMT_DATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RMT_DATE_REG;
#[doc = "`read()` method returns [rmt_date_reg::R](rmt_date_reg::R) reader structure"]
impl crate::Readable for RMT_DATE_REG {}
#[doc = "`write(|w| ..)` method takes [rmt_date_reg::W](rmt_date_reg::W) writer structure"]
impl crate::Writable for RMT_DATE_REG {}
#[doc = "RMT_DATE_REG"]
pub mod rmt_date_reg;
