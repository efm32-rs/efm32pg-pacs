#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Configuration Lock Register"]
    pub lock: LOCK,
    #[doc = "0x0c - Memory Control Register"]
    pub ram0ctrl: RAM0CTRL,
    #[doc = "0x10 - Command Register"]
    pub cmd: CMD,
    _reserved5: [u8; 0x04],
    #[doc = "0x18 - EM4 Control Register"]
    pub em4ctrl: EM4CTRL,
    #[doc = "0x1c - Temperature Limits for Interrupt Generation"]
    pub templimits: TEMPLIMITS,
    #[doc = "0x20 - Value of Last Temperature Measurement"]
    pub temp: TEMP,
    #[doc = "0x24 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x28 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x2c - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x30 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x34 - Regulator and Supply Lock Register"]
    pub pwrlock: PWRLOCK,
    #[doc = "0x38 - Power Configuration Register"]
    pub pwrcfg: PWRCFG,
    #[doc = "0x3c - Power Control Register"]
    pub pwrctrl: PWRCTRL,
    #[doc = "0x40 - DCDC Control"]
    pub dcdcctrl: DCDCCTRL,
    _reserved16: [u8; 0x08],
    #[doc = "0x4c - DCDC Miscellaneous Control Register"]
    pub dcdcmiscctrl: DCDCMISCCTRL,
    #[doc = "0x50 - DCDC Power Train NFET Zero Current Detector Control Register"]
    pub dcdczdetctrl: DCDCZDETCTRL,
    #[doc = "0x54 - DCDC Power Train PFET Current Limiter Control Register"]
    pub dcdcclimctrl: DCDCCLIMCTRL,
    #[doc = "0x58 - DCDC Low Noise Compensator Control Register"]
    pub dcdclncompctrl: DCDCLNCOMPCTRL,
    #[doc = "0x5c - DCDC Low Noise Voltage Register"]
    pub dcdclnvctrl: DCDCLNVCTRL,
    #[doc = "0x60 - DCDC Controller Timing Value Register"]
    pub dcdctiming: DCDCTIMING,
    #[doc = "0x64 - DCDC Low Power Voltage Register"]
    pub dcdclpvctrl: DCDCLPVCTRL,
    _reserved23: [u8; 0x04],
    #[doc = "0x6c - DCDC Low Power Control Register"]
    pub dcdclpctrl: DCDCLPCTRL,
    #[doc = "0x70 - DCDC Low Noise Controller Frequency Control"]
    pub dcdclnfreqctrl: DCDCLNFREQCTRL,
    _reserved25: [u8; 0x04],
    #[doc = "0x78 - DCDC Read Status Register"]
    pub dcdcsync: DCDCSYNC,
    _reserved26: [u8; 0x14],
    #[doc = "0x90 - VMON AVDD Channel Control"]
    pub vmonavddctrl: VMONAVDDCTRL,
    #[doc = "0x94 - Alternate VMON AVDD Channel Control"]
    pub vmonaltavddctrl: VMONALTAVDDCTRL,
    #[doc = "0x98 - VMON DVDD Channel Control"]
    pub vmondvddctrl: VMONDVDDCTRL,
    #[doc = "0x9c - VMON IOVDD0 Channel Control"]
    pub vmonio0ctrl: VMONIO0CTRL,
    _reserved30: [u8; 0xc4],
    #[doc = "0x164 - Configurations Related to the Bias"]
    pub biasconf: BIASCONF,
    _reserved31: [u8; 0x28],
    #[doc = "0x190 - Test Lock Register"]
    pub testlock: TESTLOCK,
    _reserved32: [u8; 0x08],
    #[doc = "0x19c - Test Control Register for Regulator and BIAS"]
    pub biastestctrl: BIASTESTCTRL,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "Status Register"]
