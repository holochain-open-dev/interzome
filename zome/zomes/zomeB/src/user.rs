use crate::utils;
use hdk3::prelude::*;
use hc_utils::WrappedAgentPubKey;

#[hdk_entry(id = "user_entry", visibility = "public")]
#[derive(Clone)]
pub struct UserEntry {
    pub username: String,
    pub user_pub_key: AgentPubKey,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct UserData {
    pub user: String,
}

#[derive(Clone, Serialize, Deserialize, SerializedBytes)]
pub struct User {
    pub username: String,
    pub user_pub_key: WrappedAgentPubKey,
}

pub fn set_username(userdata: UserData
) -> ExternResult<User> {
    let agent_info = agent_info()?;

    let userEntry = UserEntry {
        username: userdata.username,
        user_pub_key: agent_info.latest_pub_key
    };
    create_entry(&userEntry)?;
    let user = User{
        username: userdata.username,
        user_pub_key: WrappedAgentPubKey(userEntry.user_pub_key.clone())
    }
    Ok(user)
}
