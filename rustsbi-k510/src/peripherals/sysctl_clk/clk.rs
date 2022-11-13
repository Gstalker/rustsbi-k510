pub enum SysctlClkMulDivMethordE {
    SysctlClkMulChangeable,
    SysctlClkDivChangeable,
    SysctlClkMulDivChangeable,
}



pub enum SysctlClkNodeE {
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /***********************************************ROOT******************************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* clock root */
    /* sysctl_boot clk tree
       osc25m-----     ------>|-->pll0----->|--------->pll0      clk output--->to sysctl_clock module
                              |             |--div2--->pll0_div2 clk output--->to sysctl_clock module
                              |             |--div3--->pll0_div3 clk output--->to sysctl_clock module
                              |             |--div4--->pll0_div4 clk output--->to sysctl_clock module
                              |
                              |-->pll1----->|--------->pll1      clk output--->to sysctl_clock module
                              |             |--div2--->pll1_div2 clk output--->to sysctl_clock module
                              |             |--div3--->pll1_div3 clk output--->to sysctl_clock module
                              |             |--div4--->pll1_div4 clk output--->to sysctl_clock module
                              |
                              |-->pll2----->|--------->pll2      clk output--->to sysctl_clock module
                              |             |--div2--->pll2_div2 clk output--->to sysctl_clock module
                              |             |--div3--->pll2_div3 clk output--->to sysctl_clock module
                              |             |--div4--->pll2_div4 clk output--->to sysctl_clock module
                              |
                              |-->pll3----->|--------->pll3      clk output--->to sysctl_clock module
                              |             |--div2--->pll3_div2 clk output--->to sysctl_clock module
                              |             |--div3--->pll3_div3 clk output--->to sysctl_clock module
                              |             |--div4--->pll3_div4 clk output--->to sysctl_clock module
                              |
                              |----------------------------------------------->to sysctl_clock module

       osc32k----------------------------------------------------------------->to sysctl_clock module
    */
    SysctlClkRootOscIn0 = 0,    /* 25M */
    SysctlClkRootOscIn1,        /* 32K */
    SysctlClkRootPll0,           /* 1G */
    SysctlClkRootPll0Div2,     /* 500M */
    SysctlClkRootPll0Div3,     /* 333M */
    SysctlClkRootPll0Div4,     /* 250M */
    SysctlClkRootPll1,           /* 1.333G */
    SysctlClkRootPll1Div2,     /* 666M */
    SysctlClkRootPll1Div3,     /* 444M */
    SysctlClkRootPll1Div4,     /* 333M */
    SysctlClkRootPll2,           /* 2.376G */
    SysctlClkRootPll2Div2,     /* 1.188G */
    SysctlClkRootPll2Div3,     /* 792M */
    SysctlClkRootPll2Div4,     /* 594M */
    SysctlClkRootPll3,           /* 3.2G */
    SysctlClkRootPll3Div2,     /* 1.6G */
    SysctlClkRootPll3Div3,     /* 1.066G */
    SysctlClkRootPll3Div4,     /* 800M */
    SysctlClkRootMax,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************AX25MP*******************************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* ax25mp clock tree
                  +-+
      pll2_div3-->|M \         |---GATE--->core0 clk
           pll0-->|U |-->DIV-->|---GATE--->core1 clk
      pll2_div2-->|X /         |---GATE--->core0 data cache clk
                  +-+          |---GATE--->core1 data cache clk
                               |---GATE--->noc & mctl p0 axi clk

     1. ax25mp_src              --> MUX&DIV
     2. ax25mp_core0            --> core0 clk gate
     3. ax25mp_core1            --> core1 clk gate
     4. ax25mp_core0_dc         --> core0 data cache clk gate
     5. ax25mp_core1_dc         --> core1 data cache clk gate
     6. ax25mp_noc_mctl         --> ddr controller p0 & noc AXI clock (for AX25MP)
    */
    SysctlClkAx25mSrc,           /* default 500MHz ----> select pll0(1GHz), div 6/12 */


    SysctlClkAx25mCore0,
    SysctlClkAx25mCore1,        /* as same as core 0 */
    SysctlClkAx25mCore0Dc,     /* as same as core 0 */
    SysctlClkAx25mCore1Dc,     /* as same as core 0 */
    SysctlClkAx25mMctl,
    /* ax25m mtimer clock tree
                    +-+
      osc32k     -->|M \
      osc25m_gate-->|U |-->DIV-->|---GATE--->ax25m mtimer clk
                    |X /
                    +-+
    */
    SysctlClkAx25mMtimer,        /* defualt 1.25MHz ---> select OSC25M, div 20*/

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************AX25P********************************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* ax25p clock tree
                  +-+
      pll2_div3-->|M \         |---GATE--->core clk
           pll0-->|U |-->DIV-->|---GATE--->local memory clk
                  |X /
                  +-+

