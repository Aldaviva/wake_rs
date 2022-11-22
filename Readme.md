# wake_rs

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Aldaviva/wake_rs/Rust?logo=github)](https://github.com/Aldaviva/wake_rs/actions/workflows/rust.yml)

Send wake-on-LAN magic packets from the command line to the MAC addresses of your computers configured in a JSON file.

## System requirements
- Linux x64
- Linux ARMv7
- Windows x64

## Installation
1. Download the executable from the [**latest release**](https://github.com/Aldaviva/wake_rs/releases/latest) (recommended) or [latest build](https://nightly.link/Aldaviva/wake_rs/workflows/rust/master).
1. *Linux only:* give the file the execution permission.
    ```sh
    chmod +x wake_rs
    ```

## Configuration
Create a JSON file with the filename `.wake.json` in your home directory.

```json
{
    "jarnsaxa": {
        "macAddresses": [ "F0-2F-74-89-06-13" ],
        "label": "Jarnsaxa"
    },
    "aegir": {
        "macAddresses": [ "68-05-CA-C1-64-98" ],
        "label": "Aegir"
    }
}
```

- The top-level object's keys are the name of the computer you want to wake up, such as `jarnsaxa`. You will need to supply this value as a command line argument when you run this program.
- The value of each computer key is an object that contains
    - A list of one or more `macAddresses`. Wake-On-LAN packets will be sent to *all* MAC addresses for the given computer. Each item in the array is a case-insensitive string of hexadecimal bytes with byte separator characters. The separator can be any character, such as `:` or `-`, but it is required: a string of the form `000000000000` without separators is invalid. 
    - An optional `label` to show in `stdout` messages. Defaults to the computer name key when omitted.

## Usage

Pass the computer name from the configuration JSON object key as the first argument.

```sh
$ wake_rs aegir
Waking up Aegir...
Sent wake-on-LAN packet to 68-05-CA-C1-64-98
```

For example, I use an always-on Raspberry Pi server to easily wake up other computers on my network. If you have SSH keys installed, you can run this in a quick one-liner:

```bash
$ ssh erebus wake_rs aegir
Waking up Aegir...
Sent wake-on-LAN packet to 68-05-CA-C1-64-98
```

## Performance

**Environment:** Raspberry Pi 2 Model B, 900 MHz 32-bit quad-core ARM Cortex-A7, 1 GB RAM

|Language|Mean runtime|
|---|---:|
|Node.js 12 with tab completion|3,530 ms|
|Node.js 12|1,301 ms|
|.NET 7|1,505 ms|
|Rust 1.64|9 ms|
