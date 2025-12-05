use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpResponse,
};
use futures::future::LocalBoxFuture;
use std::future::{ready, Ready};

pub struct CorsMiddleware;

impl<S, B> Transform<S, ServiceRequest> for CorsMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = CorsMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CorsMiddlewareService { service }))
    }
}

pub struct CorsMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CorsMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Handle OPTIONS preflight requests
        if req.method() == actix_web::http::Method::OPTIONS {
            let response = HttpResponse::Ok()
                .insert_header((header::ACCESS_CONTROL_ALLOW_ORIGIN, "*"))
                .insert_header((header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, PUT, DELETE, OPTIONS"))
                .insert_header((header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type, Authorization"))
                .insert_header((header::ACCESS_CONTROL_MAX_AGE, "3600"))
                .finish();
            
            return Box::pin(async move {
                Ok(ServiceResponse::new(req.into_parts().0, response))
            });
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            
            // Add CORS headers to response
            res.headers_mut().insert(
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                header::HeaderValue::from_static("*"),
            );
            res.headers_mut().insert(
                header::ACCESS_CONTROL_ALLOW_METHODS,
                header::HeaderValue::from_static("GET, POST, PUT, DELETE, OPTIONS"),
            );
            res.headers_mut().insert(
                header::ACCESS_CONTROL_ALLOW_HEADERS,
                header::HeaderValue::from_static("Content-Type, Authorization"),
            );
            
            Ok(res)
        })
    }
}
