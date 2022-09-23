// use core::slice::SlicePattern;
use core::time;
// use std::sync::Mutex;
// use core::slice;
use std::thread;
use std::ffi::CString;
// use std::sync;

use windows::Win32::Foundation::{HANDLE, BOOL, CloseHandle, HWND, GetLastError};
use windows::Win32::Storage::FileSystem::{ReadFile, WriteFile, PIPE_ACCESS_DUPLEX, FILE_FLAG_OVERLAPPED, WriteFileEx};
use windows::Win32::System::IO::OVERLAPPED;
use windows::Win32::System::Pipes::{PeekNamedPipe, CreateNamedPipeA, CreateNamedPipeW, PIPE_TYPE_MESSAGE, PIPE_READMODE_MESSAGE, PIPE_WAIT, ConnectNamedPipe};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, IsWindowVisible, DestroyWindow};
use windows::s;
use windows::Win32::Foundation::WIN32_ERROR;
use windows::core::{PCSTR, PCWSTR};
use crate::communication::message::Message;
use crate::communication::message::{RLGYM_NULL_MESSAGE_HEADER, RLGYM_NULL_MESSAGE_BODY};

use super::message::deserialize_header;

pub const RLGYM_GLOBAL_PIPE_ID: &str = "RLGYM_GLOBAL_COMM_PIPE";
pub const RLGYM_GLOBAL_PIPE_NAME: &str = r"\\.\pipe\RLGYM_GLOBAL_COMM_PIPE";
pub const RLGYM_DEFAULT_PIPE_SIZE: usize = 1400;
pub const RLGYM_DEFAULT_TIMEOUT: usize = 4000;

#[derive(Default)]
pub struct CommunicationHandler {
    _current_pipe_name: String,
    _pipe: HANDLE,
    _connected: bool,
    message: Message,
    overlapped_struct: OVERLAPPED
}

impl CommunicationHandler {
    pub fn new() -> Self {
        CommunicationHandler {
            _current_pipe_name: RLGYM_GLOBAL_PIPE_NAME.to_string(),
            message: Message::new(),
            _connected: false,
            overlapped_struct: OVERLAPPED::default(),
            ..Default::default()
        }
    }

    pub fn receive_message(&self, header: Option<Vec<f32>>) -> Message {
        if !self._connected {
            panic!("RLGYM ATTEMPTED TO RECEIVE MESSAGE WITH NO CONNECTION") 
        }
        let header = match header {
            Some(header) => header,
            None => Vec::<f32>::new()
        };
        // let received_message = self.message;
        let mut received_message = self.message.clone();
        for i in 0..10 {
            let mut buffer = [0 as u8];
            let mut out: BOOL;
            unsafe {
                out = ReadFile(self._pipe, Some(&mut buffer), None, None);
            }
            // let bytes = out.0 as u32;
            // let decode_str =
            // let msg_floats = Vec::<f64>::new();
            let msg_floats = bytes_to_f32(&buffer);
            let deserialized_header = deserialize_header(&msg_floats);

            if header.len() == 0 || header == deserialized_header {
                received_message.deserialize(msg_floats);
                unsafe {
                    let out: BOOL;
                    let mut buffer = [0 as u8];
                    out = PeekNamedPipe(self._pipe, Some(&mut buffer), None, None, None);
                    if buffer[0] == 0 {
                        break
                    }
                }
                
            }
        }
        return received_message
    }

    pub fn send_message(&mut self, message: Option<Message>, header: Option<Vec<f32>>, body: Option<Vec<f32>>) {
        let mut message = match message {
            Some(message) => message,
            None => Message::new()
        };
        let header = match header {
            Some(header) => header,
            None => RLGYM_NULL_MESSAGE_HEADER.to_vec()
        };
        let body = match body {
            Some(body) => body,
            None => RLGYM_NULL_MESSAGE_BODY.to_vec()
        };
        message.set_body_header_vals(header, body);
        let serialized = message.serialize();
        let printable = serialized.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
        println!("message being sent: {printable}");
        // format!("{:02x}", 8 as u8);
        let u8_serialized = f32vec_as_u8_slice(&serialized);
        let printable = u8_serialized.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
        println!("message being sent in bytes: {:x?}", printable);
        let out: BOOL;
        unsafe {
            out = WriteFile(self._pipe, Some(u8_serialized.as_slice()), None, None);
        }
        let res_bool = out.as_bool();
        println!("send_message WriteFile result: {res_bool}");
        // let err;
        // unsafe {
        //     err = GetLastError().0;
        // }
        if !res_bool {
            let err;
            unsafe {
                err = WIN32_ERROR {
                    0: GetLastError().0
                };
            }
            let err = err.to_hresult();
            let err = err.message();
            println!("WriteFile error: {err}"); 
        }
    }

