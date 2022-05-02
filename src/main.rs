// This is a modified version of the example at:
// https://github.com/Byron/google-apis-rs/tree/main/gen/sheets4

extern crate google_sheets4 as sheets4;
extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
use sheets4::Error;
use sheets4::Sheets;
use sheets4::api::ValueRange;
use hyper::{Body, Client, StatusCode, Uri};

#[tokio::main]
async fn main() {
    // Get an ApplicationSecret instance by some means. It contains the `client_id` and
    // `client_secret`, among other things.
    println!("reading secret");
    let secret = yup_oauth2::read_application_secret("clientsecret.json")
        .await
        .expect("client secret could not be read");

    println!("successfully read secret, authenticating");
    // Instantiate the authenticator. It will choose a suitable authentication flow for you,
    // unless you replace  `None` with the desired Flow.
    // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
    // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
    // retrieve them from storage.
    let auth = yup_oauth2::InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk("tokencache.json")
    .build()
    .await
    .unwrap();

    println!("built auth");
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
      .with_native_roots()
      .https_only()
      .enable_http1()
      .build();
    let client: Client <_, hyper::Body> = hyper::Client::builder().build(connector);
    println!("built client");
    let mut hub = Sheets::new(client, auth);
    println!("created hub");
    // As the method needs a request, you would usually fill it with the desired information
    // into the respective structure. Some of the parts shown here might not be applicable !
    // Values shown here are possibly random and not representative !
    let mut req = ValueRange::default();

    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !
    let result = hub.spreadsheets().values_append(req, "spreadsheetId", "range")
             .value_input_option("amet.")
             .response_value_render_option("duo")
             .response_date_time_render_option("ipsum")
             .insert_data_option("gubergren")
             .include_values_in_response(true)
             .doit().await;

    println!("appended values to spreadsheet");
    match result {
        Err(e) => match e {
            // The Error enum provides details about what exactly happened.
            // You can also just use its `Debug`, `Display` or `Error` traits
            Error::HttpError(_)
        |Error::Io(_)
        |Error::MissingAPIKey
        |Error::MissingToken(_)
        |Error::Cancelled
        |Error::UploadSizeLimitExceeded(_, _)
        |Error::Failure(_)
        |Error::BadRequest(_)
        |Error::FieldClash(_)
        |Error::JsonDecodeError(_, _) => println!("{}", e),
        },
        Ok(res) => println!("Success: {:?}", res),
    }
}