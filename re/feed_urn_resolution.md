# Feed URN Resolution: Direct vs. Mediated Posts

## Discovery

When reacting to or commenting on feed posts, LinkedIn returns HTTP 200 with
a GraphQL error (status 400) for certain posts. The root cause is a URN mismatch
between the feed update event and the actual post being displayed.

## The Two-URN Problem

Every feed item has two relevant URNs:

1. **Feed update URN** (top-level `urn`): Identifies the feed routing event --
   WHY this item appeared in the feed. For mediated posts (someone liked,
   commented, or reposted), this is a synthetic activity URN that represents
   the mediation action, NOT the original post.

2. **Social detail URN** (`socialDetail.urn` / `socialDetail.threadId`): Identifies
   the ORIGINAL post that reactions/comments should target.

### Direct Posts

Feed items from followed accounts or companies where the author IS the poster:

```
top-level urn:     urn:li:activity:7444748916228562944
socialDetail.urn:  urn:li:activity:7444748916228562944   ← same!
```

Both URNs match. Either can be used for react/comment.

### Mediated Posts

Feed items that appear because someone in your network interacted with a post:

```
top-level urn:     urn:li:activity:7445041846344511488   ← feed event (NOT reactable)
socialDetail.urn:  urn:li:activity:7445032315958972416   ← original post (reactable)
```

The top-level URN is a feed routing event. Reacting to it returns:
```json
{
  "value": {
    "errors": [{
      "message": "Couldn't react to this post. Please try again.",
      "extensions": { "status": 400, "classification": "DataFetchingException" }
    }]
  }
}
```

## Detection

Mediated posts can be identified by:

1. **`header` block present**: Contains the person who caused the feed event
   (e.g., "Visa Nagieva likes this", "Kurt Schauer commented on this").
2. **`actionsPosition: "HEADER_COMPONENT"`**: Mediated posts use this value;
   direct posts use `"ACTOR_COMPONENT"`.
3. **URN mismatch**: `element.urn != socialDetail.urn`.

## Resolution Rule

**Always use `socialDetail.urn`** (or equivalently `socialDetail.threadId`)
as the `threadUrn` for react/comment API calls. This works for BOTH direct
and mediated posts.

JSON path in feed response:
```
element.value["com.linkedin.voyager.feed.render.UpdateV2"].socialDetail.urn
```

The `dashEntityUrn` in socialDetail also encodes both URNs:
```
urn:li:fsd_socialDetail:(<ORIGINAL_ACTIVITY>,<FEED_ACTIVITY>,urn:li:highlightedReply:-)
```

## Verified Examples

| Post Type | Feed URN (top-level) | Social Detail URN | React Result |
|-----------|---------------------|-------------------|--------------|
| Direct (André Lindenberg) | `activity:7444748916228562944` | `activity:7444748916228562944` | ✅ Success |
| Mediated (Parwiz via Visa) | `activity:7445041846344511488` | `activity:7445032315958972416` | ❌ with feed URN, ✅ with social URN |
| Mediated (Dennis via Kurt) | `activity:7445039297545658368` | `activity:7445025683304652800` | ❌ with feed URN |
| Mediated (brand eins via Christian) | `activity:7444693980098760704` | `activity:7444639861367521280` | ❌ with feed URN |
