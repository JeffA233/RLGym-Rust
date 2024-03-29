

pub const RLGYM_HEADER_END_TOKEN: [f64; 3] = [13771., 83712., 83770.];
pub const RLGYM_BODY_END_TOKEN: [f64; 3] = [82772., 83273., 83774.];
pub const RLGYM_NULL_MESSAGE_HEADER: [f64; 3] = [83373., 83734., 83775.];
pub const RLGYM_NULL_MESSAGE_BODY: [f64; 3]= [83744., 83774., 83876.];
pub const RLGYM_CONFIG_MESSAGE_HEADER: [f64; 3] = [83775., 53776., 83727.];
pub const RLGYM_STATE_MESSAGE_HEADER: [f64; 3] = [63776., 83777., 83778.];
pub const RLGYM_AGENT_ACTION_MESSAGE_HEADER: [f64; 3] = [87777., 83778., 83779.];
pub const RLGYM_RESET_GAME_STATE_MESSAGE_HEADER: [f64; 3] = [83878., 83779., 83780.];
pub const RLGYM_AGENT_ACTION_IMMEDIATE_RESPONSE_MESSAGE_HEADER: [f64; 3] = [83799., 83780., 83781.];
pub const RLGYM_REQUEST_LAST_BOT_INPUT_MESSAGE_HEADER: [f64; 3] = [83781., 83781., 83682.];
pub const RLGYM_LAST_BOT_INPUT_MESSAGE_HEADER: [f64; 3] = [11781., 83782., 83983.];
pub const RLGYM_RESET_TO_SPECIFIC_GAME_STATE_MESSAGE_HEADER: [f64; 3] = [12782., 83783., 80784.];

/// Struct that formats the data from the pipe and to the pipe
#[derive(Default)]
#[derive(Clone)]
pub struct Message {
    pub body: Vec<f64>,
    pub header: Vec<f64>
}

pub fn deserialize_header(message_floats: &Vec<f64>) -> Vec<f64> {
    let header_end = _find_first(message_floats, RLGYM_HEADER_END_TOKEN.to_vec());
    return message_floats[..header_end].to_vec()
}

pub fn _find_first(list: &Vec<f64>, target: Vec<f64>) -> usize {
    let list_len = list.len();
    let target_len = target.len();
    for i in 0..list_len {
        if list[i] == target[0] && list[i..i+target_len] == target {
            return i
        }
    }
    return 0
}

pub fn _find_first_end(list: &Vec<f64>, target: Vec<f64>) -> usize {
    let list_len = list.len();
    let target_len = target.len();
    for i in list_len-4..list_len {
        if list[i] == target[0] && list[i..i+target_len] == target {
            return i
        }
    }
    panic!("target not found in message")
}

impl Message {
    pub fn new() -> Self {
        Message {
            header: RLGYM_NULL_MESSAGE_HEADER.to_vec(),
            body: RLGYM_NULL_MESSAGE_BODY.to_vec()
        }
    }
    
    pub fn set_body_header_vals(&mut self, header: Vec<f64>, body: Vec<f64>) {
        self.header = header;
        self.body = body;
    }

    pub fn serialize(&self) -> Vec<f64> {
        let mut ret_vec = Vec::<f64>::new();
        ret_vec.append(&mut self.header.clone());
        ret_vec.append(&mut RLGYM_HEADER_END_TOKEN.to_vec());
        ret_vec.append(&mut self.body.clone());
        ret_vec.append(&mut RLGYM_BODY_END_TOKEN.to_vec());
        return ret_vec
    }

    pub fn deserialize(&mut self, message_floats: Vec<f64>) {
        let header_end = _find_first(&message_floats, RLGYM_HEADER_END_TOKEN.to_vec()) + RLGYM_HEADER_END_TOKEN.len();
        let header = message_floats[..header_end].to_vec();

        let start = header_end;
        let end = _find_first_end(&message_floats, RLGYM_BODY_END_TOKEN.to_vec());
        let body = message_floats[start..end].to_vec();

        self.body = body;
        self.header = header;
    }
}
