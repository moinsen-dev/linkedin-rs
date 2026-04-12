//! GraphQL query IDs extracted from the decompiled LinkedIn Android APK.
//!
//! These hashes are version-locked to a specific APK build. When LinkedIn
//! ships a new version, some or all of these may change silently, causing
//! HTTP 400 responses. To update:
//!
//! 1. Obtain the new APK via `apkeep`.
//! 2. Decompile with `jadx`.
//! 3. Search for `queryId` assignments in `*GraphQLClient.java` classes.
//! 4. Update the constants below.
//!
//! Source APK: international variant, version 4.1.1183
//! Decompiled classes:
//!   - `ProfileGraphQLClient.java`
//!   - `MessengerGraphQLClient.java`
//!   - `SearchGraphQLClient.java`
//!   - `CareersGraphQLClient.java`
//!   - `NotificationsGraphQLClient.java`
//!   - `FeedFrameworkGraphQLClient.java`
//!   - `ConversationsGraphQLClient.java`
//!   - `SharingGraphQLClient.java`
//!   - `MynetworkGraphQLClient.java`

// -- Profile --

/// Get profile by vanity URL / public identifier.
/// Source: `ProfileGraphQLClient.java` static initializer.
pub const PROFILE_BY_MEMBER_IDENTITY: &str =
    "voyagerIdentityDashProfiles.5f50f83f76a1e270603613bdd0fb0252";

/// Visit a profile (registers the view in "Who Viewed My Profile").
/// Source: Chrome DevTools capture (profile-top-card-supplementary decoration).
pub const PROFILE_VISIT: &str = "voyagerIdentityDashProfiles.a3de77c32c473719f1c58fae6bff43a5";

// -- Messaging --

/// List conversations by category (PRIMARY_INBOX, etc.).
/// Source: `MessengerGraphQLClient.java`.
pub const CONVERSATIONS_BY_CATEGORY: &str =
    "voyagerMessagingDashMessengerConversations.7dc50d3efc3953190125aca9c05f0af6";

/// Fetch messages within a specific conversation.
/// Source: `MessengerGraphQLClient.java`.
pub const MESSAGES_BY_CONVERSATION: &str =
    "voyagerMessagingDashMessengerMessages.7cde5843a127bbecc3de900d3894a74a";

// -- Search --

/// Search people/content via `searchDashClustersByAll`.
/// Source: `SearchGraphQLClient.java` / `SearchFrameworkUtils.java`.
pub const SEARCH_CLUSTERS: &str = "voyagerSearchDashClusters.fae19421cdd51a7cd735e0b7d7b32e0f";

/// Search jobs via `jobsDashJobCardsByJobSearch`.
/// Source: `CareersGraphQLClient.java`.
pub const JOB_CARDS_BY_SEARCH: &str = "voyagerJobsDashJobCards.4ef915ad5827cd8ea1351ad72f8e4268";

// -- Notifications --

/// Get notification cards by filter.
/// Source: `NotificationsGraphQLClient.java` static initializer.
pub const NOTIFICATION_CARDS: &str =
    "voyagerIdentityDashNotificationCards.1a1ca07d1f7a6e1033fd88d5fd2da611";

// -- Reactions --

/// Create a reaction (like, celebrate, etc.) on a post.
/// Source: `FeedFrameworkGraphQLClient.java` (createSocialDashReactions).
pub const REACTION_CREATE: &str = "voyagerSocialDashReactions.fd68eadaf15da416b0d839e21399b763";

/// Delete a reaction from a post.
/// Source: `FeedFrameworkGraphQLClient.java` (doDeleteReactionSocialDashReactions).
pub const REACTION_DELETE: &str = "voyagerSocialDashReactions.315cef4773de8e3a0ddad7655cc1685f";

// -- Comments --

/// Create a comment on a feed post.
/// Source: `ConversationsGraphQLClient.java` static initializer.
pub const COMMENT_CREATE: &str = "voyagerSocialDashNormComments.cd3d2a3fd6c9b2881c7cac32847ec05e";

// -- Posts / Shares --

/// Create a text post (web-style mutation format).
/// Source: Chrome DevTools capture of web client share flow.
pub const SHARE_CREATE: &str = "voyagerContentcreationDashShares.279996efa5064c01775d5aff003d9377";

// -- Invitations --

/// Fetch received connection invitations.
/// Source: `MynetworkGraphQLClient.receivedInvitationViews()`.
pub const INVITATION_VIEWS: &str =
    "voyagerRelationshipsDashInvitationViews.48949225027e0a85d063176777f08e7f";
