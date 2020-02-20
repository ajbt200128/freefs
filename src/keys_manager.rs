use crate::proto::{
    keys::{AuthSetupRequest, AuthUnlockRequest, KeyType, KeysRequest, SortDirection, Key, EncryptRequest, EncryptMode, DecryptRequest},
    keys_grpc::KeysClient,
};
use grpcio::{CallOption, ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder, MetadataBuilder};
use std::{fs, sync::Arc};
use protobuf::RepeatedField;
use std::env;

pub struct KeysManager {
    client: KeysClient,
    token: String,
    key: Option<Key>,
}

impl KeysManager {
    pub fn new() -> Self {
        let env = Arc::new(EnvBuilder::new().build());
        let cert_file = fs::read("/home/austin/.local/share/Keys/ca.pem").unwrap();
        let cert = ChannelCredentialsBuilder::new()
            .root_cert(cert_file)
            .build();
        let ch = ChannelBuilder::new(env).secure_connect("localhost:22405", cert);
        let client = KeysClient::new(ch);
        Self {
            client,
            token: "".to_string(),
            key: None,
        }
    }
    fn build_auth_call_options(&self) -> CallOption {
        let mut builder = MetadataBuilder::new();
        builder.add_str("authorization", &self.token).unwrap();
        let metadata = builder.build();
        CallOption::default().headers(metadata)
    }

    pub fn auth_setup(&mut self) {
        let password = env::var("FREEFS_PASSWORD").unwrap();
        let mut req = AuthSetupRequest::default();
        req.set_password(password);
        req.set_client("bbfs".to_string());
        let reply = self.client.auth_setup(&req).unwrap();
        let token = reply.authToken;
        self.token = token.into();
        println!("token: {:?}", self.token);
    }

    pub fn auth_unlock(&mut self) {
        let password = env::var("FREEFS_PASSWORD").unwrap();
        let mut req = AuthUnlockRequest::default();
        req.set_password(password);
        req.set_client("freefs".to_string());
        let reply = self.client.auth_unlock(&req).unwrap();
        let token = reply.authToken;
        self.token = token;
        println!("token: {:?}", self.token);
        self.set_key();
    }

    pub fn set_key(&mut self) {
        let mut req = KeysRequest::default();
        req.set_sortDirection(SortDirection::ASC);
        req.set_types(vec![KeyType::EDX25519, KeyType::X25519]);
        let call_opt = self.build_auth_call_options();
        let reply = self.client.keys_opt(&req, call_opt).unwrap();
        let keys = reply.keys;
        self.key = Some(keys.first().unwrap().to_owned());
    }

    pub fn encrypt(&self, data:Vec<u8>,recipients:Option<Vec<String>>, armored:bool) -> Vec<u8>{
        let recipients = recipients.unwrap_or(vec![self.key.clone().unwrap().id]);
        let mut req = EncryptRequest::default();
        req.set_data(data);
        req.set_armored(armored);
        let recipients = RepeatedField::from_vec(recipients);
        req.set_recipients(recipients);
        req.set_mode(EncryptMode::DEFAULT_ENCRYPT_MODE);
        let call_opt= self.build_auth_call_options();
        let reply = self.client.encrypt_opt(&req, call_opt).unwrap();
        let enc_data = reply.data;
        enc_data
    }

    pub fn decrypt(&self, data:Vec<u8>, armored:bool) -> Vec<u8>{
        let mut req = DecryptRequest::default();
        req.set_data(data);
        req.set_armored(armored);
        req.set_mode(EncryptMode::DEFAULT_ENCRYPT_MODE);
        let call_opt = self.build_auth_call_options();
        let reply = self.client.decrypt_opt(&req, call_opt).unwrap();
        let dec_data= reply.data;
        dec_data
    }
}
