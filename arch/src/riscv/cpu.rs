//! CPU information.
use crate::utils::init_once::InitOnce;

pub(super) static CPU_FREQ_MHZ: InitOnce<u16> = InitOnce::new_with_default(1000); // 1GHz

hal_fn_impl! {
    impl mod crate::hal_fn::cpu {
        fn cpu_id() -> u8 {
            let mut cpu_id;
            unsafe { core::arch::asm!("mv {0}, tp", out(reg) cpu_id) };
            cpu_id
        }

        fn cpu_frequency() -> u16 {
            *CPU_FREQ_MHZ
        }

        fn reset() -> ! {
            info!("shutdown...");
            sbi_rt::system_reset(sbi_rt::Shutdown, sbi_rt::NoReason);
            unreachable!()
        }
    }
}
