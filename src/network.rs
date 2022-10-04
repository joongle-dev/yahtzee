use wasm_bindgen::{closure::Closure, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    MessageEvent, RtcDataChannelEvent, RtcPeerConnection, RtcDataChannel, RtcSdpType,
    RtcSessionDescriptionInit, RtcPeerConnectionIceEvent,
};
use js_sys::Reflect;
use std::collections::HashMap;

struct Connection {
    connection: RtcPeerConnection,
    data_channel: RtcDataChannel,
}
pub struct Network {
    self_id: String,
    connections: HashMap<String, Connection>
}

impl Network {
    pub fn new(self_id: String) -> Self {
        Self { self_id: self_id, connections: Default::default() }
    }
    
    pub async fn send_sdp_offer(&mut self, target_id: String) -> Result<String, JsValue> {
        let pc = RtcPeerConnection::new()?;
        let dc = pc.create_data_channel(format!("data channel {} to {}", self.self_id, target_id).as_str());
        let offer = JsFuture::from(pc.create_offer()).await?;
        let offer_sdp = Reflect::get(&offer, &JsValue::from_str("sdp"))?.as_string().unwrap();
        
        self.connections.insert(target_id, Connection{ connection: pc, data_channel: dc });    
        Ok(offer_sdp)
    }
    pub async fn answer_sdp_offer(&mut self, sender_id: String, offer_sdp: String) -> Result<String, JsValue> {
        let pc = RtcPeerConnection::new()?;
        let dc = pc.create_data_channel(format!("data channel {} to {}", self.self_id, sender_id).as_str());
        
        let mut offer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
        offer_obj.sdp(&offer_sdp);
        
        JsFuture::from(pc.set_remote_description(&offer_obj)).await?;

        let answer = JsFuture::from(pc.create_answer()).await?;
        let answer_sdp = Reflect::get(&answer, &JsValue::from_str("sdp"))?
            .as_string()
            .unwrap();

        self.connections.insert(sender_id, Connection{ connection: pc, data_channel: dc }); 

        Ok(answer_sdp)
    }
    pub async fn receive_sdp_answer(&mut self, sender_id: String, answer_sdp: String) -> Result<(), JsValue> {
        if let Some(connection) = self.connections.get(&sender_id) {
            let d = connection.connection.create_data_channel("");
            let mut answer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
            answer_obj.sdp(&answer_sdp);
            JsFuture::from(connection.connection.set_remote_description(&answer_obj)).await?;
            return Ok(())
        }
        Err(JsValue::from("No connection"))
    }
}