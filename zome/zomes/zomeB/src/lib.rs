use hdk3::prelude::*;
use hc_utils::WrappedEntryHash;

mod user;
mod utils;

pub fn err(reason: &str) -> HdkError {
    HdkError::Wasm(WasmError::Zome(String::from(reason)))
}

entry_defs![
    Path::entry_def(),
    user::UserEntry::entry_def()
];

/** user api **/

#[hdk_extern]
pub fn set_username(
    userdata: user::UserData,
) -> ExternResult<User> {
    user::set_username(userdata)
}

#[derive(Deserialize, Serialize, SerializedBytes, Clone, Debug)]
pub struct UsernameWrapper(pub String);

pub fn get_agent_pubkey_from_username(
    username: UsernameWrapper,
) -> ExternResult<AgentPubKey> {
    let function_name = zome::FunctionName("get_agent_pubkey_from_username".to_owned());
    // needs to handle error from get_agent_pubkey_from_username in UI
    let agent_pubkey = hdk3::prelude::call(
        None,
        "zomeA".into(),
        function_name,
        None,
        &username
    );
    match agent_pubkey
    {
        Err(e) => {
            panic!("Unable to make interzome call: {:?}", e);
        }
        Ok(_) => {Ok(agent_pubkey?)}
    }
   // Ok(agent_pubkey)
}

