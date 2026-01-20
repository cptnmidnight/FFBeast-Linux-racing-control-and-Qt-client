use crate::virtual_key::VirtualKey;

/// Native keyboard input simulator (cross-platform)
pub struct NativeKeyboard;

impl NativeKeyboard {
    #[cfg(target_os = "windows")]
    pub fn send_key(vk: VirtualKey, press: bool) -> Result<(), String> {
        use std::mem;

        #[repr(C)]
        #[allow(non_snake_case)]
        struct KEYBDINPUT {
            #[allow(non_snake_case)]
            wVk: u16,
            #[allow(non_snake_case)]
            wScan: u16,
            #[allow(non_snake_case)]
            dwFlags: u32,
            time: u32,
            #[allow(non_snake_case)]
            dwExtraInfo: usize,
        }

        #[repr(C)]
        struct INPUT_u {
            ki: KEYBDINPUT,
        }

        #[repr(C)]
        struct INPUT {
            type_: u32,
            u: INPUT_u,
        }

        const INPUT_KEYBOARD: u32 = 1;
        const KEYEVENTF_KEYUP: u32 = 0x0002;

        let flags = if press { 0 } else { KEYEVENTF_KEYUP };

        let input = INPUT {
            type_: INPUT_KEYBOARD,
            u: INPUT_u {
                ki: KEYBDINPUT {
                    wVk: vk.to_windows_vk(),
                    wScan: 0,
                    dwFlags: flags,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };

        unsafe {
            let result = SendInput(
                1,
                &input as *const INPUT as *const u8,
                mem::size_of::<INPUT>() as i32,
            );
            if result == 0 {
                return Err("SendInput failed".to_string());
            }
        }

        Ok(())
    }

    #[cfg(target_os = "linux")]
    pub fn send_key(_vk: VirtualKey, _press: bool) -> Result<(), String> {
        // TODO: Implement using /dev/uinput
        tracing::warn!("Linux keyboard simulation not yet implemented");
        Ok(())
    }
}

#[cfg(target_os = "windows")]
#[link(name = "user32")]
extern "system" {
    fn SendInput(cInputs: u32, pInputs: *const u8, cbSize: i32) -> u32;
}
