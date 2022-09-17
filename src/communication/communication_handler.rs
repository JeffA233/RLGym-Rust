use windows::Win32::Foundation::HANDLE;

pub struct CommunicationHandler {
    RLGYM_GLOBAL_PIPE_ID: String,
    RLGYM_GLOBAL_PIPE_NAME: String,
    RLGYM_DEAFULT_PIPE_SIZE: usize,
    RLGYM_DEFAULT_TIMEOUT: usize,
    _current_pipe_name: String,
    _pipe: HANDLE,
    _connected: bool,
    _message: Message
}

pub struct Message {

}

// impl CommunicationHandler {
//     pub fn new() -> Self {

//     }
// }

pub fn format_pipe_id(pipe_id: usize) -> String {
    return r"\\.\pipe\!".replace("!", &pipe_id.to_string())
}