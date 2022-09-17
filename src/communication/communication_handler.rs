

pub fn format_pipe_id(pipe_id: usize) -> String {
    return r"\\.\pipe\!".replace("!", &pipe_id.to_string())
}