     1. ax25p_src               --> MUX&DIV
     2. ax25p_core              --> core clk gate
     3. ax25p_lm                --> local memory gate
    */
    SysctlClkAx25pSrc,           /* mux&div */
    SysctlClkAx25pCore,          /* gate */
    SysctlClkAx25pLm,            /* gate */

    /* ax25p mtimer clock tree
                    +-+
      osc32k     -->|M \
      osc25m_gate-->|U |-->DIV-->|---GATE--->ax25p mtimer clk
                    |X /
                    +-+

     1. ax25p_mtimer            --> MUX&DIV&GATE
    */
    SysctlClkAx25pMtimer,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************GNNE*********************************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* gnne system clock tree
                  +-+
      pll0_div2-->|M \
                  |U |-->DIV-->|---GATE--->gnne system clk
      pll1_div2-->|X /
                  +-+

     1. gnne_sys_clk_mux_div_gate               --> MUX&DIV&GATE
    */
    SysctlClkGnneSys,
    /* gnne axi clock tree
                  +-+
      pll0_div2-->|M \         |---GATE--->gnne&noc axi clk
                  |U |-->DIV-->|
      pll1_div2-->|X /         |---GATE--->gnne mctl_p0 & noc axi clk
                  +-+

     1. gnne_axi_noc_mctl_p1_mux_div            --> MUX&DIV
     2. gnne_axi_noc_gate                       --> gnne axi noc gate
     3. gnne_noc_axi_mctl_p1_gate               --> gnne noc&mctl_p1 gate
    */
    SysctlClkGnneAxi,
    SysctlClkGnneAxiNoc,
    SysctlClkGnneAxiMctl,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************NOC0*********************************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* noc clk0 tree
                  +-+
      pll0_div2-->|M \         |---DIV1--->noc_clk0--->|----GATE--->peri dma axi clk
                  |U |-->DIV-->|                       |----GATE--->sys dma axi clk
      pll1_div2-->|X /         |                       |----GATE--->sram0 axi clk
                  +-+          |                       |----GATE--->sram1 axi clk
                               |                       |----GATE--->noc&mctl_p3 clk
                               |
                               |-DIV2->noc_clk0_div2-->|----GATE--->mctl_ahb_config
                               |                       |----GATE--->usb_ahb
                               |                       |----GATE--->sd_slv_ahb
                               |
                               |-DIV3->noc_clk0_div3-->|----GATE--->sd0-2_ahb
                               |                       |----GATE--->emca_ahb
                               |                       |----GATE--->peridma_ahb
                               |                       |----GATE--->sysdma_ahb
                               |
                               |-DIV4->noc_clk0_div4-->|----GATE--->wdt0-2_ahb
                                                       |----GATE--->timer_ahb
                                                       |----GATE--->rtc_ahb
                                                       |----GATE--->gpio_ahb
                                                       |----GATE--->iomux_ahb
                                                       |----GATE--->i2c0-6_ahb
                                                       |----GATE--->pwm_ahb
                                                       |----GATE--->mailbox_ahb
                                                       |----GATE--->vad_ahb

     1. noc_clk0_mux_div                        --> MUX&DIV&DIV1 noc_clk0(div1)
     2. noc_clk0_div2                           --> noc_clk0 div2
     3. noc_clk0_div2                           --> noc_clk0 div3
     4. noc_clk0_div4                           --> noc_clk0 div4

     5. peri_dma_axi_gate                       --> peri dma axi gate,              clk source noc_clk0_mux_div.
     6. sys_dma_axi_gate                        --> peri dma axi gate,              clk source noc_clk0_mux_div.
     7. sram0_axi_gate                          --> sram0 axi gate,                 clk source noc_clk0_mux_div.
     8. sram1_axi_gate                          --> sram1 axi gate,                 clk source noc_clk0_mux_div.
     9. noc_clk0_noc_axi_mctl_p3_gate           --> noc clock0 & mctl_p3 gate,      clk source noc_clk0_mux_div.

