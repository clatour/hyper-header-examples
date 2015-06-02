extern crate hyper;
extern crate time;
extern crate unicase;
extern crate cookie;

#[macro_use]
extern crate log;

extern crate env_logger;

mod accept {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Accept, qitem};
        use hyper::mime::{Mime, TopLevel, SubLevel};

        let mut headers = Headers::new();

        headers.set(
            Accept(vec![
                qitem(Mime(TopLevel::Text, SubLevel::Html, vec![])),
            ])
        );

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Accept, qitem};
        use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

        let mut headers = Headers::new();
        headers.set(
            Accept(vec![
                qitem(Mime(TopLevel::Application, SubLevel::Json, 
                           vec![(Attr::Charset, Value::Utf8)])),
            ])
        );

        info!("{}", headers);
        headers
    }

    pub fn ex3() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Accept, QualityItem, Quality, qitem};
        use hyper::mime::{Mime, TopLevel, SubLevel};

        let mut headers = Headers::new();

        headers.set(
            Accept(vec![
                qitem(Mime(TopLevel::Text, SubLevel::Html, vec![])),
                qitem(Mime(TopLevel::Application, SubLevel::Ext("xhtml+xml".to_owned()), vec![])),
                QualityItem::new(Mime(TopLevel::Application, SubLevel::Xml, vec![]), 
                                 Quality(900)),
                                 qitem(Mime(TopLevel::Image, SubLevel::Ext("webp".to_owned()), vec![])),
                                 QualityItem::new(Mime(TopLevel::Star, SubLevel::Star, vec![]), 
                                                  Quality(800))
            ])
        );

        info!("{}", headers);
        headers
    }
}

mod accept_charset {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AcceptCharset, Charset, qitem};

        let mut headers = Headers::new();
        headers.set(
            AcceptCharset(vec![qitem(Charset::Us_Ascii)])
        );

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AcceptCharset, Charset, Quality, QualityItem};

        let mut headers = Headers::new();
        headers.set(
            AcceptCharset(vec![
                QualityItem::new(Charset::Us_Ascii, Quality(900)),
                QualityItem::new(Charset::Iso_8859_10, Quality(200)),
            ])
        );

        info!("{}", headers);
        headers
    }

    pub fn ex3() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AcceptCharset, Charset, qitem};

        let mut headers = Headers::new();
        headers.set(
            AcceptCharset(vec![qitem(Charset::Ext("utf-8".to_owned()))])
        );

        info!("{}", headers);
        headers
    }
}

mod accept_encoding {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AcceptEncoding, Encoding, qitem};

        let mut headers = Headers::new();
        headers.set(
            AcceptEncoding(vec![qitem(Encoding::Chunked)])
        );

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AcceptEncoding, Encoding, qitem};
        
        let mut headers = Headers::new();
        headers.set(
            AcceptEncoding(vec![
                qitem(Encoding::Chunked),
                qitem(Encoding::Gzip),
                qitem(Encoding::Deflate),
            ])
        );

        info!("{}", headers);
        headers
    }

    pub fn ex3() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AcceptEncoding, Encoding, QualityItem, Quality, qitem};
        
        let mut headers = Headers::new();
        headers.set(
            AcceptEncoding(vec![
                qitem(Encoding::Chunked),
                QualityItem::new(Encoding::Gzip, Quality(600)),
                QualityItem::new(Encoding::EncodingExt("*".to_owned()), Quality(0)),
            ])
        );

        info!("{}", headers);
        headers
    }
}

mod accept_language {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AcceptLanguage, Language, qitem};

        let mut headers = Headers::new();
        headers.set(
            AcceptLanguage(vec![
                qitem(
                    Language { 
                         primary: "en".to_owned(),
                         sub: Some("us".to_owned()),
                    }
                ),
            ])
        );

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AcceptLanguage, Language, QualityItem, Quality, qitem};

        let mut headers = Headers::new();
        headers.set(
            AcceptLanguage(vec![
                qitem(
                    Language { 
                        primary: "da".to_owned(),
                        sub: None,
                     }
                ),
                QualityItem::new(
                    Language {
                        primary: "en".to_owned(),
                        sub: Some("gb".to_owned()),
                    },
                    Quality(800),
                ),
                QualityItem::new(
                    Language {
                        primary: "en".to_owned(),
                        sub: None,
                    },
                    Quality(700),
                ),
            ])
        );
        info!("{}", headers);
        headers
    }
}

