use hdk3::prelude::*;
use hc_utils::WrappedAgentPubKey;

#[hdk_entry(id = "user_entry", visibility = "public")]
#[derive(Clone)]
pub struct UserEntry {
    pub username: String,
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

    let user_entry = UserEntry {
        username: userdata.user.clone()
    };
    
    create_entry(&user_entry)?;
    let user = User {
        username: userdata.user,
        user_pub_key: WrappedAgentPubKey(agent_info.agent_latest_pubkey)
    };
    Ok(user)
}
