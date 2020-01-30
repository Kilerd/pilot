use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetUserProfilePhotos {
    /// Unique identifier of the target user
pub user_id: i32,
    /// Sequential number of the first photo to be returned. By default, all photos are returned.
    pub offset: Option<i32>,
    /// Limits the number of photos to be retrieved. Values between 1—100 are accepted. Defaults to 100.
    pub limit: Option<i32>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetFile {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KickChatMember {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnbanChatMember {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RestrictChatMember {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PromoteChatMember {
// todo
}



#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetChatAdministratorCustomTitle {
// todo
}


#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetChatPermissions {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportChatInviteLink {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetChatPhoto {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteChatPhoto {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetChatTitle {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetChatDescription {
// todo
}
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PinChatMessage {
// todo
}


#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnpinChatMessage {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LeaveChat {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetChat {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetChatAdministrators {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetChatMembersCount {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetChatMember {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetChatStickerSet {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteChatStickerSet {
// todo
}
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AnswerCallbackQuery {
// todo
}

