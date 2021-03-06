
extern crate cookie;
extern crate sapper;

use sapper::header::{Cookie, SetCookie};
use sapper::{Request, Response, Result, Key};
use cookie::Cookie as Cookie_M;


pub struct SessionVal;
impl Key for SessionVal { type Value = String; }

pub fn session_val(req: &mut Request, ckey: &'static str) -> Result<()> {
    
    let mut session_value: Option<String> = None;
    match req.headers().get::<Cookie>() {
        Some(cookie_headers) => {
            
            //let mut cookie_jar = CookieJar::new();
            for header in cookie_headers.iter() {
                let raw_str = match ::std::str::from_utf8(&header.as_bytes()) {
                    Ok(string) => string,
                    Err(_) => continue
                };

                for cookie_str in raw_str.split(";").map(|s| s.trim()) {
                    if let Ok(cookie) = Cookie_M::parse(cookie_str) {
                        if cookie.name() == ckey {
                            session_value = Some(cookie.value().to_owned());
                            break;
                        }
                    }
                }
            }
        },
        None => {
            println!("no cookie in headers");
        }
    }
    
    session_value.and_then(|val| {
        req.ext_mut().insert::<SessionVal>(val);
        Some(())
    });

    Ok(())
}

// library function
pub fn set_cookie(res: &mut Response, ckey: String, val: String, domain: Option<String>, path: Option<String>, secure: Option<bool>, max_age: Option<u64>) -> Result<()> {
    let mut cookie = Cookie_M::new(ckey, val);
    if domain.is_some() {
        cookie.set_domain(domain.unwrap());
    }
    if path.is_some() {
        cookie.set_path(path.unwrap());
    }
    if secure.is_some() {
        cookie.set_secure(secure.unwrap());
    }
    
    println!("{:?}", cookie);
    
    res.headers_mut().set(SetCookie(vec![cookie.to_string()]));
    
    Ok(())
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
