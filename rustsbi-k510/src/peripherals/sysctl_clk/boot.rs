use k510_pac::sysctl;


pub enum SysctlBootModeE {
    SysctlBootDownload,
    SysctlBootSdcard,
    SysctlBootFlash,
    SysctlBootEmmc,
    SysctlBootMax,
}