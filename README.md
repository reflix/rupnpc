```
Simple UPnP discoverer written in rust

Usage: rupnpc [OPTIONS]

Options:
  -s, --search-target <SEARCH_TARGET>  
  -d, --duration <DURATION>            Scan duration in seconds [default: 3]
  -w, --warn                           Show warnings on erroneous responses
  -f, --format <FORMAT>                Set output format. Available format strings are:
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
  -h, --help                           Print help
```