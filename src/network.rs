use js_sys::Reflect;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{
    MessageEvent, RtcDataChannelEvent, RtcPeerConnection, RtcDataChannel, RtcSdpType,
    RtcSessionDescriptionInit, RtcPeerConnectionIceEvent,
};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

struct Connection {
    connection: RtcPeerConnection,
}

#[wasm_bindgen]
pub struct Network {
    self_id: String,
    connections: HashMap<String, Connection>
}

#[wasm_bindgen]
impl Network {
    #[wasm_bindgen(constructor)]
    pub fn new(self_id: String) -> Result<Network, JsValue>  {
        Ok(Self { self_id: self_id, connections: Default::default() })
    }
    
    pub async fn send_sdp_offer(&mut self, target_id: String) -> Result<String, JsValue> {
        let pc = RtcPeerConnection::new()?;
        let dc = pc.create_data_channel(format!("data channel {} to {}", self.self_id, target_id).as_str());

        let pc_clone = pc.clone();
        let onicecandidate_callback = Closure::<dyn FnMut(_)>::new(move |ev: RtcPeerConnectionIceEvent| match ev.candidate() {
            Some(candidate) => {
                let _ = pc_clone.add_ice_candidate_with_opt_rtc_ice_candidate(Some(&candidate));
            }
            None => {}
        });
        pc.set_onicecandidate(Some(onicecandidate_callback.as_ref().unchecked_ref()));
        onicecandidate_callback.forget();
        
        let offer = JsFuture::from(pc.create_offer()).await?;
        let offer_sdp = Reflect::get(&offer, &JsValue::from_str("sdp"))?.as_string().unwrap();
        let mut offer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
        offer_obj.sdp(&offer_sdp);
        JsFuture::from(pc.set_local_description(&offer_obj)).await?;
        
        self.connections.insert(target_id, Connection{ connection: pc });    
        Ok(offer_sdp)
    }
    pub async fn answer_sdp_offer(&mut self, sender_id: String, offer_sdp: String) -> Result<String, JsValue> {
        let pc = RtcPeerConnection::new()?;
        
        let pc_clone = pc.clone();
        let onicecandidate_callback = Closure::<dyn FnMut(_)>::new(move |ev: RtcPeerConnectionIceEvent| match ev.candidate() {
            Some(candidate) => {
                let _ = pc_clone.add_ice_candidate_with_opt_rtc_ice_candidate(Some(&candidate));
            }
            None => {}
        });
        pc.set_onicecandidate(Some(onicecandidate_callback.as_ref().unchecked_ref()));
        onicecandidate_callback.forget();

        let mut offer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
        offer_obj.sdp(&offer_sdp);
        
        JsFuture::from(pc.set_remote_description(&offer_obj)).await?;

        let answer = JsFuture::from(pc.create_answer()).await?;
        let answer_sdp = Reflect::get(&answer, &JsValue::from_str("sdp"))?
            .as_string()
            .unwrap();
        let mut answer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
        answer_obj.sdp(&answer_sdp);
        JsFuture::from(pc.set_local_description(&answer_obj)).await?;

        self.connections.insert(sender_id, Connection{ connection: pc }); 

        Ok(answer_sdp)
    }
    pub async fn receive_sdp_answer(&mut self, sender_id: String, answer_sdp: String) -> Result<(), JsValue> {
        if let Some(connection) = self.connections.get(&sender_id) {
            let mut answer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
            answer_obj.sdp(&answer_sdp);
            JsFuture::from(connection.connection.set_remote_description(&answer_obj)).await?;
            return Ok(())
        }
        Err(JsValue::from("No connection"))
    }
}

#[wasm_bindgen]
pub async fn send_sdp_offer(instance: &mut Network, target_id: String) -> Result<String, JsValue> {
    let pc = RtcPeerConnection::new()?;
    let dc = pc.create_data_channel(format!("data channel {} to {}", instance.self_id, target_id).as_str());
    let offer = JsFuture::from(pc.create_offer()).await?;
    let offer_sdp = Reflect::get(&offer, &JsValue::from_str("sdp"))?.as_string().unwrap();
    let mut offer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
    offer_obj.sdp(&offer_sdp);
    JsFuture::from(pc.set_local_description(&offer_obj)).await?;
    
    instance.connections.insert(target_id, Connection{ connection: pc });    
    Ok(offer_sdp)
}

#[wasm_bindgen]
pub async fn answer_sdp_offer(instance: &mut Network, sender_id: String, offer_sdp: String) -> Result<String, JsValue> {
        let pc = RtcPeerConnection::new()?;
        
        let mut offer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Offer);
        offer_obj.sdp(&offer_sdp);
        
        JsFuture::from(pc.set_remote_description(&offer_obj)).await?;

        let answer = JsFuture::from(pc.create_answer()).await?;
        let answer_sdp = Reflect::get(&answer, &JsValue::from_str("sdp"))?
            .as_string()
            .unwrap();
        let mut answer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
        answer_obj.sdp(&answer_sdp);
        JsFuture::from(pc.set_local_description(&answer_obj)).await?;

        instance.connections.insert(sender_id, Connection{ connection: pc }); 

        Ok(answer_sdp)
}

#[wasm_bindgen]
pub async fn receive_sdp_answer(instance: &mut Network, sender_id: String, answer_sdp: String) -> Result<(), JsValue> {
    if let Some(connection) = instance.connections.get(&sender_id) {
        let mut answer_obj = RtcSessionDescriptionInit::new(RtcSdpType::Answer);
        answer_obj.sdp(&answer_sdp);
        JsFuture::from(connection.connection.set_remote_description(&answer_obj)).await?;
        return Ok(())
    }
    Err(JsValue::from("No connection"))
}