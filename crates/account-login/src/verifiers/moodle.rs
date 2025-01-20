use crate::verifiers::LoginVerifier;
use crate::Error;
use reqwest::header::HeaderValue;
use std::collections::HashMap;

pub struct Moodle<'u> {
    url: &'u str,
}

impl<'u> Moodle<'u> {
    pub fn new(url: &'u str) -> Self {
        Moodle { url }
    }
}

impl LoginVerifier for Moodle<'_> {
    async fn verify_login<'m>(
        &'m self,
        username: &'m str,
        password: &'m str,
    ) -> Result<bool, Error<'m>> {
        moodle_login(self.url, username, password).await
    }
}

#[allow(unused)]
async fn moodle_login<'m>(
    url: &'m str,
    username: &'m str,
    password: &'m str,
) -> Result<bool, Error<'m>> {
    let token = moodle_tokens(url).await?;

    #[cfg(test)]
    {
        println!("Token and Cookies: {:?}", token);
    }

    let mut endpoint = reqwest::Url::parse(url).expect("Error parsing the moodle url");

    let mut request = reqwest::Request::new(reqwest::Method::POST, endpoint.clone());

    let body_string = format!(
        "anchor=&logintoken={}&username={}&password={}",
        token.0, username, password
    );

    request.body_mut().replace(reqwest::Body::from(body_string));

    let headers = request.headers_mut();

    headers.clear();

    headers.insert(
        reqwest::header::CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded"),
    );
    headers.insert(
        reqwest::header::USER_AGENT,
        HeaderValue::from_static("chandocar/0.1.0"),
    );

    let cookies_string = token
        .1
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join(";");

    headers.insert(
        reqwest::header::COOKIE,
        HeaderValue::from_str(&cookies_string).unwrap(),
    );

    let client = reqwest::Client::new();

    let response = client
        .execute(request)
        .await
        .map_err(|e| Error::NetworkError("Error trying the login POST"))?;

    let status_code = response.status().as_u16();

    if status_code != 200 {
        return Err(Error::UnexpectedResponse("Expected status 200"));
    }

    let result = response
        .url()
        .as_str()
        .starts_with(&format!("{}?testsession=", url));

    Ok(result)
}

/// The first return param is the login token, and the second is a hashmap with the cookies
async fn moodle_tokens(url: &str) -> Result<(String, HashMap<String, String>), Error> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await.expect("Invalid url provided");

    let mut cookies = HashMap::new();

    response
        .cookies()
        .try_for_each(|cookie| {
            cookies.insert(cookie.name().to_string(), cookie.value().to_string());

            Ok::<(), ()>(())
        })
        .expect("Error while reading the cookies from the moodle url");

    let body = response
        .text()
        .await
        .map_err(|_| Error::NetworkError("Error trying to GET the session"))?;

    let token = body.split("logintoken\" value=\"").collect::<Vec<&str>>()[1]
        .split("\"")
        .collect::<Vec<&str>>()[0];

    Ok((token.to_string(), cookies))
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOODLE_LOGIN_URL: &str =
        "https://www.edu.xunta.gal/centros/ieschanmonte/aulavirtual/login/index.php";

    #[tokio::test]
    async fn test_moodle_login() {
        todo!("Set a valid user");
        assert!(
            moodle_login(MOODLE_LOGIN_URL, "", "")
                .await
                .unwrap()
        );
        assert!(
            !moodle_login(MOODLE_LOGIN_URL, "bcastextrsdf", "po8yhpñoiuhñ")
                .await
                .unwrap()
        );
    }

    #[tokio::test]
    async fn stress_moodle_login() {
        let mut threads = Vec::new();

        for _ in 0..50 {
            let handle = tokio::spawn(async {
                let response = moodle_login(MOODLE_LOGIN_URL, "pepito", "adsuigflasjd").await;

                if let Ok(result) = response {
                    println!("Result: {}. Expected: false", result);
                    assert!(!result);
                } else { 
                    println!("Error: {:?}", response);
                    assert!(false);
                }
            });

            threads.push(handle);

            let handle = tokio::spawn(async {
                todo!("Set a valid user");
                let response = moodle_login(MOODLE_LOGIN_URL, "", "").await;

                if let Ok(result) = response {
                    println!("Result: {}. Expected: true", result);
                    assert!(result);
                } else {
                    println!("Error: {:?}", response);
                    assert!(false);
                }
            });

            threads.push(handle);
        }

        for handle in threads {
            handle.await.unwrap();
        }
    }
}