     10.mctl_noc_ahb_mctl_gate                  --> noc-ahb-mctl config bus gate,   clk source noc_clk0_div2.
     11.usb_ahb_gate                            --> noc-ahb-usb config bus gate,    clk source noc_clk0_div2.
     12.sd_slv_ahb_gate                         --> noc-ahb-sd-slv config bus gate, clk source noc_clk0_div2.

     13.sd0_ahb_gate                            --> noc-ahb-sd0 config bus gate,    clk source noc_clk0_div3.
     14.sd1_ahb_gate                            --> noc-ahb-sd1 config bus gate,    clk source noc_clk0_div3.
     15.sd2_ahb_gate                            --> noc-ahb-sd2 config bus gate,    clk source noc_clk0_div3.
     15.emac_ahb_gate                           --> noc-ahb-emac config bus gate,   clk source noc_clk0_div3.
     16.peri_dma_ahb_gate                       --> noc-ahb-peridma config bus gate,clk source noc_clk0_div3.
     17.sys_dma_ahb_gate                        --> noc-ahb-sysdma config bus gate, clk source noc_clk0_div3.

     18.wdt0_ahb_gate                           --> noc-apb-wdt0 config bus gate,   clk source noc_clk0_div4.
     19.wdt1_ahb_gate                           --> noc-apb-wdt1 config bus gate,   clk source noc_clk0_div4.
     20.wdt2_ahb_gate                           --> noc-apb-wdt2 config bus gate,   clk source noc_clk0_div4.
     21.timer_ahb_gate                          --> noc-apb-timer config bus gate,  clk source noc_clk0_div4.
     22.rtc_ahb_gate                            --> noc-apb-rtc config bus gate,    clk source noc_clk0_div4.
     23.gpio_ahb_gate                           --> noc-apb-gpio config bus gate,   clk source noc_clk0_div4.
     24.iomux_ahb_gate                          --> noc-apb-iomux config bus gate,  clk source noc_clk0_div4.
     25.i2c(0-6)_ahb_gate                       --> noc-apb-i2c config bus gate,    clk source noc_clk0_div4
     26.pwm_ahb_gate                            --> noc-apb-pwm config bus gate,    clk source noc_clk0_div4.
     27.mailbox_ahb_gate                        --> noc-apb-mailbox config bus gate,clk source noc_clk0_div4.
     28.vad_ahb_gate                            --> noc-apb-vad config bus gate,    clk source noc_clk0_div4.
    */
    SysctlClkNocClk0,           /* defualt 500MHz ---> select pll0_div_2 */
    SysctlClkNocClk0Div2,
    SysctlClkNocClk0Div3,
    SysctlClkNocClk0Div4,

    SysctlClkNocClk0PeriDmaAxi,
    SysctlClkNocClk0SysDmaAxi,
    SysctlClkNocClk0Sram0Axi,
    SysctlClkNocClk0Sram1Axi,
    SysctlClkNocClk0AxiP3,

    SysctlClkNocClk0Div2MctlAhb,
    SysctlClkNocClk0Div2UsbAhb,
    SysctlClkNocClk0Div2SdSlvAhb,

    SysctlClkNocClk0Div3Sd0Ahb,
    SysctlClkNocClk0Div3Sd1Ahb,
    SysctlClkNocClk0Div3Sd2Ahb,
    SysctlClkNocClk0Div3EmacAhb,
    SysctlClkNocClk0Div3PeriDmaApb,
    SysctlClkNocClk0Div3SysDmaApb,

