use windows::Win32::Foundation::{HWND, LPARAM, BOOL};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowTextW, EnumWindows, WNDENUMPROC, IsWindowVisible, ShowWindow, SW_FORCEMINIMIZE};


pub fn toggle_rl_windows(minimize: bool) {
    if minimize {
        unsafe extern "system" fn win_enum_handler(hwnd: HWND, param1: LPARAM)  -> BOOL {
            let window_visible: bool;
            window_visible = IsWindowVisible(hwnd).as_bool();

            if window_visible {
                let mut window_u16: Vec<u16> = vec![0; 500];
                let out = GetWindowTextW(hwnd, window_u16.as_mut_slice());
                window_u16.truncate(out as usize);
                let window_text = String::from_utf16(window_u16.clone().as_slice());
                let window_text = match window_text {
                    Ok(window_text) => window_text,
                    Err(err) => {println!("FromUtf8Error: {err}");
                    // let window_u8_str = window_u16.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "); 
                    // println!("u8 line: {window_u8_str}");
                    String::new()}
                };
                // println!("window text: {window_text}");
                let out;
                if window_text.contains("Rocket League") {
                    out = ShowWindow(hwnd, SW_FORCEMINIMIZE);
                    println!("Rocket League is now minimized");
                    return BOOL::from(true)
                } else {
                    // out = BOOL::from(true);
                    return BOOL::from(true)
                }
            } else {
                // let out = BOOL::from(true);
                return BOOL::from(true)
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