mod accept_ranges {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AcceptRanges, RangeUnit};

        let mut headers = Headers::new();
        headers.set(AcceptRanges(vec![RangeUnit::Bytes]));

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AcceptRanges, RangeUnit};
        
        let mut headers = Headers::new();
        headers.set(AcceptRanges(vec![RangeUnit::None]));

        info!("{}", headers);
        headers
    }

    pub fn ex3() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AcceptRanges, RangeUnit};
        
        let mut headers = Headers::new();
        headers.set(
            AcceptRanges(vec![
                RangeUnit::Unregistered("nibbles".to_owned()),
                RangeUnit::Bytes,
                RangeUnit::Unregistered("doublets".to_owned()),
                RangeUnit::Unregistered("quadlets".to_owned()),
            ])
        );

        info!("{}", headers);
        headers
    }
}

mod access_control_allow_headers {
    
    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AccessControlAllowHeaders};
        use unicase::UniCase;
        
        let mut headers = Headers::new();
        headers.set(
            AccessControlAllowHeaders(vec![UniCase("date".to_owned())])
        );

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AccessControlAllowHeaders};
        use unicase::UniCase;
        
        let mut headers = Headers::new();
        headers.set(
            AccessControlAllowHeaders(vec![
                UniCase("accept-language".to_owned()),
                UniCase("date".to_owned()),
            ])
        );

        info!("{}", headers);
        headers
    }
}

mod access_control_allow_methods {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AccessControlAllowMethods};
        use hyper::method::Method;
        
        let mut headers = Headers::new();
        headers.set(
            AccessControlAllowMethods(vec![Method::Get])
        );

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AccessControlAllowMethods};
        use hyper::method::Method;
        
        let mut headers = Headers::new();
        headers.set(
            AccessControlAllowMethods(vec![
                Method::Get,
                Method::Post,
                Method::Patch,
                Method::Extension("COPY".to_owned()),
            ])
        );

        info!("{}", headers);
        headers
    }
}

mod access_control_allow_origin {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AccessControlAllowOrigin};
        
        let mut headers = Headers::new();
        headers.set(
            AccessControlAllowOrigin::Any
        );

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AccessControlAllowOrigin};
        
        let mut headers = Headers::new();
        headers.set(
            AccessControlAllowOrigin::Null, 
        );

        info!("{}", headers);
        headers
    }

    pub fn ex3() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AccessControlAllowOrigin};
        use hyper::Url;
        
        let mut headers = Headers::new();
        headers.set(
            AccessControlAllowOrigin::Value(Url::parse("http://hyper.rs").unwrap()) 
        );

        info!("{}", headers);
        headers
    }
}

mod access_control_max_age {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AccessControlMaxAge};
        
        let mut headers = Headers::new();
        headers.set(AccessControlMaxAge(1728000u32));

        info!("{}", headers);
        headers
    }
}

mod access_control_request_headers {
    
    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AccessControlRequestHeaders};
        use unicase::UniCase;
        
        let mut headers = Headers::new();
        headers.set(
            AccessControlRequestHeaders(vec![UniCase("date".to_owned())])
        );

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AccessControlRequestHeaders};
        use unicase::UniCase;
        
        let mut headers = Headers::new();
        headers.set(
            AccessControlRequestHeaders(vec![
                UniCase("accept-language".to_owned()),
                UniCase("date".to_owned()),
            ])
        );

        info!("{}", headers);
        headers
    }
}

mod access_control_request_method {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, AccessControlRequestMethod};
        use hyper::method::Method;
        
        let mut headers = Headers::new();
        headers.set(AccessControlRequestMethod(Method::Get));

        info!("{}", headers);
        headers
    }
}

mod allow {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Allow};
        use hyper::method::Method;
        
        let mut headers = Headers::new();
        headers.set(
            Allow(vec![Method::Get])
        );

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Allow};
        use hyper::method::Method;
        
        let mut headers = Headers::new();
        headers.set(
            Allow(vec![
                Method::Get,
                Method::Post,
                Method::Patch,
                Method::Extension("COPY".to_owned()),
            ])
        );

        info!("{}", headers);
        headers
    }
}

