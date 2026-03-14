use actix_web::{
    dev::{Payload, Service, ServiceRequest, ServiceResponse, Transform},
    error, Error,
    guard::{Guard, GuardContext},
    HttpMessage,
};
use base64::engine::general_purpose;
use base64::Engine;
use bytes::BytesMut;
use futures::future::{ok, LocalBoxFuture, Ready};
use futures_util::StreamExt;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::rc::Rc;
use std::task::{Context, Poll};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, PartialEq)]
pub enum KeyType {
    Public,
    Private,
}

pub struct RequirePrivate;

impl Guard for RequirePrivate {
    fn check(&self, ctx: &GuardContext) -> bool {
        ctx.req_data()
            .get::<KeyType>()
            .map(|k| *k == KeyType::Private)
            .unwrap_or(false)
    }
}

pub struct AuthMiddleware {
    public_api_key: String,
    private_api_key: String,
    secret_key: String,
}

impl AuthMiddleware {
    pub fn new(
        public_api_key: impl Into<String>,
        private_api_key: impl Into<String>,
        secret_key: impl Into<String>,
    ) -> Self {
        Self {
            public_api_key: public_api_key.into(),
            private_api_key: private_api_key.into(),
            secret_key: secret_key.into(),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareInner<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareInner {
            service: Rc::new(service),
            public_api_key: self.public_api_key.clone(),
            private_api_key: self.private_api_key.clone(),
            secret_key: self.secret_key.clone(),
        })
    }
}

pub struct AuthMiddlewareInner<S> {
    service: Rc<S>,
    public_api_key: String,
    private_api_key: String,
    secret_key: String,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let public_api_key = self.public_api_key.clone();
        let private_api_key = self.private_api_key.clone();
        let secret_key = self.secret_key.clone();
        let service = self.service.clone();

        Box::pin(async move {
            let (api_key_header, signature_header) = {
                let headers = req.headers();
                let api_key = headers
                    .get("X-API-KEY")
                    .and_then(|v| v.to_str().ok())
                    .ok_or_else(|| error::ErrorUnauthorized("Missing API key"))?
                    .to_string();
                let signature = headers
                    .get("X-SIGNATURE")
                    .and_then(|v| v.to_str().ok())
                    .ok_or_else(|| error::ErrorUnauthorized("Missing signature"))?
                    .to_string();
                (api_key, signature)
            };

            let key_type = if api_key_header == public_api_key {
                KeyType::Public
            } else if api_key_header == private_api_key {
                KeyType::Private
            } else {
                return Err(error::ErrorUnauthorized("Invalid API key"));
            };

            let (http_req, mut payload) = req.into_parts();

            let mut body_bytes = BytesMut::new();
            const MAX_SIZE: usize = 1024 * 1024; // 1 MB

            while let Some(chunk) = payload.next().await {
                let chunk = chunk
                    .map_err(|_| error::ErrorInternalServerError("Failed to read body"))?;
                if body_bytes.len() + chunk.len() > MAX_SIZE {
                    return Err(error::ErrorPayloadTooLarge("Payload too large"));
                }
                body_bytes.extend_from_slice(&chunk);
            }

            // Проверяем HMAC подпись (constant-time)
            let mut mac = HmacSha256::new_from_slice(secret_key.as_bytes())
                .expect("HMAC can take key of any size");
            mac.update(&body_bytes);

            let signature_bytes = general_purpose::STANDARD
                .decode(&signature_header)
                .map_err(|_| error::ErrorUnauthorized("Invalid signature encoding"))?;

            mac.verify_slice(&signature_bytes)
                .map_err(|_| error::ErrorUnauthorized("Invalid signature"))?;

            // Кладём тип ключа в extensions — хендлеры и guards могут его читать
            let req = ServiceRequest::from_parts(http_req, Payload::from(body_bytes.freeze()));
            req.extensions_mut().insert(key_type);

            service.call(req).await
        })
    }
}