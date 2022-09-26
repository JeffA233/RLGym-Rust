use windows::Win32::Foundation::{HWND, LPARAM, BOOL};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowTextA, EnumWindows, WNDENUMPROC, IsWindowVisible, ShowWindow, SW_FORCEMINIMIZE};


pub fn toggle_rl_windows(minimize: bool) {
    if minimize {
        // let window_ledger: HashMap<isize, String> = HashMap::new();
        // String::from_utf8(vec)

        unsafe extern "system" fn win_enum_handler(hwnd: HWND, param1: LPARAM)  -> BOOL {
            let window_visible: bool;
            window_visible = IsWindowVisible(hwnd).as_bool();

            if window_visible {
                let mut window_u8: Vec<u8> = Vec::new();
                GetWindowTextA(hwnd, window_u8.as_mut_slice());
                let window_text = String::from_utf8(window_u8).unwrap();
                let out;
                if &window_text == "Rocket League" {
                    out = ShowWindow(hwnd, SW_FORCEMINIMIZE);
                    return BOOL::from(true)
                } else {
                    out = BOOL::from(true);
                    return out
                }
            } else {
                let out = BOOL::from(true);
                return out
            }
        }
        let func: WNDENUMPROC = Some(win_enum_handler);
        unsafe {
            // let proc = WNDENUMPROC::default();
            EnumWindows(func, None);
        }
    } else {
        println!("opening all of the windows is not implemented yet")
    }
}