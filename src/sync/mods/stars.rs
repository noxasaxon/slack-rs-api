//=============================================================================
//
//                    WARNING: This file is AUTO-GENERATED
//
// Do not make changes directly to this file.
//
// If you would like to make a change to the library, please update the schema
// definitions at https://github.com/slack-rs/slack-api-schemas
//
// If you would like to make a change how the library was generated,
// please edit https://github.com/slack-rs/slack-rs-api/tree/master/codegen
//
//=============================================================================

pub use crate::mod_types::stars_types::*;
use crate::sync::requests::SlackWebRequestSender;

/// Adds a star to an item.
///
/// Wraps https://api.slack.com/methods/stars.add

pub fn add<R>(
    client: &R,
    token: &str,
    request: &AddRequest<'_>,
) -> Result<AddResponse, AddError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let timestamp = request.timestamp.as_ref().map(|t| t.to_param_value());
    let params = vec![
        Some(("token", token)),
        request.file.map(|file| ("file", file)),
        request
            .file_comment
            .map(|file_comment| ("file_comment", file_comment)),
        request.channel.map(|channel| ("channel", channel)),
        timestamp
            .as_ref()
            .map(|timestamp| ("timestamp", &timestamp[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("stars.add");
    client
        .send(&url, &params[..])
        .map_err(AddError::Client)
        .and_then(|result| {
            serde_json::from_str::<AddResponse>(&result)
                .map_err(|e| AddError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Lists stars for a user.
///
/// Wraps https://api.slack.com/methods/stars.list

pub fn list<R>(
    client: &R,
    token: &str,
    request: &ListRequest,
) -> Result<ListResponse, ListError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let count = request.count.map(|count| count.to_string());
    let page = request.page.map(|page| page.to_string());
    let params = vec![
        Some(("token", token)),
        count.as_ref().map(|count| ("count", &count[..])),
        page.as_ref().map(|page| ("page", &page[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("stars.list");
    client
        .send(&url, &params[..])
        .map_err(ListError::Client)
        .and_then(|result| {
            serde_json::from_str::<ListResponse>(&result)
                .map_err(|e| ListError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}

/// Removes a star from an item.
///
/// Wraps https://api.slack.com/methods/stars.remove

pub fn remove<R>(
    client: &R,
    token: &str,
    request: &RemoveRequest<'_>,
) -> Result<RemoveResponse, RemoveError<R::Error>>
where
    R: SlackWebRequestSender,
{
    let timestamp = request.timestamp.as_ref().map(|t| t.to_param_value());
    let params = vec![
        Some(("token", token)),
        request.file.map(|file| ("file", file)),
        request
            .file_comment
            .map(|file_comment| ("file_comment", file_comment)),
        request.channel.map(|channel| ("channel", channel)),
        timestamp
            .as_ref()
            .map(|timestamp| ("timestamp", &timestamp[..])),
    ];
    let params = params.into_iter().filter_map(|x| x).collect::<Vec<_>>();
    let url = crate::get_slack_url_for_method("stars.remove");
    client
        .send(&url, &params[..])
        .map_err(RemoveError::Client)
        .and_then(|result| {
            serde_json::from_str::<RemoveResponse>(&result)
                .map_err(|e| RemoveError::MalformedResponse(result, e))
        })
        .and_then(|o| o.into())
}