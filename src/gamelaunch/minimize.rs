use windows::Win32::Foundation::{HWND, LPARAM, BOOL};
use windows::Win32::UI::WindowsAndMessaging::{GetWindowTextA, EnumWindows, WNDENUMPROC, IsWindowVisible, ShowWindow, SW_FORCEMINIMIZE};


pub fn toggle_rl_windows(minimize: bool) {
    if minimize {

        unsafe extern "system" fn win_enum_handler(hwnd: HWND, param1: LPARAM)  -> BOOL {
            let window_visible: bool;
            window_visible = IsWindowVisible(hwnd).as_bool();

            if window_visible {
                let mut window_u8: Vec<u8> = vec![0; 500];
                let out = GetWindowTextA(hwnd, window_u8.as_mut_slice());
                window_u8.truncate(out as usize);
                let window_text = String::from_utf8(window_u8.clone());
                let window_text = match window_text {
                    Ok(window_text) => window_text,
                    Err(err) => {println!("FromUtf8Error: {err}");
                    let window_u8_str = window_u8.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "); 
                    println!("u8 line: {window_u8_str}");
                    String::new()}
                };
                let out;
                if &window_text == "Rocket League" {
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