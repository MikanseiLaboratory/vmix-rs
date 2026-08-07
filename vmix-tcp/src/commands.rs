use crate::acts::ActivatorsData;
use crate::commands::Status::Length;
use std::{
    collections::HashMap,
    fmt::{self, Display},
    io::Read,
    net::TcpStream,
    time::Duration,
};

pub type InputNumber = u16; // 0~1000

#[derive(Debug)]
pub enum Status {
    OK,             // "OK"
    ER,             // "ER"
    Length(u64),    // Length of body
    Detail(String), // detail data
}

impl From<String> for Status {
    fn from(value: String) -> Self {
        let value = value.as_str();
        match value {
            "OK" => Self::OK,
            "ER" => Self::ER,
            _ => {
                if let Ok(length) = value.parse::<u64>() {
                    Length(length)
                } else {
                    Self::Detail(value.to_string())
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct TallyResponse {
    pub status: Status,
    pub body: HashMap<InputNumber, TallyData>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TallyData {
    OFF,
    PROGRAM,
    PREVIEW,
}
impl From<char> for TallyData {
    fn from(value: char) -> Self {
        match value {
            '0' => TallyData::OFF,
            '1' => TallyData::PROGRAM,
            '2' => TallyData::PREVIEW,
            _ => TallyData::OFF, // NO MATCHING PATTERN
        }
    }
}

#[derive(Debug)]
pub struct FunctionResponse {
    pub status: Status,
    pub body: Option<String>,
}

#[derive(Debug)]
pub struct XMLResponse {
    pub status: Status,
    pub body: String, // Change to String for raw XML content
}
#[derive(Debug)]
pub struct XMLTextResponse {
    pub status: Status,
    pub body: Option<String>,
}
#[derive(Debug)]
pub struct SubscribeResponse {
    pub status: Status,
    pub body: Option<String>,
}
#[derive(Debug)]
pub struct UnsubscribeResponse {
    pub status: Status,
    pub body: Option<String>,
}
#[derive(Debug)]
pub struct VersionResponse {
    pub status: Status,
    pub version: Option<String>, // TODO: parse semver
}

#[derive(Debug)]
pub struct ActivatorsResponse {
    pub status: Status,
    pub body: ActivatorsData,
}

#[derive(Debug)]
pub enum RecvCommand {
    TALLY(TallyResponse),
    FUNCTION(FunctionResponse),
    ACTS(ActivatorsResponse),
    XML(XMLResponse),
    XMLTEXT(XMLTextResponse),
    SUBSCRIBE(SubscribeResponse),
    UNSUBSCRIBE(UnsubscribeResponse),
    QUIT,
    VERSION(VersionResponse),
}

pub enum SendCommand {
    TALLY,
    FUNCTION(String, Option<String>),
    ACTS(String, Option<usize>),
    XML,
    XMLTEXT(String),
    SUBSCRIBE(SUBSCRIBECommand),
    UNSUBSCRIBE(SUBSCRIBECommand),
    QUIT,
    VERSION,

    RAW(String),
}

// SendCommandとRecvCommandはマルチスレッド環境で安全に使用できる
unsafe impl Send for SendCommand {}
unsafe impl Sync for SendCommand {}

unsafe impl Send for RecvCommand {}
unsafe impl Sync for RecvCommand {}

// 関連する構造体とenumにもSend + Syncを実装
unsafe impl Send for TallyData {}
unsafe impl Sync for TallyData {}

unsafe impl Send for TallyResponse {}
unsafe impl Sync for TallyResponse {}

unsafe impl Send for FunctionResponse {}
unsafe impl Sync for FunctionResponse {}

unsafe impl Send for XMLResponse {}
unsafe impl Sync for XMLResponse {}

unsafe impl Send for XMLTextResponse {}
unsafe impl Sync for XMLTextResponse {}

unsafe impl Send for SubscribeResponse {}
unsafe impl Sync for SubscribeResponse {}

unsafe impl Send for UnsubscribeResponse {}
unsafe impl Sync for UnsubscribeResponse {}

unsafe impl Send for VersionResponse {}
unsafe impl Sync for VersionResponse {}

unsafe impl Send for ActivatorsResponse {}
unsafe impl Sync for ActivatorsResponse {}

unsafe impl Send for SUBSCRIBECommand {}
unsafe impl Sync for SUBSCRIBECommand {}

unsafe impl Send for Status {}
unsafe impl Sync for Status {}

impl From<SendCommand> for Vec<u8> {
    fn from(command: SendCommand) -> Self {
        match command {
            SendCommand::TALLY => "TALLY\r\n".as_bytes().to_vec(),
            SendCommand::FUNCTION(func, query) => {
                format!("FUNCTION {} {}\r\n", func, query.unwrap_or("".to_string())).into_bytes()
            }
            SendCommand::ACTS(command, input) => {
                if let Some(input_num) = input {
                    format!("ACTS {} {}\r\n", command, input_num).into_bytes()
                } else {
                    format!("ACTS {}\r\n", command).into_bytes()
                }
            }
            SendCommand::XML => "XML\r\n".as_bytes().to_vec(),
            SendCommand::XMLTEXT(path) => format!("XMLTEXT {}\r\n", path).into_bytes(),
            SendCommand::SUBSCRIBE(command) => format!("SUBSCRIBE {}\r\n", command).into_bytes(),
            SendCommand::UNSUBSCRIBE(command) => {
                format!("UNSUBSCRIBE {}\r\n", command).into_bytes()
            }
            SendCommand::QUIT => "QUIT\r\n".as_bytes().to_vec(),
            SendCommand::VERSION => "VERSION\r\n".as_bytes().to_vec(),
            SendCommand::RAW(raw) => raw.into_bytes(),
        }
    }
}

pub enum SUBSCRIBECommand {
    TALLY,
    ACTS,
}

impl Display for SUBSCRIBECommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TALLY => write!(f, "TALLY"),
            Self::ACTS => write!(f, "ACTS"),
        }
    }
}

impl TryFrom<&mut TcpStream> for RecvCommand {
    type Error = anyhow::Error;

    fn try_from(stream: &mut TcpStream) -> Result<Self, Self::Error> {
        // Read directly from TcpStream to avoid BufReader buffering issues
        let mut value = String::new();
        let mut buffer = [0u8; 1];

        // Read byte by byte until we hit \n
        loop {
            let bytes_read = stream.read(&mut buffer)?;
            if bytes_read == 0 {
                return Err(anyhow::anyhow!(std::io::Error::new(
                    std::io::ErrorKind::ConnectionAborted,
                    "connection aborted"
                )));
            }

            let ch = buffer[0] as char;
            value.push(ch);

            if ch == '\n' {
                break;
            }
        }

        // remove \r\n
        let value = value.lines().collect::<String>();

        let commands: Vec<String> = value.split_whitespace().map(|s| s.to_string()).collect();

        // first element
        let command = commands
            .first()
            .ok_or_else(|| anyhow::anyhow!("Empty command"))?;
        let status: Status = commands.get(1).unwrap().to_owned().into();
        let body: Option<String> = commands.get(2).cloned();
        match command.as_str() {
            // Example Response: TALLY OK 0121...\r\n
            "TALLY" => {
                let mut tally_map = HashMap::new();
                // check if status is ok
                let chars: Vec<char> = body.unwrap().chars().collect::<Vec<char>>();
                for (i, char) in chars.iter().enumerate() {
                    let tally: TallyData = (*char).into();
                    let mut index = i as InputNumber;
                    index += 1;
                    tally_map.insert(index, tally);
                }
                Ok(Self::TALLY(TallyResponse {
                    status,
                    body: tally_map,
                }))
            }
            // Example Response: FUNCTION OK PreviewInput\r\n
            // Example Response: FUNCTION ER Error message\r\n
            "FUNCTION" => Ok(Self::FUNCTION(FunctionResponse { status, body })),
            // Example Response: ACTS OK Input 1 1\r\n
            "ACTS" => {
                // 2以降のベクターを使用する
                let len = commands.len();
                let raw = &commands.clone()[2..len];
                let body = ActivatorsData::try_from(raw)?;
                Ok(Self::ACTS(ActivatorsResponse { status, body }))
            }
            /*
            Example Response: XML 37\r\n
            <vmix><version>x.x.x.x</version></vmix>
            */
            "XML" => {
                if let Length(len) = &status {
                    // Read exact number of bytes with timeout handling
                    let mut xml_buffer = vec![0u8; *len as usize];
                    let mut bytes_read = 0;
                    let start_time = std::time::Instant::now();
                    let read_timeout = Duration::from_secs(5); // 5 second timeout for XML reads

                    while bytes_read < xml_buffer.len() {
                        match stream.read(&mut xml_buffer[bytes_read..]) {
                            Ok(0) => {
                                // EOF reached before reading all expected bytes
                                return Err(anyhow::anyhow!(std::io::Error::new(
                                    std::io::ErrorKind::ConnectionAborted,
                                    "connection aborted"
                                )));
                            }
                            Ok(n) => {
                                bytes_read += n;
                            }
                            Err(e) => match e.kind() {
                                std::io::ErrorKind::WouldBlock => {
                                    // Non-blocking read would block, check timeout
                                    if start_time.elapsed() > read_timeout {
                                        return Err(anyhow::anyhow!("XML read timeout"));
                                    }
                                    std::thread::sleep(Duration::from_millis(1));
                                    continue;
                                }
                                std::io::ErrorKind::ConnectionAborted
                                | std::io::ErrorKind::ConnectionReset
                                | std::io::ErrorKind::UnexpectedEof => {
                                    return Err(anyhow::anyhow!(e));
                                }
                                _ => return Err(anyhow::anyhow!(e)),
                            },
                        }
                    }

                    let xml = String::from_utf8(xml_buffer)?.trim_end().to_string();
                    return Ok(Self::XML(XMLResponse { status, body: xml }));
                }
                Err(anyhow::anyhow!("Failed to read XML"))
            }
            "XMLTEXT" => Ok(Self::XMLTEXT(XMLTextResponse { status, body })),
            "SUBSCRIBE" => Ok(Self::SUBSCRIBE(SubscribeResponse { status, body })),
            "UNSUBSCRIBE" => Ok(Self::UNSUBSCRIBE(UnsubscribeResponse { status, body })),
            "QUIT" => Ok(Self::QUIT), // No body
            "VERSION" => Ok(Self::VERSION(VersionResponse {
                status,
                version: body,
            })),
            _ => Err(anyhow::anyhow!("No matching command found: {:?}", command)),
        }
    }
}
