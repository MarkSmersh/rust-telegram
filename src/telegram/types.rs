use std::{error::Error, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize)]
pub struct Response<T> {
    pub ok: bool,
    pub error_code: Option<i32>,
    pub description: Option<String>,
    pub result: Option<T>,
}

#[derive(Serialize)]
pub struct GetMe {}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64, // up to 52 bits → i64 is safe
    pub is_bot: bool,
    pub first_name: String,

    pub last_name: Option<String>,
    pub username: Option<String>,
    pub language_code: Option<String>,

    pub is_premium: Option<bool>,
    pub added_to_attachment_menu: Option<bool>,

    // returned only in getMe
    pub can_join_groups: Option<bool>,
    pub can_read_all_group_messages: Option<bool>,
    pub supports_inline_queries: Option<bool>,
    pub supports_guest_queries: Option<bool>,
    pub can_connect_to_business: Option<bool>,
    pub has_main_web_app: Option<bool>,
    pub has_topics_enabled: Option<bool>,
    pub allows_users_to_create_topics: Option<bool>,
    pub can_manage_bots: Option<bool>,
    pub supports_join_request_queries: Option<bool>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct Update {
    pub update_id: i64,

    pub message: Option<Message>,
    pub edited_message: Option<Message>,
    pub channel_post: Option<Message>,
    pub edited_channel_post: Option<Message>,

    // pub business_connection: Option<BusinessConnection>,
    pub business_message: Option<Message>,
    pub edited_business_message: Option<Message>,
    // pub deleted_business_messages: Option<BusinessMessagesDeleted>,
    pub guest_message: Option<Message>,
    // pub message_reaction: Option<MessageReactionUpdated>,
    // pub message_reaction_count: Option<MessageReactionCountUpdated>,

    // pub inline_query: Option<InlineQuery>,
    // pub chosen_inline_result: Option<ChosenInlineResult>,
    // pub callback_query: Option<CallbackQuery>,
    //
    // pub shipping_query: Option<ShippingQuery>,
    // pub pre_checkout_query: Option<PreCheckoutQuery>,
    // pub purchased_paid_media: Option<PaidMediaPurchased>,
    //
    // pub poll: Option<Poll>,
    // pub poll_answer: Option<PollAnswer>,
    //
    // pub my_chat_member: Option<ChatMemberUpdated>,
    // pub chat_member: Option<ChatMemberUpdated>,
    // pub chat_join_request: Option<ChatJoinRequest>,
    //
    // pub chat_boost: Option<ChatBoostUpdated>,
    // pub removed_chat_boost: Option<ChatBoostRemoved>,
    //
    // pub managed_bot: Option<ManagedBotUpdated>,
    // pub subscription: Option<BotSubscriptionUpdated>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub message_id: i64,

    pub message_thread_id: Option<i64>,
    // pub direct_messages_topic: Option<DirectMessagesTopic>,
    pub from: Option<User>,
    // pub sender_chat: Option<Chat>,
    pub sender_boost_count: Option<i64>,
    pub sender_business_bot: Option<User>,
    pub sender_tag: Option<String>,
    pub receiver_user: Option<User>,
    pub ephemeral_message_id: Option<i64>,

    pub date: i64,
    pub guest_query_id: Option<String>,
    pub business_connection_id: Option<String>,

    // pub chat: Chat,

    // pub forward_origin: Option<MessageOrigin>,
    pub is_topic_message: Option<bool>,
    pub is_automatic_forward: Option<bool>,

    pub reply_to_message: Option<Box<Message>>,
    // pub external_reply: Option<ExternalReplyInfo>,
    // pub quote: Option<TextQuote>,
    // pub reply_to_story: Option<Story>,
    pub reply_to_checklist_task_id: Option<i64>,
    pub reply_to_poll_option_id: Option<String>,

    pub via_bot: Option<User>,
    // pub guest_bot_caller_user: Option<User>,
    // pub guest_bot_caller_chat: Option<Chat>,
    pub edit_date: Option<i64>,
    pub has_protected_content: Option<bool>,
    pub is_from_offline: Option<bool>,
    pub is_paid_post: Option<bool>,

    pub media_group_id: Option<String>,
    pub author_signature: Option<String>,
    pub paid_star_count: Option<i64>,

    pub text: Option<String>,
    // pub entities: Option<Vec<MessageEntity>>,
    // pub link_preview_options: Option<LinkPreviewOptions>,
    // pub suggested_post_info: Option<SuggestedPostInfo>,
    pub effect_id: Option<String>,
    // pub rich_message: Option<RichMessage>,

    // pub animation: Option<Animation>,
    // pub audio: Option<Audio>,
    // pub document: Option<Document>,
    // pub live_photo: Option<LivePhoto>,
    // pub paid_media: Option<PaidMediaInfo>,
    // pub photo: Option<Vec<PhotoSize>>,
    // pub sticker: Option<Sticker>,
    // pub story: Option<Story>,
    // pub video: Option<Video>,
    // pub video_note: Option<VideoNote>,
    // pub voice: Option<Voice>,
    pub caption: Option<String>,
    // pub caption_entities: Option<Vec<MessageEntity>>,
    pub show_caption_above_media: Option<bool>,
    pub has_media_spoiler: Option<bool>,

    // pub checklist: Option<Checklist>,
    // pub contact: Option<Contact>,
    // pub dice: Option<Dice>,
    // pub game: Option<Game>,
    // pub poll: Option<Poll>,
    // pub venue: Option<Venue>,
    // pub location: Option<Location>,
    pub new_chat_members: Option<Vec<User>>,
    pub left_chat_member: Option<User>,

    // pub chat_owner_left: Option<ChatOwnerLeft>,
    // pub chat_owner_changed: Option<ChatOwnerChanged>,
    pub new_chat_title: Option<String>,
    // pub new_chat_photo: Option<Vec<PhotoSize>>,
    pub delete_chat_photo: Option<bool>,
    pub group_chat_created: Option<bool>,
    pub supergroup_chat_created: Option<bool>,
    pub channel_chat_created: Option<bool>,

    // pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,
    pub migrate_to_chat_id: Option<i64>,
    pub migrate_from_chat_id: Option<i64>,
    // pub pinned_message: Option<MaybeInaccessibleMessage>,

    // pub invoice: Option<Invoice>,
    // pub successful_payment: Option<SuccessfulPayment>,
    // pub refunded_payment: Option<RefundedPayment>,

    // pub users_shared: Option<UsersShared>,
    // pub chat_shared: Option<ChatShared>,

    // pub gift: Option<GiftInfo>,
    // pub unique_gift: Option<UniqueGiftInfo>,
    // pub gift_upgrade_sent: Option<GiftInfo>,

    // pub connected_website: Option<String>,
    // pub write_access_allowed: Option<WriteAccessAllowed>,
    // pub passport_data: Option<PassportData>,
    //
    // pub proximity_alert_triggered: Option<ProximityAlertTriggered>,
    // pub boost_added: Option<ChatBoostAdded>,
    // pub chat_background_set: Option<ChatBackground>,
    //
    // pub checklist_tasks_done: Option<ChecklistTasksDone>,
    // pub checklist_tasks_added: Option<ChecklistTasksAdded>,
    //
    // pub community_chat_added: Option<CommunityChatAdded>,
    // pub community_chat_removed: Option<CommunityChatRemoved>,
    //
    // pub direct_message_price_changed: Option<DirectMessagePriceChanged>,
    //
    // pub forum_topic_created: Option<ForumTopicCreated>,
    // pub forum_topic_edited: Option<ForumTopicEdited>,
    // pub forum_topic_closed: Option<ForumTopicClosed>,
    // pub forum_topic_reopened: Option<ForumTopicReopened>,
    // pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,
    // pub general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,
    //
    // pub giveaway_created: Option<GiveawayCreated>,
    // pub giveaway: Option<Giveaway>,
    // pub giveaway_winners: Option<GiveawayWinners>,
    // pub giveaway_completed: Option<GiveawayCompleted>,
    //
    // pub managed_bot_created: Option<ManagedBotCreated>,
    // pub paid_message_price_changed: Option<PaidMessagePriceChanged>,
    //
    // pub poll_option_added: Option<PollOptionAdded>,
    // pub poll_option_deleted: Option<PollOptionDeleted>,
    //
    // pub suggested_post_approved: Option<SuggestedPostApproved>,
    // pub suggested_post_approval_failed: Option<SuggestedPostApprovalFailed>,
    // pub suggested_post_declined: Option<SuggestedPostDeclined>,
    // pub suggested_post_paid: Option<SuggestedPostPaid>,
    // pub suggested_post_refunded: Option<SuggestedPostRefunded>,
    //
    // pub video_chat_scheduled: Option<VideoChatScheduled>,
    // pub video_chat_started: Option<VideoChatStarted>,
    // pub video_chat_ended: Option<VideoChatEnded>,
    // pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,
    //
    // pub web_app_data: Option<WebAppData>,
    // pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Default)]
pub struct GetUpdates {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub timeout: Option<i64>,
    pub allowed_updates: Option<Vec<String>>,
}

#[derive(Serialize, Debug, Clone)]
pub enum ParseMode {
    HTML,
    Markdown,
    MarkdownV2,
}

impl ToString for ParseMode {
    fn to_string(&self) -> String {
        match self {
            Self::HTML => "HTML".to_string(),
            Self::Markdown => "Markdown".to_string(),
            Self::MarkdownV2 => "MarkdownV2".to_string(),
        }
    }
}

impl FromStr for ParseMode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "HTML" => Ok(Self::HTML),
            "Markdown" => Ok(Self::Markdown),
            "MarkdownV2" => Ok(Self::MarkdownV2),
            &_ => Err("Unable to parse string to ParseMode"),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Default)]
