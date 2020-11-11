use clap::{
    app_from_crate, crate_authors, crate_description, crate_name, crate_version, App, Arg,
    ArgGroup, ArgMatches,
};

use crate::fetch_ip_addr;
use crate::ip_addr_client::prelude::*;

pub fn get_clap_app() -> App<'static, 'static> {
    let app = app_from_crate!()
        .arg(
            Arg::with_name("aws")
                .short("A")
                .long("aws")
                .help("fetch for checkip.amazonaws.com"),
        )
        .arg(
            Arg::with_name("httpbin")
                .short("B")
                .long("httpbin")
                .help("fetch for httpbin.org/ip"),
        )
        .arg(
            Arg::with_name("ifconfig")
                .short("C")
                .long("ifconfig")
                .help("fetch for ifconfig.me"),
        )
        .group(ArgGroup::with_name("hosts").args(&["aws", "httpbin", "ifconfig"]))
        .arg(
            Arg::with_name("ssl")
                .short("s")
                .long("ssl")
                .help("fetch using SSL"),
        )
        .arg(
            Arg::with_name("no-ssl")
                .short("n")
                .long("no-ssl")
                .help("fetch using no SSL"),
        )
        .group(ArgGroup::with_name("use-ssl").args(&["ssl", "no-ssl"]))
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("output verbose"),
        );

    app
}

pub fn from_ipaddr_client(matches: &ArgMatches) -> Box<dyn IpAddrClient> {
    let verbose = matches.is_present("verbose");

    let use_ssl = UseSsl::from(matches.is_present("ssl"), matches.is_present("no-ssl"));
    if verbose {
        eprintln!("Use SSL: {:?}", use_ssl);
    }

    let hosts = Host::from(
        matches.is_present("aws"),
        matches.is_present("httpbin"),
        matches.is_present("if_config"),
    );

    if verbose {
        eprintln!("Host: {:?}", hosts);
    }

    match (hosts, use_ssl) {
        (Host::AmazonAws, UseSsl::Ssl) => Box::new(AmazonAws::new_https()),
        (Host::AmazonAws, UseSsl::NoSsl) => Box::new(AmazonAws::new_http()),
        (Host::AmazonAws, UseSsl::Undefined) => Box::new(AmazonAws::new_http()),
        (Host::HttpBin, UseSsl::Ssl) => Box::new(HttpBin::new_https()),
        (Host::HttpBin, UseSsl::NoSsl) => Box::new(HttpBin::new_http()),
        (Host::HttpBin, UseSsl::Undefined) => Box::new(HttpBin::new_http()),
        (Host::IfConfig, UseSsl::Ssl) => Box::new(IfConfig::new_https()),
        (Host::IfConfig, UseSsl::NoSsl) => Box::new(IfConfig::new_http()),
        (Host::IfConfig, UseSsl::Undefined) => Box::new(IfConfig::new_http()),
        (Host::Undefined, UseSsl::Ssl) => fetch_ip_addr::random_client_https(),
        (Host::Undefined, UseSsl::NoSsl) => fetch_ip_addr::random_client_http(),
        (Host::Undefined, UseSsl::Undefined) => fetch_ip_addr::random_client(),
    }
}

#[derive(Debug)]
enum UseSsl {
    Undefined,
    NoSsl,
    Ssl,
}

impl UseSsl {
    fn from(ssl: bool, no_ssl: bool) -> Self {
        match (ssl, no_ssl) {
            (true, false) => UseSsl::Ssl,
            (false, true) => UseSsl::NoSsl,
            (false, false) => UseSsl::Undefined,
            (true, true) => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Host {
    Undefined,
    AmazonAws,
    IfConfig,
    HttpBin,
}

impl Host {
    fn from(amazon_aws: bool, http_bin: bool, if_config: bool) -> Self {
        match (amazon_aws, http_bin, if_config) {
            (true, false, false) => Host::AmazonAws,
            (false, true, false) => Host::HttpBin,
            (false, false, true) => Host::IfConfig,
            (false, false, false) => Host::Undefined,
            _ => unreachable!(),
        }
    }
}
