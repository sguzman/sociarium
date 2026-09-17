use std::collections::BTreeMap;

use sociarium_core::{
    NormalizedRecord, ObjectId, ObservationMeta, Post, ProfileSnapshot, RemoteId, TrackedProfile,
};

use crate::models::{XPost, XPostsEnvelope, XUser};

pub(crate) fn normalize_profile(
    tracked: &TrackedProfile,
    user: &XUser,
    observation: ObservationMeta,
) -> Result<NormalizedRecord, String> {
    let remote_id = RemoteId::new(user.id.clone()).map_err(|error| error.to_string())?;

    Ok(NormalizedRecord::ProfileSnapshot(ProfileSnapshot {
        profile_id: tracked.id.clone(),
        remote_id,
        handle: Some(user.username.clone()),
        display_name: Some(user.name.clone()),
        bio: user.description.clone(),
        avatar_url: user.profile_image_url.clone(),
        metrics: user.public_metrics.clone(),
        observation,
        extensions: BTreeMap::new(),
    }))
}

pub(crate) fn normalize_posts(
    tracked: &TrackedProfile,
    username: &str,
    envelope: &XPostsEnvelope,
    observation: &ObservationMeta,
) -> Result<Vec<NormalizedRecord>, String> {
    envelope
        .data
        .iter()
        .map(|post| normalize_post(tracked, username, post, observation.clone()))
        .collect()
}

fn normalize_post(
    tracked: &TrackedProfile,
    username: &str,
    post: &XPost,
    observation: ObservationMeta,
) -> Result<NormalizedRecord, String> {
    let id = x_post_object_id(&post.id)?;
    let remote_id = RemoteId::new(post.id.clone()).map_err(|error| error.to_string())?;
    let reply_to = reference_id(post, "replied_to")?;
    let quote_of = reference_id(post, "quoted")?;

    Ok(NormalizedRecord::Post(Post {
        id,
        profile_id: tracked.id.clone(),
        remote_id,
        created_at: post.created_at,
        text: post.text.clone(),
        reply_to,
        quote_of,
        canonical_url: Some(format!("https://x.com/{username}/status/{}", post.id)),
        observation,
        extensions: BTreeMap::new(),
    }))
}

fn reference_id(post: &XPost, kind: &str) -> Result<Option<ObjectId>, String> {
    post.referenced_posts
        .iter()
        .find(|reference| reference.kind == kind)
        .map(|reference| x_post_object_id(&reference.id))
        .transpose()
}

fn x_post_object_id(remote_id: &str) -> Result<ObjectId, String> {
    ObjectId::new(format!("x:post:{remote_id}")).map_err(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use sociarium_core::{ProfileId, ProfileOwnership, SurfaceId};

    use super::*;
    use crate::models::{XPostsEnvelope, XUserEnvelope};

    fn tracked_profile() -> TrackedProfile {
        TrackedProfile {
            id: ProfileId::new("x-main").unwrap(),
            surface: SurfaceId::new("x").unwrap(),
            remote_id: None,
            handle: Some("old-handle".to_owned()),
            ownership: ProfileOwnership::SelfOwned,
            enabled: true,
        }
    }

    fn observation() -> ObservationMeta {
        ObservationMeta {
            surface: SurfaceId::new("x").unwrap(),
            observed_at: Utc.with_ymd_and_hms(2026, 9, 17, 14, 0, 0).unwrap(),
            acquisition_id: "fixture-acquisition".to_owned(),
            schema_version: 1,
        }
    }

    #[test]
    fn normalizes_profile_without_treating_handle_as_identity() {
        let envelope: XUserEnvelope =
            serde_json::from_str(include_str!("../tests/fixtures/me.json")).unwrap();
        let record = normalize_profile(&tracked_profile(), &envelope.data, observation()).unwrap();

        let NormalizedRecord::ProfileSnapshot(snapshot) = record else {
            panic!("expected profile snapshot");
        };
        assert_eq!(snapshot.remote_id.as_str(), "6679733");
        assert_eq!(snapshot.handle.as_deref(), Some("sguzman"));
    }

    #[test]
    fn normalizes_posts_and_preserves_relationship_references_when_present() {
        let envelope: XPostsEnvelope =
            serde_json::from_str(include_str!("../tests/fixtures/posts.json")).unwrap();
        let records =
            normalize_posts(&tracked_profile(), "sguzman", &envelope, &observation()).unwrap();

        let NormalizedRecord::Post(post) = &records[1] else {
            panic!("expected post");
        };
        assert_eq!(post.id.as_str(), "x:post:200");
        assert_eq!(
            post.reply_to.as_ref().map(ObjectId::as_str),
            Some("x:post:150")
        );
        assert_eq!(
            post.quote_of.as_ref().map(ObjectId::as_str),
            Some("x:post:175")
        );
        assert_eq!(
            post.canonical_url.as_deref(),
            Some("https://x.com/sguzman/status/200")
        );
    }
}