mod authorization {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Authorization};
        
        let mut headers = Headers::new();
        headers.set(Authorization("let me in".to_owned()));

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Authorization, Basic};
        
        let mut headers = Headers::new();
        headers.set(
            Authorization(
                Basic { 
                    username: "Aladdin".to_owned(), 
                    password: Some("open sesame".to_owned()) 
                }
            )
        );

        info!("{}", headers);
        headers
    }
}

mod cache_control {
    
    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, CacheControl, CacheDirective};
        
        let mut headers = Headers::new();
        headers.set(
            CacheControl(vec![CacheDirective::MaxAge(86400u32)])
        );

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, CacheControl, CacheDirective};
        
        let mut headers = Headers::new();
        headers.set(
            CacheControl(vec![
                CacheDirective::NoCache,
                CacheDirective::Private,
                CacheDirective::MaxAge(360u32),
                CacheDirective::Extension("foo".to_owned(), 
                                          Some("bar".to_owned())),
            ])
        );

        info!("{}", headers);
        headers
    }
}

mod connection {
    
    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Connection};
        
        let mut headers = Headers::new();
        headers.set(Connection::keep_alive());

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Connection, ConnectionOption};
        use unicase::UniCase;

        let mut headers = Headers::new();
        headers.set(
            Connection(vec![
                ConnectionOption::ConnectionHeader(UniCase("upgrade".to_owned())),
            ])
        );

        info!("{}", headers);
        headers
    }
}

mod content_encoding {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, ContentEncoding, Encoding};

        let mut headers = Headers::new();
        headers.set(ContentEncoding(vec![Encoding::Chunked]));

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, ContentEncoding, Encoding};
        
        let mut headers = Headers::new();
        headers.set(
            ContentEncoding(vec![
                Encoding::Gzip,
                Encoding::Chunked,
            ])
        );

        info!("{}", headers);
        headers
    }
}

mod content_language {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, ContentLanguage, Language, qitem};

        let mut headers = Headers::new();
        headers.set(
            ContentLanguage(vec![
                qitem(
                    Language { 
                         primary: "en".to_owned(),
                         sub: None,
                    }
                ),
            ])
        );

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, ContentLanguage, Language, qitem};

        let mut headers = Headers::new();
        headers.set(
            ContentLanguage(vec![
                qitem(
                    Language { 
                        primary: "da".to_owned(),
                        sub: None,
                     }
                ),
                qitem(
                    Language {
                        primary: "en".to_owned(),
                        sub: Some("gb".to_owned()),
                    }
                ),
            ])
        );

        info!("{}", headers);
        headers
    }
}

mod content_length {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, ContentLength};
        
        let mut headers = Headers::new();
        headers.set(ContentLength(1024u64));

        info!("{}", headers);
        headers
    }
}

mod content_type {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, ContentType};
        use hyper::mime::{Mime, TopLevel, SubLevel};

        let mut headers = Headers::new();

        headers.set(
            ContentType(Mime(TopLevel::Text, SubLevel::Html, vec![]))
        );

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, ContentType};
        use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};

        let mut headers = Headers::new();
        headers.set(
            ContentType(Mime(TopLevel::Application, SubLevel::Json, 
                           vec![(Attr::Charset, Value::Utf8)]))
           
        );

        info!("{}", headers);
        headers
    }
}

mod cookieheader {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Cookie};
        use cookie::Cookie as CookiePair;

        let mut headers = Headers::new();

        headers.set(
            Cookie(vec![
                CookiePair::new("foo".to_owned(), "bar".to_owned())
            ])
        );

        info!("{}", headers);
        headers
    }
}

mod date {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Date, HttpDate};
        use time;

        let mut headers = Headers::new();
        headers.set(Date(HttpDate(time::now())));

        info!("{}", headers);
        headers
    }
}

mod etag {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, ETag, EntityTag};

        let mut headers = Headers::new();
        headers.set(ETag(EntityTag::new(false, "xyzzy".to_owned())));

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, ETag, EntityTag};

        let mut headers = Headers::new();
        headers.set(ETag(EntityTag::new(true, "xyzzy".to_owned())));

        info!("{}", headers);
        headers
    }
}

mod expect {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Expect};
        
        let mut headers = Headers::new();
        headers.set(Expect::Continue);

        info!("{}", headers);
        headers
    }
}

