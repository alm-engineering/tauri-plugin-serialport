use serde::Serialize;
use serialport::{self, SerialPort};
use std::{
    collections::HashMap,
    sync::{mpsc::Sender, Arc, Mutex},
};

#[derive(Default)]
pub struct SerialPortState {
    pub serialports: Arc<Mutex<HashMap<String, SerialPortInfo>>>,
}

pub struct SerialPortInfo {
    pub serialport: Box<dyn SerialPort>,
    pub sender: Option<Sender<usize>>,
}

#[derive(Serialize, Clone)]
pub struct InvokeResult {
    pub code: i32,
    pub message: String,
}

#[derive(Serialize, Clone)]
pub struct ReadData<'a> {
    pub data: &'a [u8],
    pub size: usize,
}