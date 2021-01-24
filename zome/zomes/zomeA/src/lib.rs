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

pub fn get_agent_pubkey_from_username(username_input: UsernameWrapper) -> ExternResult<AgentPubKey> {
    let agent_info = agent_info()?;
    Ok(agent_info.agent_initial_pubkey)  
    
    /* //get entry by its entry hash instead of links
    let username_entry = UsernameEntry { username: username_input.0 };
    let username_hash = hash_entry(&username_entry)?;
    let option = GetOptions::latest();
    match get(username_hash, option)? {
        Some(el) => {
            let header_details = el.header();
            Ok(header_details.author().to_owned())
        },
        None => crate::error("The username does not exist")
    } */
}
