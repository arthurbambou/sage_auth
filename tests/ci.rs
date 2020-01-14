use sage_auth::auth::AuthenticateBuilder;
use sage_auth::invalidate::InvalidateBuilder;
use sage_auth::refresh::RefreshBuilder;
use sage_auth::validate::ValidateBuilder;

use std::env;

macro_rules! assert_ok {
    ($result:expr) => {{
        let result = &$result;
        assert!(result.is_ok(), "got error: {:?}", result);
    }};
}

#[tokio::test]
async fn test_all() {
    let r = AuthenticateBuilder::new()
        .username(&env::var("MOJANG_ACCOUNT").expect("MOJANG_ACCOUNT is not set"))
        .password(&env::var("MOJANG_PASSWORD").expect("MOJANG_PASSWORD is not set"))
        .request_user()
        .request()
        .await;
    assert_ok!(r);

    let resp = r.unwrap();
    assert!(resp.user.is_some());

    assert_ok!(
        ValidateBuilder::new()
            .access_token(&resp.access_token)
            .client_token(resp.client_token)
            .request()
            .await
    );

    assert_ok!(
        RefreshBuilder::new()
            .access_token(&resp.access_token)
            .client_token(resp.client_token)
            .request()
            .await
    );

    assert_ok!(
        InvalidateBuilder::new()
            .access_token(&resp.access_token)
            .client_token(resp.client_token)
            .request()
            .await
    );

    assert!(ValidateBuilder::new()
        .access_token(&resp.access_token)
        .client_token(resp.client_token)
        .request()
        .await
        .is_err());
}