pub mod status;
#[doc = "LOCK (rw) register accessor: an alias for `Reg<LOCK_SPEC>`"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
#[doc = "RAM0CTRL (rw) register accessor: an alias for `Reg<RAM0CTRL_SPEC>`"]
pub type RAM0CTRL = crate::Reg<ram0ctrl::RAM0CTRL_SPEC>;
#[doc = "Memory Control Register"]
pub mod ram0ctrl;
#[doc = "CMD (w) register accessor: an alias for `Reg<CMD_SPEC>`"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "EM4CTRL (rw) register accessor: an alias for `Reg<EM4CTRL_SPEC>`"]
pub type EM4CTRL = crate::Reg<em4ctrl::EM4CTRL_SPEC>;
#[doc = "EM4 Control Register"]
pub mod em4ctrl;
#[doc = "TEMPLIMITS (rw) register accessor: an alias for `Reg<TEMPLIMITS_SPEC>`"]
pub type TEMPLIMITS = crate::Reg<templimits::TEMPLIMITS_SPEC>;
#[doc = "Temperature Limits for Interrupt Generation"]
pub mod templimits;
#[doc = "TEMP (r) register accessor: an alias for `Reg<TEMP_SPEC>`"]
pub type TEMP = crate::Reg<temp::TEMP_SPEC>;
#[doc = "Value of Last Temperature Measurement"]
pub mod temp;
#[doc = "IF (r) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: an alias for `Reg<IFS_SPEC>`"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: an alias for `Reg<IFC_SPEC>`"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "PWRLOCK (rw) register accessor: an alias for `Reg<PWRLOCK_SPEC>`"]
pub type PWRLOCK = crate::Reg<pwrlock::PWRLOCK_SPEC>;
#[doc = "Regulator and Supply Lock Register"]
pub mod pwrlock;
#[doc = "PWRCFG (rw) register accessor: an alias for `Reg<PWRCFG_SPEC>`"]
pub type PWRCFG = crate::Reg<pwrcfg::PWRCFG_SPEC>;
#[doc = "Power Configuration Register"]
pub mod pwrcfg;
#[doc = "PWRCTRL (rw) register accessor: an alias for `Reg<PWRCTRL_SPEC>`"]
pub type PWRCTRL = crate::Reg<pwrctrl::PWRCTRL_SPEC>;
#[doc = "Power Control Register"]
pub mod pwrctrl;
#[doc = "DCDCCTRL (rw) register accessor: an alias for `Reg<DCDCCTRL_SPEC>`"]
pub type DCDCCTRL = crate::Reg<dcdcctrl::DCDCCTRL_SPEC>;
#[doc = "DCDC Control"]
pub mod dcdcctrl;
#[doc = "DCDCMISCCTRL (rw) register accessor: an alias for `Reg<DCDCMISCCTRL_SPEC>`"]
pub type DCDCMISCCTRL = crate::Reg<dcdcmiscctrl::DCDCMISCCTRL_SPEC>;
#[doc = "DCDC Miscellaneous Control Register"]
pub mod dcdcmiscctrl;
#[doc = "DCDCZDETCTRL (rw) register accessor: an alias for `Reg<DCDCZDETCTRL_SPEC>`"]
pub type DCDCZDETCTRL = crate::Reg<dcdczdetctrl::DCDCZDETCTRL_SPEC>;
#[doc = "DCDC Power Train NFET Zero Current Detector Control Register"]
pub mod dcdczdetctrl;
#[doc = "DCDCCLIMCTRL (rw) register accessor: an alias for `Reg<DCDCCLIMCTRL_SPEC>`"]
pub type DCDCCLIMCTRL = crate::Reg<dcdcclimctrl::DCDCCLIMCTRL_SPEC>;
#[doc = "DCDC Power Train PFET Current Limiter Control Register"]
pub mod dcdcclimctrl;
#[doc = "DCDCLNCOMPCTRL (rw) register accessor: an alias for `Reg<DCDCLNCOMPCTRL_SPEC>`"]
pub type DCDCLNCOMPCTRL = crate::Reg<dcdclncompctrl::DCDCLNCOMPCTRL_SPEC>;
#[doc = "DCDC Low Noise Compensator Control Register"]
pub mod dcdclncompctrl;
#[doc = "DCDCLNVCTRL (rw) register accessor: an alias for `Reg<DCDCLNVCTRL_SPEC>`"]
pub type DCDCLNVCTRL = crate::Reg<dcdclnvctrl::DCDCLNVCTRL_SPEC>;
#[doc = "DCDC Low Noise Voltage Register"]
pub mod dcdclnvctrl;
#[doc = "DCDCTIMING (rw) register accessor: an alias for `Reg<DCDCTIMING_SPEC>`"]
pub type DCDCTIMING = crate::Reg<dcdctiming::DCDCTIMING_SPEC>;
#[doc = "DCDC Controller Timing Value Register"]
pub mod dcdctiming;
#[doc = "DCDCLPVCTRL (rw) register accessor: an alias for `Reg<DCDCLPVCTRL_SPEC>`"]
pub type DCDCLPVCTRL = crate::Reg<dcdclpvctrl::DCDCLPVCTRL_SPEC>;
#[doc = "DCDC Low Power Voltage Register"]
pub mod dcdclpvctrl;
#[doc = "DCDCLPCTRL (rw) register accessor: an alias for `Reg<DCDCLPCTRL_SPEC>`"]
pub type DCDCLPCTRL = crate::Reg<dcdclpctrl::DCDCLPCTRL_SPEC>;
#[doc = "DCDC Low Power Control Register"]
pub mod dcdclpctrl;
#[doc = "DCDCLNFREQCTRL (rw) register accessor: an alias for `Reg<DCDCLNFREQCTRL_SPEC>`"]
pub type DCDCLNFREQCTRL = crate::Reg<dcdclnfreqctrl::DCDCLNFREQCTRL_SPEC>;
#[doc = "DCDC Low Noise Controller Frequency Control"]
pub mod dcdclnfreqctrl;
#[doc = "DCDCSYNC (r) register accessor: an alias for `Reg<DCDCSYNC_SPEC>`"]
pub type DCDCSYNC = crate::Reg<dcdcsync::DCDCSYNC_SPEC>;
#[doc = "DCDC Read Status Register"]
pub mod dcdcsync;
#[doc = "VMONAVDDCTRL (rw) register accessor: an alias for `Reg<VMONAVDDCTRL_SPEC>`"]
pub type VMONAVDDCTRL = crate::Reg<vmonavddctrl::VMONAVDDCTRL_SPEC>;
#[doc = "VMON AVDD Channel Control"]
pub mod vmonavddctrl;
#[doc = "VMONALTAVDDCTRL (rw) register accessor: an alias for `Reg<VMONALTAVDDCTRL_SPEC>`"]
pub type VMONALTAVDDCTRL = crate::Reg<vmonaltavddctrl::VMONALTAVDDCTRL_SPEC>;
#[doc = "Alternate VMON AVDD Channel Control"]
pub mod vmonaltavddctrl;
#[doc = "VMONDVDDCTRL (rw) register accessor: an alias for `Reg<VMONDVDDCTRL_SPEC>`"]
pub type VMONDVDDCTRL = crate::Reg<vmondvddctrl::VMONDVDDCTRL_SPEC>;
#[doc = "VMON DVDD Channel Control"]
pub mod vmondvddctrl;
#[doc = "VMONIO0CTRL (rw) register accessor: an alias for `Reg<VMONIO0CTRL_SPEC>`"]
pub type VMONIO0CTRL = crate::Reg<vmonio0ctrl::VMONIO0CTRL_SPEC>;
#[doc = "VMON IOVDD0 Channel Control"]
pub mod vmonio0ctrl;
#[doc = "BIASCONF (rw) register accessor: an alias for `Reg<BIASCONF_SPEC>`"]
pub type BIASCONF = crate::Reg<biasconf::BIASCONF_SPEC>;
#[doc = "Configurations Related to the Bias"]
pub mod biasconf;
#[doc = "TESTLOCK (rw) register accessor: an alias for `Reg<TESTLOCK_SPEC>`"]
pub type TESTLOCK = crate::Reg<testlock::TESTLOCK_SPEC>;
#[doc = "Test Lock Register"]
pub mod testlock;
#[doc = "BIASTESTCTRL (rw) register accessor: an alias for `Reg<BIASTESTCTRL_SPEC>`"]
pub type BIASTESTCTRL = crate::Reg<biastestctrl::BIASTESTCTRL_SPEC>;
#[doc = "Test Control Register for Regulator and BIAS"]
pub mod biastestctrl;
