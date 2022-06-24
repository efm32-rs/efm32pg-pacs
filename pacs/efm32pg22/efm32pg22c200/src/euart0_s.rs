#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: crate::Reg<ipversion::IPVERSION_SPEC>,
    #[doc = "0x04 - No Description"]
    pub en: crate::Reg<en::EN_SPEC>,
    #[doc = "0x08 - No Description"]
    pub cfg0: crate::Reg<cfg0::CFG0_SPEC>,
    #[doc = "0x0c - No Description"]
    pub cfg1: crate::Reg<cfg1::CFG1_SPEC>,
    #[doc = "0x10 - No Description"]
    pub framecfg: crate::Reg<framecfg::FRAMECFG_SPEC>,
    #[doc = "0x14 - No Description"]
    pub irhfcfg: crate::Reg<irhfcfg::IRHFCFG_SPEC>,
    #[doc = "0x18 - No Description"]
    pub irlfcfg: crate::Reg<irlfcfg::IRLFCFG_SPEC>,
    #[doc = "0x1c - No Description"]
    pub timingcfg: crate::Reg<timingcfg::TIMINGCFG_SPEC>,
    #[doc = "0x20 - No Description"]
    pub startframecfg: crate::Reg<startframecfg::STARTFRAMECFG_SPEC>,
    #[doc = "0x24 - No Description"]
    pub sigframecfg: crate::Reg<sigframecfg::SIGFRAMECFG_SPEC>,
    #[doc = "0x28 - No Description"]
    pub clkdiv: crate::Reg<clkdiv::CLKDIV_SPEC>,
    #[doc = "0x2c - No Description"]
    pub trigctrl: crate::Reg<trigctrl::TRIGCTRL_SPEC>,
    #[doc = "0x30 - No Description"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x34 - No Description"]
    pub rxdata: crate::Reg<rxdata::RXDATA_SPEC>,
    #[doc = "0x38 - No Description"]
    pub rxdatap: crate::Reg<rxdatap::RXDATAP_SPEC>,
    #[doc = "0x3c - No Description"]
    pub txdata: crate::Reg<txdata::TXDATA_SPEC>,
    #[doc = "0x40 - No Description"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x44 - No Description"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x48 - No Description"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x4c - No Description"]
    pub syncbusy: crate::Reg<syncbusy::SYNCBUSY_SPEC>,
}
#[doc = "IPVERSION register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "EN register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "No Description"]
pub mod en;
#[doc = "CFG0 register accessor: an alias for `Reg<CFG0_SPEC>`"]
pub type CFG0 = crate::Reg<cfg0::CFG0_SPEC>;
#[doc = "No Description"]
pub mod cfg0;
#[doc = "CFG1 register accessor: an alias for `Reg<CFG1_SPEC>`"]
pub type CFG1 = crate::Reg<cfg1::CFG1_SPEC>;
#[doc = "No Description"]
pub mod cfg1;
#[doc = "FRAMECFG register accessor: an alias for `Reg<FRAMECFG_SPEC>`"]
pub type FRAMECFG = crate::Reg<framecfg::FRAMECFG_SPEC>;
#[doc = "No Description"]
pub mod framecfg;
#[doc = "IRHFCFG register accessor: an alias for `Reg<IRHFCFG_SPEC>`"]
pub type IRHFCFG = crate::Reg<irhfcfg::IRHFCFG_SPEC>;
#[doc = "No Description"]
pub mod irhfcfg;
#[doc = "IRLFCFG register accessor: an alias for `Reg<IRLFCFG_SPEC>`"]
pub type IRLFCFG = crate::Reg<irlfcfg::IRLFCFG_SPEC>;
#[doc = "No Description"]
pub mod irlfcfg;
#[doc = "TIMINGCFG register accessor: an alias for `Reg<TIMINGCFG_SPEC>`"]
pub type TIMINGCFG = crate::Reg<timingcfg::TIMINGCFG_SPEC>;
#[doc = "No Description"]
pub mod timingcfg;
#[doc = "STARTFRAMECFG register accessor: an alias for `Reg<STARTFRAMECFG_SPEC>`"]
pub type STARTFRAMECFG = crate::Reg<startframecfg::STARTFRAMECFG_SPEC>;
#[doc = "No Description"]
pub mod startframecfg;
#[doc = "SIGFRAMECFG register accessor: an alias for `Reg<SIGFRAMECFG_SPEC>`"]
pub type SIGFRAMECFG = crate::Reg<sigframecfg::SIGFRAMECFG_SPEC>;
#[doc = "No Description"]
pub mod sigframecfg;
#[doc = "CLKDIV register accessor: an alias for `Reg<CLKDIV_SPEC>`"]
pub type CLKDIV = crate::Reg<clkdiv::CLKDIV_SPEC>;
#[doc = "No Description"]
pub mod clkdiv;
#[doc = "TRIGCTRL register accessor: an alias for `Reg<TRIGCTRL_SPEC>`"]
pub type TRIGCTRL = crate::Reg<trigctrl::TRIGCTRL_SPEC>;
#[doc = "No Description"]
pub mod trigctrl;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "RXDATA register accessor: an alias for `Reg<RXDATA_SPEC>`"]
pub type RXDATA = crate::Reg<rxdata::RXDATA_SPEC>;
#[doc = "No Description"]
pub mod rxdata;
#[doc = "RXDATAP register accessor: an alias for `Reg<RXDATAP_SPEC>`"]
pub type RXDATAP = crate::Reg<rxdatap::RXDATAP_SPEC>;
#[doc = "No Description"]
pub mod rxdatap;
#[doc = "TXDATA register accessor: an alias for `Reg<TXDATA_SPEC>`"]
pub type TXDATA = crate::Reg<txdata::TXDATA_SPEC>;
#[doc = "No Description"]
pub mod txdata;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "SYNCBUSY register accessor: an alias for `Reg<SYNCBUSY_SPEC>`"]
pub type SYNCBUSY = crate::Reg<syncbusy::SYNCBUSY_SPEC>;
#[doc = "No Description"]
pub mod syncbusy;