    SysctlClkNocClk0Div4Wdt0Apb,
    SysctlClkNocClk0Div4Wdt1Apb,
    SysctlClkNocClk0Div4Wdt2Apb,
    SysctlClkNocClk0Div4TimerApb,
    SysctlClkNocClk0Div4RtcApb,
    SysctlClkNocClk0Div4GpioApb,
    SysctlClkNocClk0Div4IomuxApb,
    SysctlClkNocClk0Div4I2c0Apb,
    SysctlClkNocClk0Div4I2c1Apb,
    SysctlClkNocClk0Div4I2c2Apb,
    SysctlClkNocClk0Div4I2c3Apb,
    SysctlClkNocClk0Div4I2c4Apb,
    SysctlClkNocClk0Div4I2c5Apb,
    SysctlClkNocClk0Div4I2c6Apb,
    SysctlClkNocClk0Div4PwmApb,
    SysctlClkNocClk0Div4MailboxApb,
    SysctlClkNocClk0Div4VadApb,


    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************NOC1*********************************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* noc_clk1 tree
      pll0_div2---->DIV-------->|----GATE--->noc_clk1_axi_mctl_p2_clk_gate
                                |----GATE--->h264_axi_gate
                                |
                                |----noc_peri_apb_div-->|----GATE--->uart(0-3)_apb_gate
                                |                       |----GATE--->i2s_s_apb_gate
                                |                       |----GATE--->audio_apb_gate
                                |
                                |----noc_peri_ahb_div-->|----GATE--->spi_0-3_ahb_gate
                                |
                                |----GATE--->vo_axi_gate
                                |----GATE--->twod_axi_gate
                                |----GATE--->mfbc_axi_gate
                                |----GATE--->vi_axi_gate
                                |----GATE--->isp_f2k_axi_gate
                                |----GATE--->isp_r2k_axi_gate
                                |----GATE--->isp_tof_axi_gate
                                |
                                |
                                |---noc_clk1_div4-->|----GATE--->csi_0-3_apb_gate
                                                    |----GATE--->isp_f2k_apb_gate
                                                    |----GATE--->isp_r2k_apb_gate
                                                    |----GATE--->isp_tof_apb_gate
                                                    |----GATE--->mfbc_apb_gate
                                                    |----GATE--->mipi_corner_apb_gate
                                                    |----GATE--->vi_apb_gate
    */
    SysctlClkNocClk1,           /* defualt 500MHz ---> select pll0_div_2 */
    SysctlClkNocClk1AxiMctl,
    SysctlClkNocClk1H264Axi,
    SysctlClkNocClk1VoAxi,
    SysctlClkNocClk1TwodAxi,
    SysctlClkNocClk1MfbcAxi,
    SysctlClkNocClk1ViAxi,
    SysctlClkNocClk1IspF2kAxi,
    SysctlClkNocClk1IspR2kAxi,
    SysctlClkNocClk1IspTofAxi,

    SysctlClkNocClk1PeriApb,
    SysctlClkNocClk1PeriApbUart0Apb,
    SysctlClkNocClk1PeriApbUart1Apb,
    SysctlClkNocClk1PeriApbUart2Apb,
    SysctlClkNocClk1PeriApbUart3Apb,
    SysctlClkNocClk1PeriApbI2sSApb,
    SysctlClkNocClk1PeriApbAudioApb,

    SysctlClkNocClk1PeriAhb,
    SysctlClkNocClk1PeriAhbSpi0Ahb,
    SysctlClkNocClk1PeriAhbSpi1Ahb,
    SysctlClkNocClk1PeriAhbSpi2Ahb,
    SysctlClkNocClk1PeriAhbSpi3Ahb,

    SysctlClkNocClk1Div4,
    SysctlClkNocClk1Csi0Apb,
    SysctlClkNocClk1Csi1Apb,
    SysctlClkNocClk1Csi2Apb,
    SysctlClkNocClk1Csi3Apb,
    SysctlClkNocClk1F2kApb,
    SysctlClkNocClk1R2kApb,
    SysctlClkNocClk1TofApb,
    SysctlClkNocClk1MfbcApb,
    SysctlClkNocClk1MipiCornerApb,
    SysctlClkNocClk1ViApb,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************DISPLAY*********************************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* display system&apb clk tree

      pll1_div4--------->DIV--> display system clk---->|----GATE--->dsi_apb_clk
                                                       |----GATE--->dsi_system_clk
                                                       |----GATE--->vo_apb_clk
                                                       |----GATE--->twod_apb_clk
                                                       |----GATE--->bt1120_apb_clk

     1. display_sys_and_apb_clk_div             --> DIV
     2. dsi_apb_clk_gate                        --> dsi apb clk gate
     3. dsi_system_clk_gate                     --> dsi system clk gate
     4. vo_apb_clk_gate                         --> video out apb clk gate
     5. twod_apb_clk_gate
     6. bt1120_apb_clk_gate
    */
    SysctlClkDispSysAndApbClkDiv,
    SysctlClkDispSysAndApbClkDivDsiApb,
    SysctlClkDispSysAndApbClkDivDsiSystem,
    SysctlClkDispSysAndApbClkDivVoApb,
    SysctlClkDispSysAndApbClkDivTwodApb,
    SysctlClkDispSysAndApbClkDivBt1120Apb,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************CSI SYS CLK**************************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* csi0/1 system clk tree

      pll1_div4--------->DIV--> csi0 system clk
      pll1_div4--------->DIV--> csi1 system clk

