use windows::Win32::Foundation::HANDLE;
use windows::Win32::Foundation::BOOL;
use windows::Win32::Storage::FileSystem::ReadFile;
use windows::Win32::System::IO::OVERLAPPED;
use crate::communication::message::Message;

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

    pub fn recieve_message(self, header: Vec<f64>) {
        if !self._connected {
            panic!("RLGYM ATTEMPTED TO RECEIVE MESSAGE WITH NO CONNECTION") 
        }
        let received_message = self.message;
        let mut buffer = [0 as u8; RLGYM_DEFAULT_PIPE_SIZE];
        let mut out: BOOL;
        unsafe {
            out = ReadFile(self._pipe, Some(&mut buffer), None, None);
        }
        // let bytes = out.0 as u32;
        // let decode_str =
        let msg_floats = bytes_to_f32(buffer);
    }
}

pub fn format_pipe_id(pipe_id: usize) -> String {
    return r"\\.\pipe\!".replace("!", &pipe_id.to_string())
}

pub fn bytes_to_f32(bytes: [u8; RLGYM_DEFAULT_PIPE_SIZE]) -> Vec<f32> {
    for i in 0..RLGYM_DEFAULT_PIPE_SIZE {

    }
    f32::from_ne_bytes(bytes)
}