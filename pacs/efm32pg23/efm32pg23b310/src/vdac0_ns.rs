#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - No Description"]
    pub ipversion: crate::Reg<ipversion::IPVERSION_SPEC>,
    #[doc = "0x04 - No Description"]
    pub en: crate::Reg<en::EN_SPEC>,
    #[doc = "0x08 - No Description"]
    pub swrst: crate::Reg<swrst::SWRST_SPEC>,
    #[doc = "0x0c - No Description"]
    pub cfg: crate::Reg<cfg::CFG_SPEC>,
    #[doc = "0x10 - No Description"]
    pub status: crate::Reg<status::STATUS_SPEC>,
    #[doc = "0x14 - No Description"]
    pub ch0cfg: crate::Reg<ch0cfg::CH0CFG_SPEC>,
    #[doc = "0x18 - No Description"]
    pub ch1cfg: crate::Reg<ch1cfg::CH1CFG_SPEC>,
    #[doc = "0x1c - No Description"]
    pub cmd: crate::Reg<cmd::CMD_SPEC>,
    #[doc = "0x20 - No Description"]
    pub if_: crate::Reg<if_::IF_SPEC>,
    #[doc = "0x24 - No Description"]
    pub ien: crate::Reg<ien::IEN_SPEC>,
    #[doc = "0x28 - No Description"]
    pub ch0f: crate::Reg<ch0f::CH0F_SPEC>,
    #[doc = "0x2c - No Description"]
    pub ch1f: crate::Reg<ch1f::CH1F_SPEC>,
    #[doc = "0x30 - No Description"]
    pub outctrl: crate::Reg<outctrl::OUTCTRL_SPEC>,
    #[doc = "0x34 - No Description"]
    pub outtimercfg: crate::Reg<outtimercfg::OUTTIMERCFG_SPEC>,
}
#[doc = "IPVERSION register accessor: an alias for `Reg<IPVERSION_SPEC>`"]
pub type IPVERSION = crate::Reg<ipversion::IPVERSION_SPEC>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "EN register accessor: an alias for `Reg<EN_SPEC>`"]
pub type EN = crate::Reg<en::EN_SPEC>;
#[doc = "No Description"]
pub mod en;
#[doc = "SWRST register accessor: an alias for `Reg<SWRST_SPEC>`"]
pub type SWRST = crate::Reg<swrst::SWRST_SPEC>;
#[doc = "No Description"]
pub mod swrst;
#[doc = "CFG register accessor: an alias for `Reg<CFG_SPEC>`"]
pub type CFG = crate::Reg<cfg::CFG_SPEC>;
#[doc = "No Description"]
pub mod cfg;
#[doc = "STATUS register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "No Description"]
pub mod status;
#[doc = "CH0CFG register accessor: an alias for `Reg<CH0CFG_SPEC>`"]
pub type CH0CFG = crate::Reg<ch0cfg::CH0CFG_SPEC>;
#[doc = "No Description"]
pub mod ch0cfg;
#[doc = "CH1CFG register accessor: an alias for `Reg<CH1CFG_SPEC>`"]
pub type CH1CFG = crate::Reg<ch1cfg::CH1CFG_SPEC>;
#[doc = "No Description"]
pub mod ch1cfg;
#[doc = "CMD register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "IF register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "No Description"]
pub mod ien;
#[doc = "CH0F register accessor: an alias for `Reg<CH0F_SPEC>`"]
pub type CH0F = crate::Reg<ch0f::CH0F_SPEC>;
#[doc = "No Description"]
pub mod ch0f;
#[doc = "CH1F register accessor: an alias for `Reg<CH1F_SPEC>`"]
pub type CH1F = crate::Reg<ch1f::CH1F_SPEC>;
#[doc = "No Description"]
pub mod ch1f;
#[doc = "OUTCTRL register accessor: an alias for `Reg<OUTCTRL_SPEC>`"]
pub type OUTCTRL = crate::Reg<outctrl::OUTCTRL_SPEC>;
#[doc = "No Description"]
pub mod outctrl;
#[doc = "OUTTIMERCFG register accessor: an alias for `Reg<OUTTIMERCFG_SPEC>`"]
pub type OUTTIMERCFG = crate::Reg<outtimercfg::OUTTIMERCFG_SPEC>;
#[doc = "No Description"]
pub mod outtimercfg;