mod expires {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Expires, HttpDate};
        use time::{self, Duration};

        let mut headers = Headers::new();
        headers.set(Expires(HttpDate(time::now() + Duration::days(1))));

        info!("{}", headers);
        headers
    }
}

mod from {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, From};

        let mut headers = Headers::new();
        headers.set(From("webmaster@example.org".to_owned()));

        info!("{}", headers);
        headers
    }
}

mod host {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Host};

        let mut headers = Headers::new();
        headers.set(
            Host{ 
                hostname: "hyper.rs".to_owned(),
                port: None,
            }
        );

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Host};

        let mut headers = Headers::new();
        headers.set(
            Host{ 
                hostname: "hyper.rs".to_owned(),
                port: Some(8080),
            }
        );

        info!("{}", headers);
        headers
    }
}

mod if_match {
    
    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, IfMatch};

        let mut headers = Headers::new();
        headers.set(IfMatch::Any);

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, IfMatch, EntityTag};

        let mut headers = Headers::new();
        headers.set(
            IfMatch::Items(vec![
                EntityTag::new(false, "xyzzy".to_owned()),
                EntityTag::new(false, "foobar".to_owned()),
                EntityTag::new(false, "bazquux".to_owned()),
            ])
        );

        info!("{}", headers);
        headers
    }
}

mod if_modified_since {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, IfModifiedSince, HttpDate};
        use time::{self, Duration};

        let mut headers = Headers::new();
        headers.set(IfModifiedSince(HttpDate(time::now() - Duration::days(1))));

        info!("{}", headers);
        headers
    }
}

mod if_none_match {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, IfNoneMatch};

        let mut headers = Headers::new();
        headers.set(IfNoneMatch::Any);

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, IfNoneMatch, EntityTag};

        let mut headers = Headers::new();
        headers.set(
            IfNoneMatch::Items(vec![
                EntityTag::new(false, "xyzzy".to_owned()),
                EntityTag::new(false, "foobar".to_owned()),
                EntityTag::new(false, "bazquux".to_owned()),
            ])
        );

        info!("{}", headers);
        headers
    }
}

mod if_range {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, IfRange, HttpDate};
        use time::{self, Duration};

        let mut headers = Headers::new();
        headers.set(IfRange::Date(HttpDate(time::now() - Duration::days(1))));

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, IfRange, EntityTag};

        let mut headers = Headers::new();
        headers.set(IfRange::EntityTag(EntityTag::new(false, "xyzzy".to_owned())));

        info!("{}", headers);
        headers
    }
}

mod if_unmodified_since {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, IfUnmodifiedSince, HttpDate};
        use time::{self, Duration};

        let mut headers = Headers::new();
        headers.set(IfUnmodifiedSince(HttpDate(time::now() - Duration::days(1))));

        info!("{}", headers);
        headers
    }
}

mod last_modified {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, LastModified, HttpDate};
        use time::{self, Duration};

        let mut headers = Headers::new();
        headers.set(LastModified(HttpDate(time::now() - Duration::days(1))));

        info!("{}", headers);
        headers
    }
}

mod location {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Location};

        let mut headers = Headers::new();
        headers.set(Location("/People.html#tim".to_owned()));

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Location};

        let mut headers = Headers::new();
        headers.set(Location("http://www.example.com/index.html".to_owned()));

        info!("{}", headers);
        headers
    }
}

mod pragma {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Pragma};

        let mut headers = Headers::new();
        headers.set(Pragma::NoCache);

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Pragma};

        let mut headers = Headers::new();
        headers.set(Pragma::Ext("foobar".to_owned()));

        info!("{}", headers);
        headers
    }
}

mod referer {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Referer};

        let mut headers = Headers::new();
        headers.set(Referer("/People.html#tim".to_owned()));

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Referer};

        let mut headers = Headers::new();
        headers.set(Referer("http://www.example.com/index.html".to_owned()));

        info!("{}", headers);
        headers
    }
}

mod server {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Server};

        let mut headers = Headers::new();
        headers.set(Server("hyper/0.5.2".to_owned()));

        info!("{}", headers);
        headers
    }
}

