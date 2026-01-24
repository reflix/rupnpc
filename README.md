```
Simple UPnP discoverer written in rust

Usage: rupnpc [OPTIONS]

Options:
  -u, --urn <URN>            Search for a specific resource with the given unique resource name
  -d, --duration <DURATION>  Scan duration in seconds [default: 3]
  -t, --ttl <TTL>            Time to live
  -w, --warn                 Show warnings on erroneous responses
  -f, --format <FORMAT>      Set output format. Available format strings are:
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
  -n, --no-unique-udns       Disables filtering of already printed devices. Disable, if you assume that there are
                             devices returning the same unique device name (UDN)
  -h, --help                 Print help
```