use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Hello,
    Ready,
    Resumed,
    Reconnect,
    InvalidSession,
    ApplicationCommandPermissionsUpdate,
    AutoModerationRuleCreate,
    AutoModerationRuleUpdate,
    AutoModerationRuleDelete,
    AutoModerationActionExecution,
    ChannelCreate,
    ChannelUpdate,
    ChannelDelete,
    ChannelPinsUpdate,
    ThreadCreate,
    ThreadUpdate,
    ThreadDelete,
    ThreadListSync,
    ThreadMemberUpdate,
    ThreadMembersUpdate,
    EntitlementCreate,
    EntitlementUpdate,
    EntitlementDelete,
    GuildCreate,
    GuildUpdate,
    GuildDelete,
    GuildAuditLogEntryCreate,
    GuildBanAdd,
    GuildBanRemove,
    GuildEmojisUpdate,
    GuildStickersUpdate,
    GuildIntegrationsUpdate,
    GuildMemberAdd,
    GuildMemberRemove,
    GuildMemberUpdate,
    GuildMembersChunk,
    GuildRoleCreate,
    GuildRoleUpdate,
    GuildRoleDelete,
    GuildScheduledEventCreate,
    GuildScheduledEventUpdate,
    GuildScheduledEventDelete,
    GuildScheduledEventUserAdd,
    GuildScheduledEventUserRemove,
    IntegrationCreate,
    IntegrationUpdate,
    IntegrationDelete,
    InteractionCreate,
    InviteCreate,
    InviteDelete,
    MessageCreate,
    MessageUpdate,
    MessageDelete,
    MessageDeleteBulk,
    MessageReactionAdd,
    MessageReactionRemove,
    MessageReactionRemoveAll,
    MessageReactionRemoveEmoji,
    PresenceUpdate,
    StageInstanceCreate,
    StageInstanceUpdate,
    StageInstanceDelete,
    TypingStart,
    UserUpdate,
    VoiceStateUpdate,
    VoiceServerUpdate,
    WebhooksUpdate,
}

impl FromStr for Event {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "HELLO" => Event::Hello,
            "READY" => Event::Ready,
            "RESUMED" => Event::Resumed,
            "RECONNECT" => Event::Reconnect,
            "INVALID_SESSION" => Event::InvalidSession,
            "APPLICATION_COMMAND_PERMISSIONS_UPDATE" => Event::ApplicationCommandPermissionsUpdate,
            "AUTO_MODERATION_RULE_CREATE" => Event::AutoModerationRuleCreate,
            "AUTO_MODERATION_RULE_UPDATE" => Event::AutoModerationRuleUpdate,
            "AUTO_MODERATION_RULE_DELETE" => Event::AutoModerationRuleDelete,
            "AUTO_MODERATION_ACTION_EXECUTION" => Event::AutoModerationActionExecution,
            "CHANNEL_CREATE" => Event::ChannelCreate,
            "CHANNEL_UPDATE" => Event::ChannelUpdate,
            "CHANNEL_DELETE" => Event::ChannelDelete,
            "CHANNEL_PINS_UPDATE" => Event::ChannelPinsUpdate,
            "THREAD_CREATE" => Event::ThreadCreate,
            "THREAD_UPDATE" => Event::ThreadUpdate,
            "THREAD_DELETE" => Event::ThreadDelete,
            "THREAD_LIST_SYNC" => Event::ThreadListSync,
            "THREAD_MEMBER_UPDATE" => Event::ThreadMemberUpdate,
            "THREAD_MEMBERS_UPDATE" => Event::ThreadMembersUpdate,
            "ENTITLEMENT_CREATE" => Event::EntitlementCreate,
            "ENTITLEMENT_UPDATE" => Event::EntitlementUpdate,
            "ENTITLEMENT_DELETE" => Event::EntitlementDelete,
            "GUILD_CREATE" => Event::GuildCreate,
            "GUILD_UPDATE" => Event::GuildUpdate,
            "GUILD_DELETE" => Event::GuildDelete,
            "GUILD_AUDIT_LOG_ENTRY_CREATE" => Event::GuildAuditLogEntryCreate,
            "GUILD_BAN_ADD" => Event::GuildBanAdd,
            "GUILD_BAN_REMOVE" => Event::GuildBanRemove,
            "GUILD_EMOJIS_UPDATE" => Event::GuildEmojisUpdate,
            "GUILD_STICKERS_UPDATE" => Event::GuildStickersUpdate,
            "GUILD_INTEGRATIONS_UPDATE" => Event::GuildIntegrationsUpdate,
            "GUILD_MEMBER_ADD" => Event::GuildMemberAdd,
            "GUILD_MEMBER_REMOVE" => Event::GuildMemberRemove,
            "GUILD_MEMBER_UPDATE" => Event::GuildMemberUpdate,
            "GUILD_MEMBERS_CHUNK" => Event::GuildMembersChunk,
            "GUILD_ROLE_CREATE" => Event::GuildRoleCreate,
            "GUILD_ROLE_UPDATE" => Event::GuildRoleUpdate,
            "GUILD_ROLE_DELETE" => Event::GuildRoleDelete,
            "GUILD_SCHEDULED_EVENT_CREATE" => Event::GuildScheduledEventCreate,
            "GUILD_SCHEDULED_EVENT_UPDATE" => Event::GuildScheduledEventUpdate,
            "GUILD_SCHEDULED_EVENT_DELETE" => Event::GuildScheduledEventDelete,
            "GUILD_SCHEDULED_EVENT_USER_ADD" => Event::GuildScheduledEventUserAdd,
            "GUILD_SCHEDULED_EVENT_USER_REMOVE" => Event::GuildScheduledEventUserRemove,
            "INTEGRATION_CREATE" => Event::IntegrationCreate,
            "INTEGRATION_UPDATE" => Event::IntegrationUpdate,
            "INTEGRATION_DELETE" => Event::IntegrationDelete,
            "INTERACTION_CREATE" => Event::InteractionCreate,
            "INVITE_CREATE" => Event::InviteCreate,
            "INVITE_DELETE" => Event::InviteDelete,
            "MESSAGE_CREATE" => Event::MessageCreate,
            "MESSAGE_UPDATE" => Event::MessageUpdate,
            "MESSAGE_DELETE" => Event::MessageDelete,
            "MESSAGE_DELETE_BULK" => Event::MessageDeleteBulk,
            "MESSAGE_REACTION_ADD" => Event::MessageReactionAdd,
            "MESSAGE_REACTION_REMOVE" => Event::MessageReactionRemove,
            "MESSAGE_REACTION_REMOVE_ALL" => Event::MessageReactionRemoveAll,
            "MESSAGE_REACTION_REMOVE_EMOJI" => Event::MessageReactionRemoveEmoji,
            "PRESENCE_UPDATE" => Event::PresenceUpdate,
            "STAGE_INSTANCE_CREATE" => Event::StageInstanceCreate,
            "STAGE_INSTANCE_UPDATE" => Event::StageInstanceUpdate,
            "STAGE_INSTANCE_DELETE" => Event::StageInstanceDelete,
            "TYPING_START" => Event::TypingStart,
            "USER_UPDATE" => Event::UserUpdate,
            "VOICE_STATE_UPDATE" => Event::VoiceStateUpdate,
            "VOICE_SERVER_UPDATE" => Event::VoiceServerUpdate,
            "WEBHOOKS_UPDATE" => Event::WebhooksUpdate,

            _ => return Err(()),
        })
    }
}