pub struct SendMessage {
    pub business_connection_id: Option<String>,
    pub chat_id: i64,
    pub message_thread_id: Option<i64>,
    pub direct_messages_topic_id: Option<i64>,
    pub receiver_user_id: Option<i64>,
    pub callback_query_id: Option<String>,
    pub text: String,
    pub parse_mode: Option<ParseMode>,
    // pub entities: Option<Vec<MessageEntity>>,
    // pub link_preview_options: Option<LinkPreviewOptions>,
    pub disable_notification: Option<bool>,
    pub protect_content: Option<bool>,
    pub allow_paid_broadcast: Option<bool>,
    pub message_effect_id: Option<String>,
    // pub suggested_post_parameters: Option<SuggestedPostParameters>,
    // pub reply_parameters: Option<ReplyParameters>,
    // pub reply_markup: Option<ReplyMarkup>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chat {
    pub id: i64, // up to 52 bits → i64 is safe

    #[serde(rename = "type")]
    pub chat_type: ChatType,

    pub title: Option<String>,
    pub username: Option<String>,

    pub first_name: Option<String>,
    pub last_name: Option<String>,

    pub is_forum: Option<bool>,
    pub is_direct_messages: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatType {
    Private,
    Group,
    Supergroup,
    Channel,
}
#[derive(Debug, Serialize, Default)]
pub struct SendMessageDraft {
    pub chat_id: i64,
    pub message_thread_id: Option<i64>,
    pub draft_id: i64, // must be non-zero
    pub text: Option<String>,
    pub parse_mode: Option<ParseMode>,
    // pub entities: Option<Vec<MessageEntity>>,
}
