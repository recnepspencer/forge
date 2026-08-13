use std::time::Duration;

use crate::protocol::BankUserNodeDenialKind;

use super::BankUserSession;

#[cfg(test)]
mod tests;

impl BankUserSession {
    pub(in crate::session) async fn send_upstream<Request>(
        &self,
        endpoint: url::Url,
        request: &Request,
        deadline_milliseconds: u64,
    ) -> Result<reqwest::Response, BankUserNodeDenialKind>
    where
        Request: serde::Serialize + ?Sized,
    {
        send_with_deadline(
            &self.client,
            endpoint,
            request,
            self.maximum_deadline,
            deadline_milliseconds,
        )
        .await
    }

    pub(in crate::session) async fn forward<Request, Response>(
        &self,
        endpoint: url::Url,
        request: &Request,
        deadline_milliseconds: u64,
    ) -> Result<Response, BankUserNodeDenialKind>
    where
        Request: serde::Serialize + ?Sized,
        Response: serde::de::DeserializeOwned,
    {
        let response = self
            .send_upstream(endpoint, request, deadline_milliseconds)
            .await?;
        response.json::<Response>().await.map_err(upstream_error)
    }
}

async fn send_with_deadline<Request>(
    client: &reqwest::Client,
    endpoint: url::Url,
    request: &Request,
    maximum_deadline: Duration,
    deadline_milliseconds: u64,
) -> Result<reqwest::Response, BankUserNodeDenialKind>
where
    Request: serde::Serialize + ?Sized,
{
    let timeout = Duration::from_millis(deadline_milliseconds);
    if timeout.is_zero() || timeout > maximum_deadline {
        return Err(BankUserNodeDenialKind::MalformedRequest);
    }
    client
        .post(endpoint)
        .timeout(timeout)
        .json(request)
        .send()
        .await
        .map_err(upstream_error)
}

pub(in crate::session) fn upstream_error(error: reqwest::Error) -> BankUserNodeDenialKind {
    if error.is_timeout() {
        BankUserNodeDenialKind::UpstreamDeadlineExceeded
    } else if error.is_decode() {
        BankUserNodeDenialKind::UpstreamProtocolViolation
    } else {
        BankUserNodeDenialKind::UpstreamUnavailable
    }
}
