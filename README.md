```
rupnpc 0.1.0
Simple UPnP discoverer written in rust.

USAGE:
    rupnpc [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --duration <duration>              Scan duration in seconds [default: 3]
    -f, --format <format>                  Set output format. Available format strings are:
                                               - name
                                               - manufacturer
                                               - model_name
                                               - udn
                                               - upc
                                               - serial
                                               - manufacturer_url
                                               - model_description
                                               - model_url
                                               - model_number
                                               - url
                                               - device_type
                                           To print name and url for each discovered item pass -f "{name} {url}".
    -s, --search-target <search-target>
```