    */
    SysctlClkCsi0System,
    SysctlClkCsi1System,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************CSI0 PIXEL***************************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* csi0 clock tree
                    +-+
                    |M \
      pll2_div4---->|U |-->DIV-->|---GATE--->csi0_pixel_clk
                    |X /
                    +-+

     1. csi0_pixel_clk                          --> DIV&GATE (only one parent)
    */
    SysctlClkCsi0Pixel,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************CSI1 PIXEL***************************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* csi1 clock tree
                    +-+
      pll2_div4---->|M \
      pll2_div3---->|U |-->DIV-->|---GATE--->csi1_pixel_clk
      pll0     ---->|X /
      pll1     ---->+-+

     1. csi1_pixel_clk                          --> MUX&DIV&GATE
    */
    SysctlClkCsi1Pixel,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************TPG PIXEL****************************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* tpg clock tree
                    +-+
                    |M \
      pll2_div4---->|U |-->DIV-->|---GATE--->tpg_pixel_clk
                    |X /
                    +-+

     1. tpg_pixel_clk                          --> DIV&GATE (only one parent)
    */
    SysctlClkTpgPixel,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************DISPLAY PIXEL************************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* display pixel clock tree
                    +-+
      pll2_div4---->|M \
      pll2_div3---->|U |-->DIV-->|---GATE--->display_pixel_clk
      pll0     ---->|X /
      pll1     ---->+-+

     1. display_pixel_clk                       --> MUX&DIV&GATE
    */
    SysctlClkDisplayPixel,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************AUDIO OUT SERIAL*********************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* audio out serial clock tree
      pll0_div0---->DIV---->GATE--->----INVERT--->audio clk out
    */
    SysctlClkAudioOutSerial,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************AUDIO IN SERIAL CLK******************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* audio in serial clock tree
      pll0_div0---->DIV---->GATE--->----INVERT--->audio clk in
    */
    SysctlClkAudioInSerial,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************SDIO MASTER(system clock)************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* audio in serial clock tree
      pll1_div3---->DIV---->GATE--->----INVERT--->audio clk in
    */
    /* sd master clk fixed div 5
       pll0 --> fixed div 5--> sd master clk --> gate --> sd_0_master_clk
                                             --> gate --> sd_1_master_clk
                                             --> gate --> sd_2_master_clk
    */
    SysctlClkSdMaster,
    SysctlClkSdMasterSd0,
    SysctlClkSdMasterSd1,
    SysctlClkSdMasterSd2,


    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************MIPI CLOCK***************************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* mipi clock tree
                    +-+
      pll2_div2---->|M \
      pll1     ---->|U |-->DIV-->|--->GATE--->txphy_ref_clock
                    |X /         |--->GATE--->rxphy_ref_clock
                    +-+

