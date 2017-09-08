#![allow(non_camel_case_types)]
#![no_std]

extern "C" {
    pub fn MX_USB_DEVICE_Init();
    pub fn MX_GPIO_Init();
    pub fn HAL_Init();
    pub fn SystemClock_Config();
    
    /* interrupt handlers */
    pub fn NMI_Handler();
    pub fn HardFault_Handler();
    pub fn MemManage_Handler();
    pub fn BusFault_Handler();
    pub fn UsageFault_Handler();
    pub fn SVC_Handler();
    pub fn DebugMon_Handler();
    pub fn PendSV_Handler();
    pub fn SysTick_Handler();
    pub fn USB_LP_CAN1_RX0_IRQHandler();
    /* send data to the host */
    pub fn USBD_CUSTOM_HID_SendReport_FS(buf: *mut u8, len: u16) -> u8;
    /* delay for some seconds */
    pub fn HAL_Delay(delay: u32);
    /* the buffer for received data */
    pub static mut hid_recv_data: [u8; 512];
    pub static mut hid_recv_len: u32;
}

pub fn nmi_handler() { unsafe { NMI_Handler(); }}
pub fn hardfault_handler() { unsafe { HardFault_Handler(); }}
pub fn mem_manage_handler() { unsafe { MemManage_Handler(); }}
pub fn bus_fault_handler() { unsafe { BusFault_Handler(); }}
pub fn svc_handler() { unsafe { SVC_Handler(); }}
pub fn pend_sv_handler() { unsafe { PendSV_Handler(); }}
pub fn systick_handler() { unsafe { SysTick_Handler(); }}
pub fn usb_lp_can1_rx0_irqhandler() { unsafe { USB_LP_CAN1_RX0_IRQHandler(); }}

pub fn hid_send(buffer: &mut [u8], length: usize) -> bool {
    unsafe {
        USBD_CUSTOM_HID_SendReport_FS(&mut buffer[0] as *mut u8, length as u16) == 0
    }
}

pub fn hal_delay(delay: u32) { unsafe { HAL_Delay(delay); }}
pub fn hal_init() { unsafe { HAL_Init(); }}

pub fn system_clock_config() { unsafe { SystemClock_Config(); }}
pub fn gpio_init() { unsafe { MX_GPIO_Init(); }}
pub fn usb_init() { unsafe { MX_USB_DEVICE_Init(); }}