mod set_cookie {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, SetCookie};
        use cookie::Cookie as CookiePair;

        let mut headers = Headers::new();
        let mut cookie = CookiePair::new("foo".to_owned(), "bar".to_owned());

        cookie.path = Some("/path".to_owned());
        cookie.domain = Some("example.com".to_owned());

        headers.set(
            SetCookie(vec![
                cookie,
                CookiePair::new("baz".to_owned(), "quux".to_owned()),
            ])
        );

        info!("{}", headers);
        headers
    }
}

mod transfer_encoding {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, TransferEncoding, Encoding};
        
        let mut headers = Headers::new();
        headers.set(
            TransferEncoding(vec![
                Encoding::Gzip,
                Encoding::Chunked,
            ])
        );

        info!("{}", headers);
        headers
    }
}

mod upgrade {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Upgrade, Protocol, ProtocolName};
        
        let mut headers = Headers::new();
        headers.set(Upgrade(vec![Protocol::new(ProtocolName::WebSocket, None)]));

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Upgrade, Protocol, ProtocolName};
        
        let mut headers = Headers::new();
        headers.set(
            Upgrade(vec![
                Protocol::new(ProtocolName::Http, Some("2.0".to_owned())),
                Protocol::new(ProtocolName::Unregistered("SHTTP".to_owned()), Some("1.3".to_owned())),
                Protocol::new(ProtocolName::Unregistered("IRC".to_owned()), Some("6.9".to_owned())),
                Protocol::new(ProtocolName::Unregistered("RTA".to_owned()), Some("x11".to_owned())),
            ])
        );

        info!("{}", headers);
        headers
    }
}

mod user_agent {
    
    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, UserAgent};
        
        let mut headers = Headers::new();
        headers.set(UserAgent("hyper/0.5.2".to_owned()));

        info!("{}", headers);
        headers
    }
}

mod vary {

    pub fn ex1() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Vary};

        let mut headers = Headers::new();
        headers.set(Vary::Any);

        info!("{}", headers);
        headers
    }

    pub fn ex2() -> ::hyper::header::Headers {
        use hyper::header::{Headers, Vary};
        use unicase::UniCase;

        let mut headers = Headers::new();
        headers.set(
            Vary::Items(vec![
                UniCase("accept-encoding".to_owned()),
                UniCase("accept-language".to_owned()),
            ])
        );

        info!("{}", headers);
        headers
    }
}


fn main() {
    env_logger::init().unwrap();

    accept::ex1();
    accept::ex2();
    accept::ex3();

    accept_charset::ex1();
    accept_charset::ex2();
    accept_charset::ex3();

    accept_encoding::ex1();
    accept_encoding::ex2();
    accept_encoding::ex3();

    accept_language::ex1();
    accept_language::ex2();

    accept_ranges::ex1();
    accept_ranges::ex2();
    accept_ranges::ex3();

    access_control_allow_headers::ex1();
    access_control_allow_headers::ex2();

    access_control_allow_methods::ex1();
    access_control_allow_methods::ex2();

    access_control_allow_origin::ex1();
    access_control_allow_origin::ex2();
    access_control_allow_origin::ex3();

    access_control_max_age::ex1();
    
    access_control_request_headers::ex1();
    access_control_request_headers::ex2();

    access_control_request_method::ex1();

    allow::ex1();
    allow::ex2();

    authorization::ex1();
    authorization::ex2();

    cache_control::ex1();
    cache_control::ex2();

    connection::ex1();
    connection::ex2();

    content_encoding::ex1();
    content_encoding::ex2();

    content_language::ex1();
    content_language::ex2();

    content_length::ex1();

    content_type::ex1();
    content_type::ex2();

    cookieheader::ex1();

    date::ex1();

    etag::ex1();
    etag::ex2();

    expect::ex1();

    expires::ex1();

    from::ex1();

    host::ex1();
    host::ex2();
    
    if_match::ex1();
    if_match::ex2();

    if_modified_since::ex1();

    if_none_match::ex1();
    if_none_match::ex2();

    if_range::ex1();
    if_range::ex2();

    if_unmodified_since::ex1();

    last_modified::ex1();

    location::ex1();
    location::ex2();

    pragma::ex1();
    pragma::ex2();

    referer::ex1();
    referer::ex2();
    
    server::ex1();

    set_cookie::ex1();

    transfer_encoding::ex1();

    upgrade::ex1();
    upgrade::ex2();

    user_agent::ex1();

    vary::ex1();
    vary::ex2();
}