     osc25m_gate--------------------->GATE--->txphy_pll_clock
   */
    SysctlClkMipiRef,
    SysctlClkMipiRefTxphyRef,
    SysctlClkMipiRefRxphyRef,
    SysctlClkMipiRefTxphyPll,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************SECURITY AHB CLOCK*******************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* security ahb (for config) clock tree
      pll1_div3---->DIV---->GATE--->|----GATE--->otp_ahb
                                    |----GATE--->rom_ahb
                                    |----GATE--->sha_ahb
                                    |----GATE--->aes_ahb
                                    |----GATE--->rsa_ahb
                                    |----GATE--->puf_ahb
    */
    /* secure ahb clock register addr:0x94*/
    SysctlClkSecHclk,        /* default 222MHz ---> pll_div_3(444MHz)  div 2 */
    SysctlClkPufHclk,        /* default 222MHz ---> pll_div_3(444MHz)  div 2 */
    SysctlClkOtpHclk,        /* default 222MHz ---> pll_div_3(444MHz)  div 2 */
    SysctlClkRomHclk,        /* default 222MHz ---> pll_div_3(444MHz)  div 2 */
    SysctlClkRsaHclk,        /* default 222MHz ---> pll_div_3(444MHz)  div 2 */
    SysctlClkAesHclk,        /* default 222MHz ---> pll_div_3(444MHz)  div 2 */
    SysctlClkShaHclk,        /* default 222MHz ---> pll_div_3(444MHz)  div 2 */
    /* OTP work(system) clock register addr:0x98*/

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************DDR controller core CLOCK************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* ddr controller core clock tree
                    +-+
      pll1--------->|M \
      pll2_div2---->|U |-->DIV-->|---GATE--->mctl_ddrc_clk
      pll3_div3---->|X /
      pll3_div4---->+-+
    */
    SysctlClkDdrControllerCore,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************EMAC loopback clock******************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* emac loopback clk tree
       pll0_div4 --> DIV--> gate --> emac_loopback
    */
    SysctlClkEmacLoopback,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************UART system clock********************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* uart0-3 system clock tree
                    +-+
      osc25m_gate-->|M \
      pll1_div4---->|U |-->DIV-->|---GATE--->uart_X_system_clock
                    |X /
                    +-+
    */
    /* uart0 work(system) clock register addr:0x5c*/
    SysctlClkUart0Sclk,          /* defalut 25MHz --->OSC25M div 1 */
    /* uart1 work(system) clock register addr:0x60*/
    SysctlClkUart1Sclk,          /* defalut 25MHz --->OSC25M div 1 */
    /* uart2 work(system) clock register addr:0x64*/
    SysctlClkUart2Sclk,          /* defalut 25MHz --->OSC25M div 1 */
    /* uart3 work(system) clock register addr:0x68*/
    SysctlClkUart3Sclk,          /* defalut 25MHz --->OSC25M div 1 */


    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************SPI system clock*********************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* spi0-2 system clock tree
                    +-+
      osc25m_gate-->|M \
      pll0       -->|U |-->DIV-->|---GATE--->spi_0.1.3_system_clock
                    |X /
                    +-+
    */
    /* spi3 system clock tree
       osc25m_gate----> DIV ----> GATE --->spi_3_system_clock
    */
    /* spi0 work(system) clock register addr:0x74*/
    SysctlClkSpi0Sclk,           /* default 3.125MHz--->OSC25M div 8*/
    SysctlClkSpi1Sclk,
    SysctlClkSpi2Sclk,
    SysctlClkSpi3Sclk,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************OTP system clock*********************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* otp system clock tree
       osc25m_gate----> fixed div(2) ----> GATE --->otp_system_clock
    */
    SysctlClkOtpSclk,        /* default 12.5MHz --> OSC25M div 2(hardware fixed)*/

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************I2C system clock*********************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* i2c0-6 sclk tree
                    +-+
      pll0_div4  -->|M \
      osc25m_gate-->|U |-->DIV-->|---GATE--->i2c_0_sclk
                    |X /
                    +-+
    */
    SysctlClkI2c0Sclk,
    SysctlClkI2c1Sclk,
    SysctlClkI2c2Sclk,
    SysctlClkI2c3Sclk,
    SysctlClkI2c4Sclk,
    SysctlClkI2c5Sclk,
    SysctlClkI2c6Sclk,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************WDT system clock*********************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* wdt0 sclk tree
      osc25m_gate--------->DIV----->GATE--->wdt_0_sclk
    */
    /* wdt0 work(tick) clock register addr:0xd4*/
    SysctlClkWdt0Tclk,       /* default 0.757M ----> OSC25M div 33 */
    /* wdt1 work(tick) clock register addr:0xd8*/
    SysctlClkWdt1Tclk,       /* default 0.757M ----> OSC25M div 33 */
    /* wdt2 work(tick) clock register addr:0xdc*/
    SysctlClkWdt2Tclk,       /* default 0.757M ----> OSC25M div 33 */

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************VAD system clock*********************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* vad sclk tree
       osc25m_gate------>DIV------>GATE----->vad seiral(sample) clock ----> phase(invert or not)---> vad sclk
    */
    SysctlClkVadTclk,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************timer system clock*******************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* timer x system clock tree
                    +-+
      osc25m_gate-->|M \
      osc32k     -->|U |--->DIV-->|---GATE(all timer)--->timer_x_tick_clk(system clock)
                    |X /
                    +-+
    */
    SysctlClkTimer0Tclk,
    SysctlClkTimer1Tclk,
    SysctlClkTimer2Tclk,
    SysctlClkTimer3Tclk,
    SysctlClkTimer4Tclk,
    SysctlClkTimer5Tclk,

    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /********************************************usb clock****************************************************/
    /*********************************************************************************************************/
    /*********************************************************************************************************/
    /* usb clock tree
        osc25m_gate---->|--->GATE----->usb phy core clk

        osc32k          |--->GATE----->usb wakeup clk
    */
    SysctlClkUsbPhyCore,
    SysctlClkUsbWakeup,

    /* 孤立节点 输入时钟使能 */
    SysctlClkI2ssSclk,

    SysctlClkNodeMax,
}