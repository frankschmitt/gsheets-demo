// This is a modified version of the example at:
// https://github.com/Byron/google-apis-rs/tree/main/gen/sheets4

extern crate google_sheets4 as sheets4;
extern crate hyper;
extern crate hyper_rustls;
extern crate yup_oauth2 as oauth2;
use sheets4::Error;
use sheets4::Sheets;
use sheets4::api::{ValueRange, Spreadsheet};
use hyper::{Body, Client, StatusCode, Uri};
use chrono::prelude::*;

// excerpt from: https://github.com/Byron/google-apis-rs/blob/main/gen/sheets4-cli/src/main.rs
/*async fn _spreadsheets_values_update(&self, opt: &ArgMatches<'n>, dry_run: bool, err: &mut InvalidOptionsError)
                                                    -> Result<(), DoitError> {
        
        let mut field_cursor = FieldCursor::default();
        let mut object = json::value::Value::Object(Default::default());
        
        for kvarg in opt.values_of("kv").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let last_errc = err.issues.len();
            let (key, value) = parse_kv_arg(&*kvarg, err, false);
            let mut temp_cursor = field_cursor.clone();
            if let Err(field_err) = temp_cursor.set(&*key) {
                err.issues.push(field_err);
            }
            if value.is_none() {
                field_cursor = temp_cursor.clone();
                if err.issues.len() > last_errc {
                    err.issues.remove(last_errc);
                }
                continue;
            }
        
            let type_info: Option<(&'static str, JsonTypeInfo)> =
                match &temp_cursor.to_string()[..] {
                    "major-dimension" => Some(("majorDimension", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    "range" => Some(("range", JsonTypeInfo { jtype: JsonType::String, ctype: ComplexType::Pod })),
                    _ => {
                        let suggestion = FieldCursor::did_you_mean(key, &vec!["major-dimension", "range"]);
                        err.issues.push(CLIError::Field(FieldError::Unknown(temp_cursor.to_string(), suggestion, value.map(|v| v.to_string()))));
                        None
                    }
                };
            if let Some((field_cursor_str, type_info)) = type_info {
                FieldCursor::from(field_cursor_str).set_json_value(&mut object, value.unwrap(), type_info, err, &temp_cursor);
            }
        }
        let mut request: api::ValueRange = json::value::from_value(object).unwrap();
        let mut call = self.hub.spreadsheets().values_update(request, opt.value_of("spreadsheet-id").unwrap_or(""), opt.value_of("range").unwrap_or(""));
        for parg in opt.values_of("v").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
            let (key, value) = parse_kv_arg(&*parg, err, false);
            match key {
                "value-input-option" => {
                    call = call.value_input_option(value.unwrap_or(""));
                },
                "response-value-render-option" => {
                    call = call.response_value_render_option(value.unwrap_or(""));
                },
                "response-date-time-render-option" => {
                    call = call.response_date_time_render_option(value.unwrap_or(""));
                },
                "include-values-in-response" => {
                    call = call.include_values_in_response(arg_from_str(value.unwrap_or("false"), err, "include-values-in-response", "boolean"));
                },
                _ => {
                    let mut found = false;
                    for param in &self.gp {
                        if key == *param {
                            found = true;
                            call = call.param(self.gpm.iter().find(|t| t.0 == key).unwrap_or(&("", key)).1, value.unwrap_or("unset"));
                            break;
                        }
                    }
                    if !found {
                        err.issues.push(CLIError::UnknownParameter(key.to_string(),
                                                                  {let mut v = Vec::new();
                                                                           v.extend(self.gp.iter().map(|v|*v));
                                                                           v.extend(["include-values-in-response", "response-date-time-render-option", "response-value-render-option", "value-input-option"].iter().map(|v|*v));
                                                                           v } ));
                    }
                }
            }
        }
        let protocol = CallType::Standard;
        if dry_run {
            Ok(())
        } else {
            assert!(err.issues.len() == 0);
            for scope in self.opt.values_of("url").map(|i|i.collect()).unwrap_or(Vec::new()).iter() {
                call = call.add_scope(scope);
            }
            let mut ostream = match writer_from_opts(opt.value_of("out")) {
                Ok(mut f) => f,
                Err(io_err) => return Err(DoitError::IoError(opt.value_of("out").unwrap_or("-").to_string(), io_err)),
            };
            match match protocol {
                CallType::Standard => call.doit().await,
                _ => unreachable!()
            } {
                Err(api_err) => Err(DoitError::ApiError(api_err)),
                Ok((mut response, output_schema)) => {
                    let mut value = json::value::to_value(&output_schema).expect("serde to work");
                    remove_json_null_values(&mut value);
                    json::to_writer_pretty(&mut ostream, &value).unwrap();
                    ostream.flush().unwrap();
                    Ok(())
                }
            }
        }
    }
*/

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
    // set data for insert
    let mut req = ValueRange::default();
    let local = Local::now();
    let val = local.format("%Y-%m-%d %H:%M:%S").to_string();
    req.values = Some(vec![vec![val]]);

    //let object = json!(12);
    //let mut req: ValueRange = json::value::from_value(object).unwrap();
    //let mut call = hub.spreadsheets().values_append(request, opt.value_of("spreadsheet-id").unwrap_or(""), opt.value_of("range").unwrap_or(""));
    
    // You can configure optional parameters by calling the respective setters at will, and
    // execute the final call using `doit()`.
    // Values shown here are possibly random and not representative !
    let create_response = hub.spreadsheets().create(Spreadsheet::default()).doit().await;
    //let ss_id = ss.unwrap().get("spreadsheetId");
    //println!("created spreadsheet, response {:?}", create_response);
    let spreadsheet_id = match create_response {
        Err(res) => { 
            println!("ERROR: {:?}", res);
            // use hard-coded spreadsheet id
            "1WQSoh6FsAPPDF49Idl1x3BQT7gAGCwgaBINwyphTLFE".to_string()
        },
        Ok(res) => {
            println!("Success: {:?}", res);
            // we get a tuple of (body, Spreadsheet); the body doesn't concern us, get just the spreadsheet
            let (_body, sheet) = res;
            println!("id: {:?}", sheet.spreadsheet_id);
            sheet.spreadsheet_id.unwrap()
            //"1WQSoh6FsAPPDF49Idl1x3BQT7gAGCwgaBINwyphTLFE".to_string();
        }
 
    };

    // the ID can be obtained from the URL in GSheets, e.g.
    //    https://docs.google.com/spreadsheets/d/1WQSoh6FsAPPDF49Idl1x3BQT7gAGCwgaBINwyphTLFE/edit#gid=0 => 1WQSoh6FsAPPDF49Idl1x3BQT7gAGCwgaBINwyphTLFE
    // the option values are available at https://developers.google.com/sheets/api/reference/rest/v4/ValueInputOption
    //let append_response = hub.spreadsheets().values_append(req, "1WQSoh6FsAPPDF49Idl1x3BQT7gAGCwgaBINwyphTLFE", "Sheet1!A2:E")
    let append_response = hub.spreadsheets().values_append(req, &spreadsheet_id , "Sheet1!A2:E")
             .value_input_option("RAW")
             .response_value_render_option("FORMATTED_VALUE")
             .response_date_time_render_option("SERIAL_NUMBER")
             //.insert_data_option("gubergren")
             .include_values_in_response(true)
             .doit().await;
    
    
    println!("appended values to spreadsheet {:?}, URL: https://docs.google.com/spreadsheets/d/{}", spreadsheet_id, spreadsheet_id);
    match append_response {
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