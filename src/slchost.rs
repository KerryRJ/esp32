#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - HOST_SLCHOST_FUNC2_0_REG"]
    pub host_slchost_func2_0_reg: HOST_SLCHOST_FUNC2_0_REG,
    #[doc = "0x14 - HOST_SLCHOST_FUNC2_1_REG"]
    pub host_slchost_func2_1_reg: HOST_SLCHOST_FUNC2_1_REG,
    _reserved2: [u8; 8usize],
    #[doc = "0x20 - HOST_SLCHOST_FUNC2_2_REG"]
    pub host_slchost_func2_2_reg: HOST_SLCHOST_FUNC2_2_REG,
    _reserved3: [u8; 16usize],
    #[doc = "0x34 - HOST_SLCHOST_GPIO_STATUS0_REG"]
    pub host_slchost_gpio_status0_reg: HOST_SLCHOST_GPIO_STATUS0_REG,
    #[doc = "0x38 - HOST_SLCHOST_GPIO_STATUS1_REG"]
    pub host_slchost_gpio_status1_reg: HOST_SLCHOST_GPIO_STATUS1_REG,
    #[doc = "0x3c - HOST_SLCHOST_GPIO_IN0_REG"]
    pub host_slchost_gpio_in0_reg: HOST_SLCHOST_GPIO_IN0_REG,
    #[doc = "0x40 - HOST_SLCHOST_GPIO_IN1_REG"]
    pub host_slchost_gpio_in1_reg: HOST_SLCHOST_GPIO_IN1_REG,
    #[doc = "0x44 - HOST_SLC0HOST_TOKEN_RDATA_REG"]
    pub host_slc0host_token_rdata_reg: HOST_SLC0HOST_TOKEN_RDATA_REG,
    #[doc = "0x48 - HOST_SLC0_HOST_PF_REG"]
    pub host_slc0_host_pf_reg: HOST_SLC0_HOST_PF_REG,
    #[doc = "0x4c - HOST_SLC1_HOST_PF_REG"]
    pub host_slc1_host_pf_reg: HOST_SLC1_HOST_PF_REG,
    #[doc = "0x50 - HOST_SLC0HOST_INT_RAW_REG"]
    pub host_slc0host_int_raw_reg: HOST_SLC0HOST_INT_RAW_REG,
    #[doc = "0x54 - HOST_SLC1HOST_INT_RAW_REG"]
    pub host_slc1host_int_raw_reg: HOST_SLC1HOST_INT_RAW_REG,
    #[doc = "0x58 - HOST_SLC0HOST_INT_ST_REG"]
    pub host_slc0host_int_st_reg: HOST_SLC0HOST_INT_ST_REG,
    #[doc = "0x5c - HOST_SLC1HOST_INT_ST_REG"]
    pub host_slc1host_int_st_reg: HOST_SLC1HOST_INT_ST_REG,
    #[doc = "0x60 - HOST_SLCHOST_PKT_LEN_REG"]
    pub host_slchost_pkt_len_reg: HOST_SLCHOST_PKT_LEN_REG,
    #[doc = "0x64 - HOST_SLCHOST_STATE_W0_REG"]
    pub host_slchost_state_w0_reg: HOST_SLCHOST_STATE_W0_REG,
    #[doc = "0x68 - HOST_SLCHOST_STATE_W1_REG"]
    pub host_slchost_state_w1_reg: HOST_SLCHOST_STATE_W1_REG,
    #[doc = "0x6c - HOST_SLCHOST_CONF_W0_REG"]
    pub host_slchost_conf_w0_reg: HOST_SLCHOST_CONF_W0_REG,
    #[doc = "0x70 - HOST_SLCHOST_CONF_W1_REG"]
    pub host_slchost_conf_w1_reg: HOST_SLCHOST_CONF_W1_REG,
    #[doc = "0x74 - HOST_SLCHOST_CONF_W2_REG"]
    pub host_slchost_conf_w2_reg: HOST_SLCHOST_CONF_W2_REG,
    #[doc = "0x78 - HOST_SLCHOST_CONF_W3_REG"]
    pub host_slchost_conf_w3_reg: HOST_SLCHOST_CONF_W3_REG,
    #[doc = "0x7c - HOST_SLCHOST_CONF_W4_REG"]
    pub host_slchost_conf_w4_reg: HOST_SLCHOST_CONF_W4_REG,
    #[doc = "0x80 - HOST_SLCHOST_CONF_W5_REG"]
    pub host_slchost_conf_w5_reg: HOST_SLCHOST_CONF_W5_REG,
    _reserved23: [u8; 4usize],
    #[doc = "0x88 - HOST_SLCHOST_CONF_W6_REG"]
    pub host_slchost_conf_w6_reg: HOST_SLCHOST_CONF_W6_REG,
    #[doc = "0x8c - HOST_SLCHOST_CONF_W7_REG"]
    pub host_slchost_conf_w7_reg: HOST_SLCHOST_CONF_W7_REG,
    #[doc = "0x90 - HOST_SLCHOST_PKT_LEN0_REG"]
    pub host_slchost_pkt_len0_reg: HOST_SLCHOST_PKT_LEN0_REG,
    #[doc = "0x94 - HOST_SLCHOST_PKT_LEN1_REG"]
    pub host_slchost_pkt_len1_reg: HOST_SLCHOST_PKT_LEN1_REG,
    #[doc = "0x98 - HOST_SLCHOST_PKT_LEN2_REG"]
    pub host_slchost_pkt_len2_reg: HOST_SLCHOST_PKT_LEN2_REG,
    #[doc = "0x9c - HOST_SLCHOST_CONF_W8_REG"]
    pub host_slchost_conf_w8_reg: HOST_SLCHOST_CONF_W8_REG,
    #[doc = "0xa0 - HOST_SLCHOST_CONF_W9_REG"]
    pub host_slchost_conf_w9_reg: HOST_SLCHOST_CONF_W9_REG,
    #[doc = "0xa4 - HOST_SLCHOST_CONF_W10_REG"]
    pub host_slchost_conf_w10_reg: HOST_SLCHOST_CONF_W10_REG,
    #[doc = "0xa8 - HOST_SLCHOST_CONF_W11_REG"]
    pub host_slchost_conf_w11_reg: HOST_SLCHOST_CONF_W11_REG,
    #[doc = "0xac - HOST_SLCHOST_CONF_W12_REG"]
    pub host_slchost_conf_w12_reg: HOST_SLCHOST_CONF_W12_REG,
    #[doc = "0xb0 - HOST_SLCHOST_CONF_W13_REG"]
    pub host_slchost_conf_w13_reg: HOST_SLCHOST_CONF_W13_REG,
    #[doc = "0xb4 - HOST_SLCHOST_CONF_W14_REG"]
    pub host_slchost_conf_w14_reg: HOST_SLCHOST_CONF_W14_REG,
    #[doc = "0xb8 - HOST_SLCHOST_CONF_W15_REG"]
    pub host_slchost_conf_w15_reg: HOST_SLCHOST_CONF_W15_REG,
    #[doc = "0xbc - HOST_SLCHOST_CHECK_SUM0_REG"]
    pub host_slchost_check_sum0_reg: HOST_SLCHOST_CHECK_SUM0_REG,
    #[doc = "0xc0 - HOST_SLCHOST_CHECK_SUM1_REG"]
    pub host_slchost_check_sum1_reg: HOST_SLCHOST_CHECK_SUM1_REG,
    #[doc = "0xc4 - HOST_SLC1HOST_TOKEN_RDATA_REG"]
    pub host_slc1host_token_rdata_reg: HOST_SLC1HOST_TOKEN_RDATA_REG,
    #[doc = "0xc8 - HOST_SLC0HOST_TOKEN_WDATA_REG"]
    pub host_slc0host_token_wdata_reg: HOST_SLC0HOST_TOKEN_WDATA_REG,
    #[doc = "0xcc - HOST_SLC1HOST_TOKEN_WDATA_REG"]
    pub host_slc1host_token_wdata_reg: HOST_SLC1HOST_TOKEN_WDATA_REG,
    #[doc = "0xd0 - HOST_SLCHOST_TOKEN_CON_REG"]
    pub host_slchost_token_con_reg: HOST_SLCHOST_TOKEN_CON_REG,
    #[doc = "0xd4 - HOST_SLC0HOST_INT_CLR_REG"]
    pub host_slc0host_int_clr_reg: HOST_SLC0HOST_INT_CLR_REG,
    #[doc = "0xd8 - HOST_SLC1HOST_INT_CLR_REG"]
    pub host_slc1host_int_clr_reg: HOST_SLC1HOST_INT_CLR_REG,
    #[doc = "0xdc - HOST_SLC0HOST_FUNC1_INT_ENA_REG"]
    pub host_slc0host_func1_int_ena_reg: HOST_SLC0HOST_FUNC1_INT_ENA_REG,
    #[doc = "0xe0 - HOST_SLC1HOST_FUNC1_INT_ENA_REG"]
    pub host_slc1host_func1_int_ena_reg: HOST_SLC1HOST_FUNC1_INT_ENA_REG,
    #[doc = "0xe4 - HOST_SLC0HOST_FUNC2_INT_ENA_REG"]
    pub host_slc0host_func2_int_ena_reg: HOST_SLC0HOST_FUNC2_INT_ENA_REG,
    #[doc = "0xe8 - HOST_SLC1HOST_FUNC2_INT_ENA_REG"]
    pub host_slc1host_func2_int_ena_reg: HOST_SLC1HOST_FUNC2_INT_ENA_REG,
    #[doc = "0xec - HOST_SLC0HOST_INT_ENA_REG"]
    pub host_slc0host_int_ena_reg: HOST_SLC0HOST_INT_ENA_REG,
    #[doc = "0xf0 - HOST_SLC1HOST_INT_ENA_REG"]
    pub host_slc1host_int_ena_reg: HOST_SLC1HOST_INT_ENA_REG,
    #[doc = "0xf4 - HOST_SLC0HOST_RX_INFOR_REG"]
    pub host_slc0host_rx_infor_reg: HOST_SLC0HOST_RX_INFOR_REG,
    #[doc = "0xf8 - HOST_SLC1HOST_RX_INFOR_REG"]
    pub host_slc1host_rx_infor_reg: HOST_SLC1HOST_RX_INFOR_REG,
    #[doc = "0xfc - HOST_SLC0HOST_LEN_WD_REG"]
    pub host_slc0host_len_wd_reg: HOST_SLC0HOST_LEN_WD_REG,
    #[doc = "0x100 - HOST_SLC_APBWIN_WDATA_REG"]
    pub host_slc_apbwin_wdata_reg: HOST_SLC_APBWIN_WDATA_REG,
    #[doc = "0x104 - HOST_SLC_APBWIN_CONF_REG"]
    pub host_slc_apbwin_conf_reg: HOST_SLC_APBWIN_CONF_REG,
    #[doc = "0x108 - HOST_SLC_APBWIN_RDATA_REG"]
    pub host_slc_apbwin_rdata_reg: HOST_SLC_APBWIN_RDATA_REG,
    #[doc = "0x10c - HOST_SLCHOST_RDCLR0_REG"]
    pub host_slchost_rdclr0_reg: HOST_SLCHOST_RDCLR0_REG,
    #[doc = "0x110 - HOST_SLCHOST_RDCLR1_REG"]
    pub host_slchost_rdclr1_reg: HOST_SLCHOST_RDCLR1_REG,
    #[doc = "0x114 - HOST_SLC0HOST_INT_ENA1_REG"]
    pub host_slc0host_int_ena1_reg: HOST_SLC0HOST_INT_ENA1_REG,
    #[doc = "0x118 - HOST_SLC1HOST_INT_ENA1_REG"]
    pub host_slc1host_int_ena1_reg: HOST_SLC1HOST_INT_ENA1_REG,
    _reserved60: [u8; 92usize],
    #[doc = "0x178 - HOST_SLCHOSTDATE_REG"]
    pub host_slchostdate_reg: HOST_SLCHOSTDATE_REG,
    #[doc = "0x17c - HOST_SLCHOSTID_REG"]
    pub host_slchostid_reg: HOST_SLCHOSTID_REG,
    _reserved62: [u8; 112usize],
    #[doc = "0x1f0 - HOST_SLCHOST_CONF_REG"]
    pub host_slchost_conf_reg: HOST_SLCHOST_CONF_REG,
    #[doc = "0x1f4 - HOST_SLCHOST_INF_ST_REG"]
    pub host_slchost_inf_st_reg: HOST_SLCHOST_INF_ST_REG,
}
#[doc = "HOST_SLCHOST_FUNC2_0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_func2_0_reg](host_slchost_func2_0_reg) module"]
pub type HOST_SLCHOST_FUNC2_0_REG = crate::Reg<u32, _HOST_SLCHOST_FUNC2_0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_FUNC2_0_REG;
#[doc = "`read()` method returns [host_slchost_func2_0_reg::R](host_slchost_func2_0_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_FUNC2_0_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_func2_0_reg::W](host_slchost_func2_0_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_FUNC2_0_REG {}
#[doc = "HOST_SLCHOST_FUNC2_0_REG"]
pub mod host_slchost_func2_0_reg;
#[doc = "HOST_SLCHOST_FUNC2_1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_func2_1_reg](host_slchost_func2_1_reg) module"]
pub type HOST_SLCHOST_FUNC2_1_REG = crate::Reg<u32, _HOST_SLCHOST_FUNC2_1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_FUNC2_1_REG;
#[doc = "`read()` method returns [host_slchost_func2_1_reg::R](host_slchost_func2_1_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_FUNC2_1_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_func2_1_reg::W](host_slchost_func2_1_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_FUNC2_1_REG {}
#[doc = "HOST_SLCHOST_FUNC2_1_REG"]
pub mod host_slchost_func2_1_reg;
#[doc = "HOST_SLCHOST_FUNC2_2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_func2_2_reg](host_slchost_func2_2_reg) module"]
pub type HOST_SLCHOST_FUNC2_2_REG = crate::Reg<u32, _HOST_SLCHOST_FUNC2_2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_FUNC2_2_REG;
#[doc = "`read()` method returns [host_slchost_func2_2_reg::R](host_slchost_func2_2_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_FUNC2_2_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_func2_2_reg::W](host_slchost_func2_2_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_FUNC2_2_REG {}
#[doc = "HOST_SLCHOST_FUNC2_2_REG"]
pub mod host_slchost_func2_2_reg;
#[doc = "HOST_SLCHOST_GPIO_STATUS0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_gpio_status0_reg](host_slchost_gpio_status0_reg) module"]
pub type HOST_SLCHOST_GPIO_STATUS0_REG = crate::Reg<u32, _HOST_SLCHOST_GPIO_STATUS0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_GPIO_STATUS0_REG;
#[doc = "`read()` method returns [host_slchost_gpio_status0_reg::R](host_slchost_gpio_status0_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_GPIO_STATUS0_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_gpio_status0_reg::W](host_slchost_gpio_status0_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_GPIO_STATUS0_REG {}
#[doc = "HOST_SLCHOST_GPIO_STATUS0_REG"]
pub mod host_slchost_gpio_status0_reg;
#[doc = "HOST_SLCHOST_GPIO_STATUS1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_gpio_status1_reg](host_slchost_gpio_status1_reg) module"]
pub type HOST_SLCHOST_GPIO_STATUS1_REG = crate::Reg<u32, _HOST_SLCHOST_GPIO_STATUS1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_GPIO_STATUS1_REG;
#[doc = "`read()` method returns [host_slchost_gpio_status1_reg::R](host_slchost_gpio_status1_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_GPIO_STATUS1_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_gpio_status1_reg::W](host_slchost_gpio_status1_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_GPIO_STATUS1_REG {}
#[doc = "HOST_SLCHOST_GPIO_STATUS1_REG"]
pub mod host_slchost_gpio_status1_reg;
#[doc = "HOST_SLCHOST_GPIO_IN0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_gpio_in0_reg](host_slchost_gpio_in0_reg) module"]
pub type HOST_SLCHOST_GPIO_IN0_REG = crate::Reg<u32, _HOST_SLCHOST_GPIO_IN0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_GPIO_IN0_REG;
#[doc = "`read()` method returns [host_slchost_gpio_in0_reg::R](host_slchost_gpio_in0_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_GPIO_IN0_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_gpio_in0_reg::W](host_slchost_gpio_in0_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_GPIO_IN0_REG {}
#[doc = "HOST_SLCHOST_GPIO_IN0_REG"]
pub mod host_slchost_gpio_in0_reg;
#[doc = "HOST_SLCHOST_GPIO_IN1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_gpio_in1_reg](host_slchost_gpio_in1_reg) module"]
pub type HOST_SLCHOST_GPIO_IN1_REG = crate::Reg<u32, _HOST_SLCHOST_GPIO_IN1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_GPIO_IN1_REG;
#[doc = "`read()` method returns [host_slchost_gpio_in1_reg::R](host_slchost_gpio_in1_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_GPIO_IN1_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_gpio_in1_reg::W](host_slchost_gpio_in1_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_GPIO_IN1_REG {}
#[doc = "HOST_SLCHOST_GPIO_IN1_REG"]
pub mod host_slchost_gpio_in1_reg;
#[doc = "HOST_SLC0HOST_TOKEN_RDATA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_token_rdata_reg](host_slc0host_token_rdata_reg) module"]
pub type HOST_SLC0HOST_TOKEN_RDATA_REG = crate::Reg<u32, _HOST_SLC0HOST_TOKEN_RDATA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_TOKEN_RDATA_REG;
#[doc = "`read()` method returns [host_slc0host_token_rdata_reg::R](host_slc0host_token_rdata_reg::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_TOKEN_RDATA_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_token_rdata_reg::W](host_slc0host_token_rdata_reg::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_TOKEN_RDATA_REG {}
#[doc = "HOST_SLC0HOST_TOKEN_RDATA_REG"]
pub mod host_slc0host_token_rdata_reg;
#[doc = "HOST_SLC0_HOST_PF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0_host_pf_reg](host_slc0_host_pf_reg) module"]
pub type HOST_SLC0_HOST_PF_REG = crate::Reg<u32, _HOST_SLC0_HOST_PF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0_HOST_PF_REG;
#[doc = "`read()` method returns [host_slc0_host_pf_reg::R](host_slc0_host_pf_reg::R) reader structure"]
impl crate::Readable for HOST_SLC0_HOST_PF_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc0_host_pf_reg::W](host_slc0_host_pf_reg::W) writer structure"]
impl crate::Writable for HOST_SLC0_HOST_PF_REG {}
#[doc = "HOST_SLC0_HOST_PF_REG"]
pub mod host_slc0_host_pf_reg;
#[doc = "HOST_SLC1_HOST_PF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1_host_pf_reg](host_slc1_host_pf_reg) module"]
pub type HOST_SLC1_HOST_PF_REG = crate::Reg<u32, _HOST_SLC1_HOST_PF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1_HOST_PF_REG;
#[doc = "`read()` method returns [host_slc1_host_pf_reg::R](host_slc1_host_pf_reg::R) reader structure"]
impl crate::Readable for HOST_SLC1_HOST_PF_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc1_host_pf_reg::W](host_slc1_host_pf_reg::W) writer structure"]
impl crate::Writable for HOST_SLC1_HOST_PF_REG {}
#[doc = "HOST_SLC1_HOST_PF_REG"]
pub mod host_slc1_host_pf_reg;
#[doc = "HOST_SLC0HOST_INT_RAW_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_int_raw_reg](host_slc0host_int_raw_reg) module"]
pub type HOST_SLC0HOST_INT_RAW_REG = crate::Reg<u32, _HOST_SLC0HOST_INT_RAW_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_INT_RAW_REG;
#[doc = "`read()` method returns [host_slc0host_int_raw_reg::R](host_slc0host_int_raw_reg::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_INT_RAW_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_int_raw_reg::W](host_slc0host_int_raw_reg::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_INT_RAW_REG {}
#[doc = "HOST_SLC0HOST_INT_RAW_REG"]
pub mod host_slc0host_int_raw_reg;
#[doc = "HOST_SLC1HOST_INT_RAW_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_int_raw_reg](host_slc1host_int_raw_reg) module"]
pub type HOST_SLC1HOST_INT_RAW_REG = crate::Reg<u32, _HOST_SLC1HOST_INT_RAW_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_INT_RAW_REG;
#[doc = "`read()` method returns [host_slc1host_int_raw_reg::R](host_slc1host_int_raw_reg::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_INT_RAW_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_int_raw_reg::W](host_slc1host_int_raw_reg::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_INT_RAW_REG {}
#[doc = "HOST_SLC1HOST_INT_RAW_REG"]
pub mod host_slc1host_int_raw_reg;
#[doc = "HOST_SLC0HOST_INT_ST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_int_st_reg](host_slc0host_int_st_reg) module"]
pub type HOST_SLC0HOST_INT_ST_REG = crate::Reg<u32, _HOST_SLC0HOST_INT_ST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_INT_ST_REG;
#[doc = "`read()` method returns [host_slc0host_int_st_reg::R](host_slc0host_int_st_reg::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_INT_ST_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_int_st_reg::W](host_slc0host_int_st_reg::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_INT_ST_REG {}
#[doc = "HOST_SLC0HOST_INT_ST_REG"]
pub mod host_slc0host_int_st_reg;
#[doc = "HOST_SLC1HOST_INT_ST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_int_st_reg](host_slc1host_int_st_reg) module"]
pub type HOST_SLC1HOST_INT_ST_REG = crate::Reg<u32, _HOST_SLC1HOST_INT_ST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_INT_ST_REG;
#[doc = "`read()` method returns [host_slc1host_int_st_reg::R](host_slc1host_int_st_reg::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_INT_ST_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_int_st_reg::W](host_slc1host_int_st_reg::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_INT_ST_REG {}
#[doc = "HOST_SLC1HOST_INT_ST_REG"]
pub mod host_slc1host_int_st_reg;
#[doc = "HOST_SLCHOST_PKT_LEN_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_pkt_len_reg](host_slchost_pkt_len_reg) module"]
pub type HOST_SLCHOST_PKT_LEN_REG = crate::Reg<u32, _HOST_SLCHOST_PKT_LEN_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_PKT_LEN_REG;
#[doc = "`read()` method returns [host_slchost_pkt_len_reg::R](host_slchost_pkt_len_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_PKT_LEN_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_pkt_len_reg::W](host_slchost_pkt_len_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_PKT_LEN_REG {}
#[doc = "HOST_SLCHOST_PKT_LEN_REG"]
pub mod host_slchost_pkt_len_reg;
#[doc = "HOST_SLCHOST_STATE_W0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_state_w0_reg](host_slchost_state_w0_reg) module"]
pub type HOST_SLCHOST_STATE_W0_REG = crate::Reg<u32, _HOST_SLCHOST_STATE_W0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_STATE_W0_REG;
#[doc = "`read()` method returns [host_slchost_state_w0_reg::R](host_slchost_state_w0_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_STATE_W0_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_state_w0_reg::W](host_slchost_state_w0_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_STATE_W0_REG {}
#[doc = "HOST_SLCHOST_STATE_W0_REG"]
pub mod host_slchost_state_w0_reg;
#[doc = "HOST_SLCHOST_STATE_W1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_state_w1_reg](host_slchost_state_w1_reg) module"]
pub type HOST_SLCHOST_STATE_W1_REG = crate::Reg<u32, _HOST_SLCHOST_STATE_W1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_STATE_W1_REG;
#[doc = "`read()` method returns [host_slchost_state_w1_reg::R](host_slchost_state_w1_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_STATE_W1_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_state_w1_reg::W](host_slchost_state_w1_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_STATE_W1_REG {}
#[doc = "HOST_SLCHOST_STATE_W1_REG"]
pub mod host_slchost_state_w1_reg;
#[doc = "HOST_SLCHOST_CONF_W0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w0_reg](host_slchost_conf_w0_reg) module"]
pub type HOST_SLCHOST_CONF_W0_REG = crate::Reg<u32, _HOST_SLCHOST_CONF_W0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W0_REG;
#[doc = "`read()` method returns [host_slchost_conf_w0_reg::R](host_slchost_conf_w0_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W0_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w0_reg::W](host_slchost_conf_w0_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W0_REG {}
#[doc = "HOST_SLCHOST_CONF_W0_REG"]
pub mod host_slchost_conf_w0_reg;
#[doc = "HOST_SLCHOST_CONF_W1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w1_reg](host_slchost_conf_w1_reg) module"]
pub type HOST_SLCHOST_CONF_W1_REG = crate::Reg<u32, _HOST_SLCHOST_CONF_W1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W1_REG;
#[doc = "`read()` method returns [host_slchost_conf_w1_reg::R](host_slchost_conf_w1_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W1_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w1_reg::W](host_slchost_conf_w1_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W1_REG {}
#[doc = "HOST_SLCHOST_CONF_W1_REG"]
pub mod host_slchost_conf_w1_reg;
#[doc = "HOST_SLCHOST_CONF_W2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w2_reg](host_slchost_conf_w2_reg) module"]
pub type HOST_SLCHOST_CONF_W2_REG = crate::Reg<u32, _HOST_SLCHOST_CONF_W2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W2_REG;
#[doc = "`read()` method returns [host_slchost_conf_w2_reg::R](host_slchost_conf_w2_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W2_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w2_reg::W](host_slchost_conf_w2_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W2_REG {}
#[doc = "HOST_SLCHOST_CONF_W2_REG"]
pub mod host_slchost_conf_w2_reg;
#[doc = "HOST_SLCHOST_CONF_W3_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w3_reg](host_slchost_conf_w3_reg) module"]
pub type HOST_SLCHOST_CONF_W3_REG = crate::Reg<u32, _HOST_SLCHOST_CONF_W3_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W3_REG;
#[doc = "`read()` method returns [host_slchost_conf_w3_reg::R](host_slchost_conf_w3_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W3_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w3_reg::W](host_slchost_conf_w3_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W3_REG {}
#[doc = "HOST_SLCHOST_CONF_W3_REG"]
pub mod host_slchost_conf_w3_reg;
#[doc = "HOST_SLCHOST_CONF_W4_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w4_reg](host_slchost_conf_w4_reg) module"]
pub type HOST_SLCHOST_CONF_W4_REG = crate::Reg<u32, _HOST_SLCHOST_CONF_W4_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W4_REG;
#[doc = "`read()` method returns [host_slchost_conf_w4_reg::R](host_slchost_conf_w4_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W4_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w4_reg::W](host_slchost_conf_w4_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W4_REG {}
#[doc = "HOST_SLCHOST_CONF_W4_REG"]
pub mod host_slchost_conf_w4_reg;
#[doc = "HOST_SLCHOST_CONF_W5_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w5_reg](host_slchost_conf_w5_reg) module"]
pub type HOST_SLCHOST_CONF_W5_REG = crate::Reg<u32, _HOST_SLCHOST_CONF_W5_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W5_REG;
#[doc = "`read()` method returns [host_slchost_conf_w5_reg::R](host_slchost_conf_w5_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W5_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w5_reg::W](host_slchost_conf_w5_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W5_REG {}
#[doc = "HOST_SLCHOST_CONF_W5_REG"]
pub mod host_slchost_conf_w5_reg;
#[doc = "HOST_SLCHOST_CONF_W6_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w6_reg](host_slchost_conf_w6_reg) module"]
pub type HOST_SLCHOST_CONF_W6_REG = crate::Reg<u32, _HOST_SLCHOST_CONF_W6_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W6_REG;
#[doc = "`read()` method returns [host_slchost_conf_w6_reg::R](host_slchost_conf_w6_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W6_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w6_reg::W](host_slchost_conf_w6_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W6_REG {}
#[doc = "HOST_SLCHOST_CONF_W6_REG"]
pub mod host_slchost_conf_w6_reg;
#[doc = "HOST_SLCHOST_CONF_W7_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w7_reg](host_slchost_conf_w7_reg) module"]
pub type HOST_SLCHOST_CONF_W7_REG = crate::Reg<u32, _HOST_SLCHOST_CONF_W7_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W7_REG;
#[doc = "`read()` method returns [host_slchost_conf_w7_reg::R](host_slchost_conf_w7_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W7_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w7_reg::W](host_slchost_conf_w7_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W7_REG {}
#[doc = "HOST_SLCHOST_CONF_W7_REG"]
pub mod host_slchost_conf_w7_reg;
#[doc = "HOST_SLCHOST_PKT_LEN0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_pkt_len0_reg](host_slchost_pkt_len0_reg) module"]
pub type HOST_SLCHOST_PKT_LEN0_REG = crate::Reg<u32, _HOST_SLCHOST_PKT_LEN0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_PKT_LEN0_REG;
#[doc = "`read()` method returns [host_slchost_pkt_len0_reg::R](host_slchost_pkt_len0_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_PKT_LEN0_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_pkt_len0_reg::W](host_slchost_pkt_len0_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_PKT_LEN0_REG {}
#[doc = "HOST_SLCHOST_PKT_LEN0_REG"]
pub mod host_slchost_pkt_len0_reg;
#[doc = "HOST_SLCHOST_PKT_LEN1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_pkt_len1_reg](host_slchost_pkt_len1_reg) module"]
pub type HOST_SLCHOST_PKT_LEN1_REG = crate::Reg<u32, _HOST_SLCHOST_PKT_LEN1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_PKT_LEN1_REG;
#[doc = "`read()` method returns [host_slchost_pkt_len1_reg::R](host_slchost_pkt_len1_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_PKT_LEN1_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_pkt_len1_reg::W](host_slchost_pkt_len1_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_PKT_LEN1_REG {}
#[doc = "HOST_SLCHOST_PKT_LEN1_REG"]
pub mod host_slchost_pkt_len1_reg;
#[doc = "HOST_SLCHOST_PKT_LEN2_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_pkt_len2_reg](host_slchost_pkt_len2_reg) module"]
pub type HOST_SLCHOST_PKT_LEN2_REG = crate::Reg<u32, _HOST_SLCHOST_PKT_LEN2_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_PKT_LEN2_REG;
#[doc = "`read()` method returns [host_slchost_pkt_len2_reg::R](host_slchost_pkt_len2_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_PKT_LEN2_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_pkt_len2_reg::W](host_slchost_pkt_len2_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_PKT_LEN2_REG {}
#[doc = "HOST_SLCHOST_PKT_LEN2_REG"]
pub mod host_slchost_pkt_len2_reg;
#[doc = "HOST_SLCHOST_CONF_W8_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w8_reg](host_slchost_conf_w8_reg) module"]
pub type HOST_SLCHOST_CONF_W8_REG = crate::Reg<u32, _HOST_SLCHOST_CONF_W8_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W8_REG;
#[doc = "`read()` method returns [host_slchost_conf_w8_reg::R](host_slchost_conf_w8_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W8_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w8_reg::W](host_slchost_conf_w8_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W8_REG {}
#[doc = "HOST_SLCHOST_CONF_W8_REG"]
pub mod host_slchost_conf_w8_reg;
#[doc = "HOST_SLCHOST_CONF_W9_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w9_reg](host_slchost_conf_w9_reg) module"]
pub type HOST_SLCHOST_CONF_W9_REG = crate::Reg<u32, _HOST_SLCHOST_CONF_W9_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W9_REG;
#[doc = "`read()` method returns [host_slchost_conf_w9_reg::R](host_slchost_conf_w9_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W9_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w9_reg::W](host_slchost_conf_w9_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W9_REG {}
#[doc = "HOST_SLCHOST_CONF_W9_REG"]
pub mod host_slchost_conf_w9_reg;
#[doc = "HOST_SLCHOST_CONF_W10_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w10_reg](host_slchost_conf_w10_reg) module"]
pub type HOST_SLCHOST_CONF_W10_REG = crate::Reg<u32, _HOST_SLCHOST_CONF_W10_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W10_REG;
#[doc = "`read()` method returns [host_slchost_conf_w10_reg::R](host_slchost_conf_w10_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W10_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w10_reg::W](host_slchost_conf_w10_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W10_REG {}
#[doc = "HOST_SLCHOST_CONF_W10_REG"]
pub mod host_slchost_conf_w10_reg;
#[doc = "HOST_SLCHOST_CONF_W11_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w11_reg](host_slchost_conf_w11_reg) module"]
pub type HOST_SLCHOST_CONF_W11_REG = crate::Reg<u32, _HOST_SLCHOST_CONF_W11_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W11_REG;
#[doc = "`read()` method returns [host_slchost_conf_w11_reg::R](host_slchost_conf_w11_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W11_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w11_reg::W](host_slchost_conf_w11_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W11_REG {}
#[doc = "HOST_SLCHOST_CONF_W11_REG"]
pub mod host_slchost_conf_w11_reg;
#[doc = "HOST_SLCHOST_CONF_W12_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w12_reg](host_slchost_conf_w12_reg) module"]
pub type HOST_SLCHOST_CONF_W12_REG = crate::Reg<u32, _HOST_SLCHOST_CONF_W12_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W12_REG;
#[doc = "`read()` method returns [host_slchost_conf_w12_reg::R](host_slchost_conf_w12_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W12_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w12_reg::W](host_slchost_conf_w12_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W12_REG {}
#[doc = "HOST_SLCHOST_CONF_W12_REG"]
pub mod host_slchost_conf_w12_reg;
#[doc = "HOST_SLCHOST_CONF_W13_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w13_reg](host_slchost_conf_w13_reg) module"]
pub type HOST_SLCHOST_CONF_W13_REG = crate::Reg<u32, _HOST_SLCHOST_CONF_W13_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W13_REG;
#[doc = "`read()` method returns [host_slchost_conf_w13_reg::R](host_slchost_conf_w13_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W13_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w13_reg::W](host_slchost_conf_w13_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W13_REG {}
#[doc = "HOST_SLCHOST_CONF_W13_REG"]
pub mod host_slchost_conf_w13_reg;
#[doc = "HOST_SLCHOST_CONF_W14_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w14_reg](host_slchost_conf_w14_reg) module"]
pub type HOST_SLCHOST_CONF_W14_REG = crate::Reg<u32, _HOST_SLCHOST_CONF_W14_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W14_REG;
#[doc = "`read()` method returns [host_slchost_conf_w14_reg::R](host_slchost_conf_w14_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W14_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w14_reg::W](host_slchost_conf_w14_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W14_REG {}
#[doc = "HOST_SLCHOST_CONF_W14_REG"]
pub mod host_slchost_conf_w14_reg;
#[doc = "HOST_SLCHOST_CONF_W15_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_w15_reg](host_slchost_conf_w15_reg) module"]
pub type HOST_SLCHOST_CONF_W15_REG = crate::Reg<u32, _HOST_SLCHOST_CONF_W15_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_W15_REG;
#[doc = "`read()` method returns [host_slchost_conf_w15_reg::R](host_slchost_conf_w15_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W15_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_w15_reg::W](host_slchost_conf_w15_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W15_REG {}
#[doc = "HOST_SLCHOST_CONF_W15_REG"]
pub mod host_slchost_conf_w15_reg;
#[doc = "HOST_SLCHOST_CHECK_SUM0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_check_sum0_reg](host_slchost_check_sum0_reg) module"]
pub type HOST_SLCHOST_CHECK_SUM0_REG = crate::Reg<u32, _HOST_SLCHOST_CHECK_SUM0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CHECK_SUM0_REG;
#[doc = "`read()` method returns [host_slchost_check_sum0_reg::R](host_slchost_check_sum0_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CHECK_SUM0_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_check_sum0_reg::W](host_slchost_check_sum0_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CHECK_SUM0_REG {}
#[doc = "HOST_SLCHOST_CHECK_SUM0_REG"]
pub mod host_slchost_check_sum0_reg;
#[doc = "HOST_SLCHOST_CHECK_SUM1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_check_sum1_reg](host_slchost_check_sum1_reg) module"]
pub type HOST_SLCHOST_CHECK_SUM1_REG = crate::Reg<u32, _HOST_SLCHOST_CHECK_SUM1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CHECK_SUM1_REG;
#[doc = "`read()` method returns [host_slchost_check_sum1_reg::R](host_slchost_check_sum1_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CHECK_SUM1_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_check_sum1_reg::W](host_slchost_check_sum1_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CHECK_SUM1_REG {}
#[doc = "HOST_SLCHOST_CHECK_SUM1_REG"]
pub mod host_slchost_check_sum1_reg;
#[doc = "HOST_SLC1HOST_TOKEN_RDATA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_token_rdata_reg](host_slc1host_token_rdata_reg) module"]
pub type HOST_SLC1HOST_TOKEN_RDATA_REG = crate::Reg<u32, _HOST_SLC1HOST_TOKEN_RDATA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_TOKEN_RDATA_REG;
#[doc = "`read()` method returns [host_slc1host_token_rdata_reg::R](host_slc1host_token_rdata_reg::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_TOKEN_RDATA_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_token_rdata_reg::W](host_slc1host_token_rdata_reg::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_TOKEN_RDATA_REG {}
#[doc = "HOST_SLC1HOST_TOKEN_RDATA_REG"]
pub mod host_slc1host_token_rdata_reg;
#[doc = "HOST_SLC0HOST_TOKEN_WDATA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_token_wdata_reg](host_slc0host_token_wdata_reg) module"]
pub type HOST_SLC0HOST_TOKEN_WDATA_REG = crate::Reg<u32, _HOST_SLC0HOST_TOKEN_WDATA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_TOKEN_WDATA_REG;
#[doc = "`read()` method returns [host_slc0host_token_wdata_reg::R](host_slc0host_token_wdata_reg::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_TOKEN_WDATA_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_token_wdata_reg::W](host_slc0host_token_wdata_reg::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_TOKEN_WDATA_REG {}
#[doc = "HOST_SLC0HOST_TOKEN_WDATA_REG"]
pub mod host_slc0host_token_wdata_reg;
#[doc = "HOST_SLC1HOST_TOKEN_WDATA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_token_wdata_reg](host_slc1host_token_wdata_reg) module"]
pub type HOST_SLC1HOST_TOKEN_WDATA_REG = crate::Reg<u32, _HOST_SLC1HOST_TOKEN_WDATA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_TOKEN_WDATA_REG;
#[doc = "`read()` method returns [host_slc1host_token_wdata_reg::R](host_slc1host_token_wdata_reg::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_TOKEN_WDATA_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_token_wdata_reg::W](host_slc1host_token_wdata_reg::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_TOKEN_WDATA_REG {}
#[doc = "HOST_SLC1HOST_TOKEN_WDATA_REG"]
pub mod host_slc1host_token_wdata_reg;
#[doc = "HOST_SLCHOST_TOKEN_CON_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_token_con_reg](host_slchost_token_con_reg) module"]
pub type HOST_SLCHOST_TOKEN_CON_REG = crate::Reg<u32, _HOST_SLCHOST_TOKEN_CON_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_TOKEN_CON_REG;
#[doc = "`read()` method returns [host_slchost_token_con_reg::R](host_slchost_token_con_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_TOKEN_CON_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_token_con_reg::W](host_slchost_token_con_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_TOKEN_CON_REG {}
#[doc = "HOST_SLCHOST_TOKEN_CON_REG"]
pub mod host_slchost_token_con_reg;
#[doc = "HOST_SLC0HOST_INT_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_int_clr_reg](host_slc0host_int_clr_reg) module"]
pub type HOST_SLC0HOST_INT_CLR_REG = crate::Reg<u32, _HOST_SLC0HOST_INT_CLR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_INT_CLR_REG;
#[doc = "`read()` method returns [host_slc0host_int_clr_reg::R](host_slc0host_int_clr_reg::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_INT_CLR_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_int_clr_reg::W](host_slc0host_int_clr_reg::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_INT_CLR_REG {}
#[doc = "HOST_SLC0HOST_INT_CLR_REG"]
pub mod host_slc0host_int_clr_reg;
#[doc = "HOST_SLC1HOST_INT_CLR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_int_clr_reg](host_slc1host_int_clr_reg) module"]
pub type HOST_SLC1HOST_INT_CLR_REG = crate::Reg<u32, _HOST_SLC1HOST_INT_CLR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_INT_CLR_REG;
#[doc = "`read()` method returns [host_slc1host_int_clr_reg::R](host_slc1host_int_clr_reg::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_INT_CLR_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_int_clr_reg::W](host_slc1host_int_clr_reg::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_INT_CLR_REG {}
#[doc = "HOST_SLC1HOST_INT_CLR_REG"]
pub mod host_slc1host_int_clr_reg;
#[doc = "HOST_SLC0HOST_FUNC1_INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_func1_int_ena_reg](host_slc0host_func1_int_ena_reg) module"]
pub type HOST_SLC0HOST_FUNC1_INT_ENA_REG = crate::Reg<u32, _HOST_SLC0HOST_FUNC1_INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_FUNC1_INT_ENA_REG;
#[doc = "`read()` method returns [host_slc0host_func1_int_ena_reg::R](host_slc0host_func1_int_ena_reg::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_FUNC1_INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_func1_int_ena_reg::W](host_slc0host_func1_int_ena_reg::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_FUNC1_INT_ENA_REG {}
#[doc = "HOST_SLC0HOST_FUNC1_INT_ENA_REG"]
pub mod host_slc0host_func1_int_ena_reg;
#[doc = "HOST_SLC1HOST_FUNC1_INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_func1_int_ena_reg](host_slc1host_func1_int_ena_reg) module"]
pub type HOST_SLC1HOST_FUNC1_INT_ENA_REG = crate::Reg<u32, _HOST_SLC1HOST_FUNC1_INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_FUNC1_INT_ENA_REG;
#[doc = "`read()` method returns [host_slc1host_func1_int_ena_reg::R](host_slc1host_func1_int_ena_reg::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_FUNC1_INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_func1_int_ena_reg::W](host_slc1host_func1_int_ena_reg::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_FUNC1_INT_ENA_REG {}
#[doc = "HOST_SLC1HOST_FUNC1_INT_ENA_REG"]
pub mod host_slc1host_func1_int_ena_reg;
#[doc = "HOST_SLC0HOST_FUNC2_INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_func2_int_ena_reg](host_slc0host_func2_int_ena_reg) module"]
pub type HOST_SLC0HOST_FUNC2_INT_ENA_REG = crate::Reg<u32, _HOST_SLC0HOST_FUNC2_INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_FUNC2_INT_ENA_REG;
#[doc = "`read()` method returns [host_slc0host_func2_int_ena_reg::R](host_slc0host_func2_int_ena_reg::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_FUNC2_INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_func2_int_ena_reg::W](host_slc0host_func2_int_ena_reg::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_FUNC2_INT_ENA_REG {}
#[doc = "HOST_SLC0HOST_FUNC2_INT_ENA_REG"]
pub mod host_slc0host_func2_int_ena_reg;
#[doc = "HOST_SLC1HOST_FUNC2_INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_func2_int_ena_reg](host_slc1host_func2_int_ena_reg) module"]
pub type HOST_SLC1HOST_FUNC2_INT_ENA_REG = crate::Reg<u32, _HOST_SLC1HOST_FUNC2_INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_FUNC2_INT_ENA_REG;
#[doc = "`read()` method returns [host_slc1host_func2_int_ena_reg::R](host_slc1host_func2_int_ena_reg::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_FUNC2_INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_func2_int_ena_reg::W](host_slc1host_func2_int_ena_reg::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_FUNC2_INT_ENA_REG {}
#[doc = "HOST_SLC1HOST_FUNC2_INT_ENA_REG"]
pub mod host_slc1host_func2_int_ena_reg;
#[doc = "HOST_SLC0HOST_INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_int_ena_reg](host_slc0host_int_ena_reg) module"]
pub type HOST_SLC0HOST_INT_ENA_REG = crate::Reg<u32, _HOST_SLC0HOST_INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_INT_ENA_REG;
#[doc = "`read()` method returns [host_slc0host_int_ena_reg::R](host_slc0host_int_ena_reg::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_int_ena_reg::W](host_slc0host_int_ena_reg::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_INT_ENA_REG {}
#[doc = "HOST_SLC0HOST_INT_ENA_REG"]
pub mod host_slc0host_int_ena_reg;
#[doc = "HOST_SLC1HOST_INT_ENA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_int_ena_reg](host_slc1host_int_ena_reg) module"]
pub type HOST_SLC1HOST_INT_ENA_REG = crate::Reg<u32, _HOST_SLC1HOST_INT_ENA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_INT_ENA_REG;
#[doc = "`read()` method returns [host_slc1host_int_ena_reg::R](host_slc1host_int_ena_reg::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_INT_ENA_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_int_ena_reg::W](host_slc1host_int_ena_reg::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_INT_ENA_REG {}
#[doc = "HOST_SLC1HOST_INT_ENA_REG"]
pub mod host_slc1host_int_ena_reg;
#[doc = "HOST_SLC0HOST_RX_INFOR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_rx_infor_reg](host_slc0host_rx_infor_reg) module"]
pub type HOST_SLC0HOST_RX_INFOR_REG = crate::Reg<u32, _HOST_SLC0HOST_RX_INFOR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_RX_INFOR_REG;
#[doc = "`read()` method returns [host_slc0host_rx_infor_reg::R](host_slc0host_rx_infor_reg::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_RX_INFOR_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_rx_infor_reg::W](host_slc0host_rx_infor_reg::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_RX_INFOR_REG {}
#[doc = "HOST_SLC0HOST_RX_INFOR_REG"]
pub mod host_slc0host_rx_infor_reg;
#[doc = "HOST_SLC1HOST_RX_INFOR_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_rx_infor_reg](host_slc1host_rx_infor_reg) module"]
pub type HOST_SLC1HOST_RX_INFOR_REG = crate::Reg<u32, _HOST_SLC1HOST_RX_INFOR_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_RX_INFOR_REG;
#[doc = "`read()` method returns [host_slc1host_rx_infor_reg::R](host_slc1host_rx_infor_reg::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_RX_INFOR_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_rx_infor_reg::W](host_slc1host_rx_infor_reg::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_RX_INFOR_REG {}
#[doc = "HOST_SLC1HOST_RX_INFOR_REG"]
pub mod host_slc1host_rx_infor_reg;
#[doc = "HOST_SLC0HOST_LEN_WD_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_len_wd_reg](host_slc0host_len_wd_reg) module"]
pub type HOST_SLC0HOST_LEN_WD_REG = crate::Reg<u32, _HOST_SLC0HOST_LEN_WD_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_LEN_WD_REG;
#[doc = "`read()` method returns [host_slc0host_len_wd_reg::R](host_slc0host_len_wd_reg::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_LEN_WD_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_len_wd_reg::W](host_slc0host_len_wd_reg::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_LEN_WD_REG {}
#[doc = "HOST_SLC0HOST_LEN_WD_REG"]
pub mod host_slc0host_len_wd_reg;
#[doc = "HOST_SLC_APBWIN_WDATA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc_apbwin_wdata_reg](host_slc_apbwin_wdata_reg) module"]
pub type HOST_SLC_APBWIN_WDATA_REG = crate::Reg<u32, _HOST_SLC_APBWIN_WDATA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC_APBWIN_WDATA_REG;
#[doc = "`read()` method returns [host_slc_apbwin_wdata_reg::R](host_slc_apbwin_wdata_reg::R) reader structure"]
impl crate::Readable for HOST_SLC_APBWIN_WDATA_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc_apbwin_wdata_reg::W](host_slc_apbwin_wdata_reg::W) writer structure"]
impl crate::Writable for HOST_SLC_APBWIN_WDATA_REG {}
#[doc = "HOST_SLC_APBWIN_WDATA_REG"]
pub mod host_slc_apbwin_wdata_reg;
#[doc = "HOST_SLC_APBWIN_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc_apbwin_conf_reg](host_slc_apbwin_conf_reg) module"]
pub type HOST_SLC_APBWIN_CONF_REG = crate::Reg<u32, _HOST_SLC_APBWIN_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC_APBWIN_CONF_REG;
#[doc = "`read()` method returns [host_slc_apbwin_conf_reg::R](host_slc_apbwin_conf_reg::R) reader structure"]
impl crate::Readable for HOST_SLC_APBWIN_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc_apbwin_conf_reg::W](host_slc_apbwin_conf_reg::W) writer structure"]
impl crate::Writable for HOST_SLC_APBWIN_CONF_REG {}
#[doc = "HOST_SLC_APBWIN_CONF_REG"]
pub mod host_slc_apbwin_conf_reg;
#[doc = "HOST_SLC_APBWIN_RDATA_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc_apbwin_rdata_reg](host_slc_apbwin_rdata_reg) module"]
pub type HOST_SLC_APBWIN_RDATA_REG = crate::Reg<u32, _HOST_SLC_APBWIN_RDATA_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC_APBWIN_RDATA_REG;
#[doc = "`read()` method returns [host_slc_apbwin_rdata_reg::R](host_slc_apbwin_rdata_reg::R) reader structure"]
impl crate::Readable for HOST_SLC_APBWIN_RDATA_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc_apbwin_rdata_reg::W](host_slc_apbwin_rdata_reg::W) writer structure"]
impl crate::Writable for HOST_SLC_APBWIN_RDATA_REG {}
#[doc = "HOST_SLC_APBWIN_RDATA_REG"]
pub mod host_slc_apbwin_rdata_reg;
#[doc = "HOST_SLCHOST_RDCLR0_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_rdclr0_reg](host_slchost_rdclr0_reg) module"]
pub type HOST_SLCHOST_RDCLR0_REG = crate::Reg<u32, _HOST_SLCHOST_RDCLR0_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_RDCLR0_REG;
#[doc = "`read()` method returns [host_slchost_rdclr0_reg::R](host_slchost_rdclr0_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_RDCLR0_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_rdclr0_reg::W](host_slchost_rdclr0_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_RDCLR0_REG {}
#[doc = "HOST_SLCHOST_RDCLR0_REG"]
pub mod host_slchost_rdclr0_reg;
#[doc = "HOST_SLCHOST_RDCLR1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_rdclr1_reg](host_slchost_rdclr1_reg) module"]
pub type HOST_SLCHOST_RDCLR1_REG = crate::Reg<u32, _HOST_SLCHOST_RDCLR1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_RDCLR1_REG;
#[doc = "`read()` method returns [host_slchost_rdclr1_reg::R](host_slchost_rdclr1_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_RDCLR1_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_rdclr1_reg::W](host_slchost_rdclr1_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_RDCLR1_REG {}
#[doc = "HOST_SLCHOST_RDCLR1_REG"]
pub mod host_slchost_rdclr1_reg;
#[doc = "HOST_SLC0HOST_INT_ENA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc0host_int_ena1_reg](host_slc0host_int_ena1_reg) module"]
pub type HOST_SLC0HOST_INT_ENA1_REG = crate::Reg<u32, _HOST_SLC0HOST_INT_ENA1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC0HOST_INT_ENA1_REG;
#[doc = "`read()` method returns [host_slc0host_int_ena1_reg::R](host_slc0host_int_ena1_reg::R) reader structure"]
impl crate::Readable for HOST_SLC0HOST_INT_ENA1_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc0host_int_ena1_reg::W](host_slc0host_int_ena1_reg::W) writer structure"]
impl crate::Writable for HOST_SLC0HOST_INT_ENA1_REG {}
#[doc = "HOST_SLC0HOST_INT_ENA1_REG"]
pub mod host_slc0host_int_ena1_reg;
#[doc = "HOST_SLC1HOST_INT_ENA1_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slc1host_int_ena1_reg](host_slc1host_int_ena1_reg) module"]
pub type HOST_SLC1HOST_INT_ENA1_REG = crate::Reg<u32, _HOST_SLC1HOST_INT_ENA1_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLC1HOST_INT_ENA1_REG;
#[doc = "`read()` method returns [host_slc1host_int_ena1_reg::R](host_slc1host_int_ena1_reg::R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_INT_ENA1_REG {}
#[doc = "`write(|w| ..)` method takes [host_slc1host_int_ena1_reg::W](host_slc1host_int_ena1_reg::W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_INT_ENA1_REG {}
#[doc = "HOST_SLC1HOST_INT_ENA1_REG"]
pub mod host_slc1host_int_ena1_reg;
#[doc = "HOST_SLCHOSTDATE_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchostdate_reg](host_slchostdate_reg) module"]
pub type HOST_SLCHOSTDATE_REG = crate::Reg<u32, _HOST_SLCHOSTDATE_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOSTDATE_REG;
#[doc = "`read()` method returns [host_slchostdate_reg::R](host_slchostdate_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOSTDATE_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchostdate_reg::W](host_slchostdate_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOSTDATE_REG {}
#[doc = "HOST_SLCHOSTDATE_REG"]
pub mod host_slchostdate_reg;
#[doc = "HOST_SLCHOSTID_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchostid_reg](host_slchostid_reg) module"]
pub type HOST_SLCHOSTID_REG = crate::Reg<u32, _HOST_SLCHOSTID_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOSTID_REG;
#[doc = "`read()` method returns [host_slchostid_reg::R](host_slchostid_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOSTID_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchostid_reg::W](host_slchostid_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOSTID_REG {}
#[doc = "HOST_SLCHOSTID_REG"]
pub mod host_slchostid_reg;
#[doc = "HOST_SLCHOST_CONF_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_conf_reg](host_slchost_conf_reg) module"]
pub type HOST_SLCHOST_CONF_REG = crate::Reg<u32, _HOST_SLCHOST_CONF_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_CONF_REG;
#[doc = "`read()` method returns [host_slchost_conf_reg::R](host_slchost_conf_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_conf_reg::W](host_slchost_conf_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_REG {}
#[doc = "HOST_SLCHOST_CONF_REG"]
pub mod host_slchost_conf_reg;
#[doc = "HOST_SLCHOST_INF_ST_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [host_slchost_inf_st_reg](host_slchost_inf_st_reg) module"]
pub type HOST_SLCHOST_INF_ST_REG = crate::Reg<u32, _HOST_SLCHOST_INF_ST_REG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOST_SLCHOST_INF_ST_REG;
#[doc = "`read()` method returns [host_slchost_inf_st_reg::R](host_slchost_inf_st_reg::R) reader structure"]
impl crate::Readable for HOST_SLCHOST_INF_ST_REG {}
#[doc = "`write(|w| ..)` method takes [host_slchost_inf_st_reg::W](host_slchost_inf_st_reg::W) writer structure"]
impl crate::Writable for HOST_SLCHOST_INF_ST_REG {}
#[doc = "HOST_SLCHOST_INF_ST_REG"]
pub mod host_slchost_inf_st_reg;
