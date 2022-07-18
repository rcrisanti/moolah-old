use reqwest::{RequestBuilder, Response, StatusCode};
use std::{future::Future, pin::Pin};

use crate::{errors::InternalResponseError, ResponseResult};

type Action<T> = Box<dyn Fn(Response) -> Pin<Box<dyn Future<Output = ResponseResult<T>>>>>;

pub struct ResponseAction<T> {
    action: Action<T>,
}

impl<T> ResponseAction<T> {
    pub fn new(action: Action<T>) -> Self {
        ResponseAction { action }
    }

    pub async fn act(&self, response: Response) -> ResponseResult<T> {
        (self.action)(response).await
    }
}

impl<T, C> From<C> for ResponseAction<T>
where
    C: Fn(Response) -> ResponseResult<T> + Copy + 'static,
{
    fn from(closure: C) -> Self {
        ResponseAction {
            action: Box::new(move |response| Box::pin(async move { closure(response) })),
        }
    }
}

pub struct Requester<T> {
    pub on_fallthrough: ResponseAction<T>,
    pub on_no_response: Box<dyn Fn() -> ResponseResult<T>>,
    pub on_unauth: ResponseAction<T>,
}

impl<T> Requester<T> {
    pub async fn make(
        &self,
        request: RequestBuilder,
        on_ok: ResponseAction<T>,
    ) -> ResponseResult<T> {
        let response = request.send().await.ok();

        if let Some(response) = response {
            match response.status() {
                StatusCode::OK => on_ok.act(response).await,
                StatusCode::UNAUTHORIZED => self.on_unauth.act(response).await,
                _ => self.on_fallthrough.act(response).await,
            }
        } else {
            (self.on_no_response)()
        }
    }
}

impl<T> Default for Requester<T> {
    fn default() -> Self {
        Requester {
            on_unauth: (|_| Err(InternalResponseError::Unauthorized)).into(),
            on_fallthrough: ResponseAction::new(Box::new(|response: Response| {
                Box::pin(async move {
                    response.text().await.map_or(
                        Err(InternalResponseError::Other(
                            "could not get body text".into(),
                        )),
                        |err_text| {
                            Err(InternalResponseError::ResponseAwaitError(
                                "error text body",
                                err_text,
                            ))
                        },
                    )
                })
            })),
            on_no_response: Box::new(|| {
                Err(InternalResponseError::Other(
                    "could not send request".into(),
                ))
            }),
        }
    }
}
