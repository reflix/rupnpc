use std::collections::HashMap;
use std::fmt::Formatter;
use std::str::FromStr;
use std::time::Duration;
use futures::TryStreamExt;
use rupnp::ssdp::{SearchTarget, URN};
use structopt::StructOpt;
use anyhow::{Context, Result};
use rt_format::{Format, FormatArgument, ParsedFormat, Specifier};
use rupnp::Device;
use rupnp::http::Uri;

#[derive(Debug, StructOpt)]
#[structopt(name = "rupnpc", about = "Simple UPnP discoverer written in rust.")]
struct Args {
    #[structopt(long, short)]
    search_target: Option<String>,

    /// Scan duration in seconds.
    #[structopt(long, short, default_value = "3")]
    duration: u8,

    /// Set output format. Available format strings are:
    ///     - name
    ///     - manufacturer
    ///     - model_name
    ///     - udn
    ///     - upc
    ///     - serial
    ///     - manufacturer_url
    ///     - model_description
    ///     - model_url
    ///     - model_number
    ///     - url
    ///     - device_type
    /// To print name and url for each discovered item pass -f "{name} {url}".
    #[structopt(long, short, verbatim_doc_comment)]
    format: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::from_args();

    let search_target: SearchTarget = (&args).try_into()?;
    let devices = rupnp::discover(&search_target, (&args).into()).await?;
    pin_utils::pin_mut!(devices);
    let format = args.format.unwrap_or_else(|| "name: {name}, manufacturer: {manufacturer}, model_name: {model_name}, udn: {udn}, url: {url}".into());
    let vec = Vec::new();
    while let Some(device) = devices.try_next().await? {
        let named = device.to_named_args();
        let format = match ParsedFormat::parse(&format, &vec, &named) {
            Ok(format) => format,
            Err(error_pos) => {
                eprintln!("Invalid format string at position {error_pos}");
                break;
            }
        };

        println!("{}", format);
    }
    Ok(())
}

impl TryFrom<&Args> for SearchTarget {
    type Error = anyhow::Error;

    fn try_from(arguments: &Args) -> std::result::Result<Self, Self::Error> {
        match &arguments.search_target {
            None => Ok(SearchTarget::All),
            Some(value) => {
                SearchTarget::from_str(value.as_str()).with_context(|| format!("Unable to convert '{value}' to an URN."))
            }
        }
    }
}

impl From<&Args> for Duration {
    fn from(args: &Args) -> Self {
        Duration::from_secs(args.duration as u64)
    }
}

trait NamedArgs {
    fn to_named_args(&self) -> HashMap<&str, Value>;
}

impl NamedArgs for Device {
    fn to_named_args(&self) -> HashMap<&str, Value> {
        let mut named_args = HashMap::new();
        named_args.insert("name", self.friendly_name().into());
        named_args.insert("manufacturer", self.manufacturer().into());
        named_args.insert("model_name", self.model_name().into());
        named_args.insert("udn", self.udn().into());
        named_args.insert("upc", self.upc().into());
        named_args.insert("serial", self.serial_number().into());
        named_args.insert("manufacturer_url", self.manufacturer_url().into());
        named_args.insert("model_description", self.model_description().into());
        named_args.insert("model_url", self.model_url().into());
        named_args.insert("model_number", self.model_number().into());
        named_args.insert("url", self.url().into());
        named_args.insert("device_type", self.device_type().into());
        named_args
    }
}

enum Value<'a> {
    Empty,
    String(&'a str),
    Uri(&'a Uri),
    Urn(&'a URN),
}

impl<'a> FormatArgument for Value<'a> {
    fn supports_format(&self, specifier: &Specifier) -> bool {
        matches!(specifier.format, Format::Display | Format::Debug)
    }

    fn fmt_display(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Value::Empty => std::fmt::Display::fmt("", f),
            Value::String(value) => std::fmt::Display::fmt(value, f),
            Value::Uri(value) => std::fmt::Display::fmt(value, f),
            Value::Urn(value) => std::fmt::Display::fmt(value, f),
        }
    }

    fn fmt_debug(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Value::Empty => std::fmt::Debug::fmt("", f),
            Value::String(value) => std::fmt::Debug::fmt(value, f),
            Value::Uri(value) => std::fmt::Debug::fmt(value, f),
            Value::Urn(value) => std::fmt::Debug::fmt(value, f),
        }
    }

    fn fmt_octal(&self, _f: &mut Formatter) -> std::fmt::Result {
        Err(std::fmt::Error)
    }

    fn fmt_lower_hex(&self, _f: &mut Formatter) -> std::fmt::Result {
        Err(std::fmt::Error)
    }

    fn fmt_upper_hex(&self, _f: &mut Formatter) -> std::fmt::Result {
        Err(std::fmt::Error)
    }

    fn fmt_binary(&self, _f: &mut Formatter) -> std::fmt::Result {
        Err(std::fmt::Error)
    }

    fn fmt_lower_exp(&self, _f: &mut Formatter) -> std::fmt::Result {
        Err(std::fmt::Error)
    }

    fn fmt_upper_exp(&self, _f: &mut Formatter) -> std::fmt::Result {
        Err(std::fmt::Error)
    }
}

impl<'a> From<&'a str> for Value<'a> {
    fn from(value: &'a str) -> Self {
        Self::String(value)
    }
}

impl<'a> From<Option<&'a str>> for Value<'a> {
    fn from(value: Option<&'a str>) -> Self {
        match value {
            None => Self::Empty,
            Some(value) => Self::String(value),
        }
    }
}

impl<'a> From<&'a Uri> for Value<'a> {
    fn from(uri: &'a Uri) -> Self {
        Self::Uri(uri)
    }
}

impl<'a> From<&'a URN> for Value<'a> {
    fn from(urn: &'a URN) -> Self {
        Self::Urn(urn)
    }
}
