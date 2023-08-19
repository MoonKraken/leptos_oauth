//exchange token for auth code
use cfg_if::cfg_if;
use leptos::*;
use leptos::ServerFnError::ServerError;

use crate::model::{AuthContext, TokenClaims};

// the consuming application is responsible for persisting the refresh token
// they need to give us a function for retrieving it so we can use it to get
// another access token from the oauth provider
// TODO what user identifier do we use to look up the token?
#[server(RefreshAccessToken, "/api")]
pub async fn refresh_access_token() -> Result<(), ServerFnError> {
    //TODO
    Ok(())
}

// This is the API we call once we receive an auth code in the callback URL
// if all goes well we should get an access and refresh token
// we'll tell the browser to save the access token to a cookie
// TODO we need to provide a means of having the application tell us how to persist the refresh token
// and perform that as well, so the frontend can invoke `refresh_access_token()` to get a new one
#[server(TokenRequest, "/api")]
pub async fn token_request(code: String) -> Result<(), ServerFnError> {
    use jsonwebtoken_google::Parser;
    use http::{header, HeaderName, HeaderValue};
    use leptos_actix::ResponseOptions;
    use oauth2::reqwest::async_http_client;
    use oauth2::AuthorizationCode;
    use oauth2::{AuthType, TokenResponse};

    let client = get_oauth_client()?;
    log!("Requesting tokens in using code {}", code);

    let token_result = client
        .set_auth_type(AuthType::RequestBody)
        .exchange_code(AuthorizationCode::new(code))
        .request_async(async_http_client)
        .await?;
        // Err(e) => {
        //     error!("TokenResponse Error: {:#?}", e);
        //     ServerFnError("oh no")
        // }
    let access_token = token_result.access_token().secret();
    log!("Access Token: {:#?}", &access_token);
    log!("Token Response: {:?}", token_result);
    // let mut cookie = Cookie::new("access_token", access_token.secret());
    // cookie.set_http_only(true);
    // cookie.set_same_site(SameSite::Lax);
    // let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) else {
    //         return Ok(false);
    //     };
    log!("Adding cookie");
    let header_value = HeaderValue::from_str(&format!("access_token={access_token}"))?;
    let response = expect_context::<ResponseOptions>();
    response.append_header(header::SET_COOKIE, header_value);
    log!("Cookie added");

    let parser = Parser::new(&dotenvy::var("OAUTH2_CLIENT_ID").unwrap());
    let claims = parser.parse::<TokenClaims>(access_token).await.unwrap();

    dbg!(claims);
    Ok(())
}

#[cfg(feature = "ssr")]
pub fn get_oauth_client() -> Result<oauth2::basic::BasicClient, ServerFnError> {
    use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

    let client = oauth2::basic::BasicClient::new(
        ClientId::new(dotenvy::var("OAUTH2_CLIENT_ID").unwrap()),
        Some(ClientSecret::new(
            dotenvy::var("OAUTH2_CLIENT_SECRET").unwrap(),
        )),
        AuthUrl::new(dotenvy::var("OAUTH2_AUTH_URI").unwrap())
            .map_err(|pe| ServerError(pe.to_string()))?,
        Some(TokenUrl::new(dotenvy::var("OAUTH2_TOKEN_URI").unwrap())?),
    )
    .set_redirect_uri(
        RedirectUrl::new(dotenvy::var("OAUTH2_REDIRECT_URI").unwrap())
            .map_err(|pe| ServerError(pe.to_string()))?,
    );
    Ok(client)
}

#[server(GetLoginUrl, "/api")]
pub async fn get_login_url() -> Result<String, ServerFnError> {
    use oauth2::{CsrfToken, Scope};
    let scopes = dotenvy::var("OAUTH2_SCOPES").unwrap();

    let client = get_oauth_client()?;
    let (auth_url, _csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(scopes.split(" ").map(|scope| Scope::new(scope.into())))
        .add_extra_param("access_type", "offline")
        .url();

    Ok(auth_url.to_string())
}