    pub fn open_pipe(&mut self, pipe_name: Option<&String>, num_allowed_instances: Option<usize>) {
        let pipe_name = match pipe_name {
            Some(pipe_name) => pipe_name,
            None => RLGYM_GLOBAL_PIPE_NAME
        };
        let num_allowed_instances = match num_allowed_instances {
            Some(num) => num,
            None => 1
        };

        if self._connected {
            self.close_pipe()
        }

        self._connected = false;

        let mut _connected = false;

        let immu_connected = _connected;

        let handler = thread::spawn(move || {
            let mut i = 0;
            while !immu_connected {
                let win_handle: HWND;
                let is_visible: BOOL;
                unsafe{
                    win_handle = FindWindowA(None, s!("DIEmWin"));
                    is_visible = IsWindowVisible(win_handle);
                    if is_visible.as_bool() {
                        let res = DestroyWindow(win_handle);
                        println!("DIEmWin detector successfully closed window");
                    }
                }
                thread::sleep(time::Duration::new(1, 0));
                i += 1;
                if i > 6 {
                    break
                }
            }
        });
        let pipe_name_u16: Vec<u16> = pipe_name.encode_utf16().collect();
        // let pipe_name_u16: = pipe_name.encode_utf16().collect();
        let out;
        let c_str = CString::new(pipe_name).expect("CString::new failed");
        let pcstr = PCSTR::from_raw(c_str.as_bytes_with_nul().as_ptr());
        let pcwstr = PCWSTR::from_raw(pipe_name_u16.as_ptr());
        let pcstr_str;
        unsafe {
            out = CreateNamedPipeA(pcstr,
                 PIPE_ACCESS_DUPLEX,
                  PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
                   num_allowed_instances as u32,
                    RLGYM_DEFAULT_PIPE_SIZE as u32,
                    RLGYM_DEFAULT_PIPE_SIZE as u32,
                      0,
                       None);
            pcstr_str = pcstr.display();
        }
        // let pipe_name_joinedstr: String = pipe_name_u16.join(" ");
        // println!("");
        println!("{pcstr_str}");
        // match pcstr_str {
        //     Ok(some) => println!("to_string -> {some}"),
        //     Err(some) => println!("to_string error: {some}")
        // }


        match out {
            Ok(out) => self._pipe = out,
            Err(err) => panic!("CreateNamedPipeA Err: {err}")
        };
        let print = self._pipe.0;
        println!("NamedPipe handle: {print}");
        
        let out;
        unsafe {
            out = ConnectNamedPipe(self._pipe, None);
        }
        if !out.as_bool() {
            let out = out.0;
            println!("error connecting to named pipe: {out}");
            let err;
            unsafe {
                err = WIN32_ERROR {
                    0: GetLastError().0
                };
            }
            let err = err.to_hresult();
            let err = err.message();
            println!("NamedPipe error: {err}");
        }
        pipe_name_u16;

        self._connected = true;
        _connected = true;

        let res = handler.join();

    }

    pub fn close_pipe(&mut self) {
        self._connected = false;
        unsafe {
            let out = CloseHandle(self._pipe);
        }
    }

    pub fn is_connected(&self) -> bool {
        return self._connected.clone()
    }
}

pub fn format_pipe_id(pipe_id: usize) -> String {
    return r"\\.\pipe\!".replace("!", &pipe_id.to_string())
}

pub fn bytes_to_f32(bytes: &[u8]) -> Vec<f32> {
    // let bytes_len = bytes.len();
    let mut float_vec = Vec::<f32>::new();
    let bytes_vec = bytes.to_vec();
    for i in (0..bytes.len()).step_by(4) {
        // let mut slice = [0 as u8; 4];
        let slice = bytes_vec[i..i+4].try_into().unwrap();
        float_vec.push(f32::from_ne_bytes(slice) as f32);
    }
    return float_vec
}

// pub fn f32_to_bytes(f32_vec: &Vec<f32>) -> Vec<u8> {
//     // let mut ret_bytes = [0 as u8; 0];
//     // let mut bytes_arr = f32_vec.as_slice();
//     // for num in bytes_arr {
//     //     ret_bytes.
//     // }
//     let u8_vec: Vec<u8> = f32_vec.iter().map(|x| *x as u8).collect();
//     return u8_vec
// }

pub fn f32vec_as_u8_slice(v: &[f32]) -> Vec<u8> {
    let mut u8_vec = Vec::<u8>::new();
    for val in v {
        u8_vec.extend_from_slice(&mut val.to_ne_bytes())
    }
    u8_vec
}

// pub fn f32vec_as_u8_slice(v: &[f32]) -> &[u8] {
//     unsafe {
//         std::slice::from_raw_parts(
//             v.as_ptr() as *const u8,
//             v.len() * std::mem::size_of::<f32>(),
//         )
//     }
// }

// pub fn f32_vec_as_u8_slice(v: &[f32]) -> &[u8] {
//     let res = Vec::<u8>::new();

//     for f in v {
//         let bits = f.to_ne_bytes();

//     }
// }

// pub fn handle_diemwin_potential(connected: &sync::Mutex<bool>) {
//     while !*connected.lock().unwrap() {
//         let win_handle: HWND;
//         let is_visible: BOOL;
//         unsafe{
//             win_handle = FindWindowA(None, s!("DIEmWin"));
//             is_visible = IsWindowVisible(win_handle);
//             if is_visible.as_bool() {
//                 DestroyWindow(win_handle);
//             }
//         }
//         println!("DIEmWin detector successfully closed window");
//     }

// }
