// use std::sync::Mutex;
// use core::slice;
use std::thread;
// use std::sync;

use windows::Win32::Foundation::{HANDLE, BOOL, CloseHandle, HWND};
use windows::Win32::Storage::FileSystem::{ReadFile, WriteFile, PIPE_ACCESS_DUPLEX, FILE_FLAG_OVERLAPPED};
// use windows::Win32::System::IO::OVERLAPPED;
use windows::Win32::System::Pipes::{PeekNamedPipe, CreateNamedPipeW, PIPE_TYPE_MESSAGE, PIPE_READMODE_MESSAGE, PIPE_WAIT};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, IsWindowVisible, DestroyWindow};
use windows::s;
use windows::core::PCWSTR;
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
    message: Message
}

impl CommunicationHandler {
    pub fn new() -> Self {
        CommunicationHandler {
            _current_pipe_name: RLGYM_GLOBAL_PIPE_NAME.to_string(),
            message: Message::new(),
            _connected: false,
            ..Default::default()
        }
    }

    pub fn recieve_message(self, header: Option<Vec<f32>>) -> Message {
        if !self._connected {
            panic!("RLGYM ATTEMPTED TO RECEIVE MESSAGE WITH NO CONNECTION") 
        }
        let header = match header {
            Some(header) => header,
            None => Vec::<f32>::new()
        };
        // let received_message = self.message;
        let mut received_message = self.message;
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

    pub fn send_message(self, message: Option<Message>, header: Option<Vec<f32>>, body: Option<Vec<f32>>) {
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

        let u8_serialized = f32vec_as_u8_slice(&serialized);

        let out: BOOL;
        unsafe {
            out = WriteFile(self._pipe, Some(u8_serialized), None, None);
        }
    }

    pub fn open_pipe(&mut self, pipe_name: Option<String>, num_allowed_instances: Option<usize>) {
        let pipe_name = match pipe_name {
            Some(pipe_name) => pipe_name,
            None => RLGYM_GLOBAL_PIPE_NAME.to_owned()
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
            while !immu_connected {
                let win_handle: HWND;
                let is_visible: BOOL;
                unsafe{
                    win_handle = FindWindowA(None, s!("DIEmWin"));
                    is_visible = IsWindowVisible(win_handle);
                    if is_visible.as_bool() {
                        DestroyWindow(win_handle);
                    }
                }
                println!("DIEmWin detector successfully closed window");
            }
        });
        let pipe_name_u16: Vec<u16> = pipe_name.encode_utf16().collect();
        unsafe {
            self._pipe = CreateNamedPipeW(PCWSTR::from_raw(pipe_name_u16.as_ptr()),
                 PIPE_ACCESS_DUPLEX | FILE_FLAG_OVERLAPPED,
                  PIPE_TYPE_MESSAGE | PIPE_READMODE_MESSAGE | PIPE_WAIT,
                   num_allowed_instances as u32,
                    RLGYM_DEFAULT_PIPE_SIZE as u32,
                    RLGYM_DEFAULT_PIPE_SIZE as u32,
                      0,
                       None);
        }

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
    for i in 0..bytes.len() {
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

pub fn f32vec_as_u8_slice(v: &[f32]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            v.as_ptr() as *const u8,
            v.len() * std::mem::size_of::<f32>(),
        )
    }
}

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
