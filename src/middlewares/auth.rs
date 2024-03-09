// use crate::models::person::Claims;
// use actix_web::{
//     dev,
//     error::ErrorUnauthorized,
//     Error,
//     FromRequest,
//     HttpRequest,
// };
// use futures::future::{err, ok, Ready};
// use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
// use crate::env;
//
// pub struct AuthorizationService;
//
// impl FromRequest for AuthorizationService {
//     type Error = Error;
//     type Future = Ready<Result<AuthorizationService, Error>>;
//
//     fn from_request(_req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
//         let _auth = _req.headers().get("Authorization");
//         match _auth {
//             Some(_) => {
//                 let _split: Vec<&str> = _auth.unwrap().to_str().unwrap().split("Bearer").collect();
//                 let token = _split[1].trim();
//                 let _var = env::var("SECRET_KEY").expect("Failed to get secret key.");
//                 let key = _var.as_bytes();
//                 match decode::<Claims>(
//                     token,
//                     &DecodingKey::from_secret(key),
//                     &Validation::new(Algorithm::HS256),
//                 ) {
//                     Ok(_token) => ok(AuthorizationService),
//                     Err(_e) => err(ErrorUnauthorized("invalid token!")),
//                 }
//             }
//             None => err(ErrorUnauthorized("blocked!")),
//         }
//     }
// }
