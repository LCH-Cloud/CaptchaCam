# CaptchaCam

CaptchaCam is a command-line tool for controlling the camera and capturing images. It allows adjusting various parameters, such as delay before capturing, the filename of the captured image, choosing the camera by name or number, and much more.

## Usage

The tool uses the following commands and parameters:

```shell
    --delay <delay>          Sets the delay (in seconds) before the camera starts taking pictures. [default: 0]
    --filename <filename>    Sets the filename (as a string) of the image. [default: image]
    --devname <devname>      Specifies the camera name.
    --devnum <devnum>        Sets the camera number. [default: 0]
    --devlist                Lists all available cameras. [default: false]
    --quiet                  Enables or disables the output. [default: false]
    --debug                  Enables debug mode. [default: false]
    --update                 Updates the program. [default: false]
    --selfupdate             Updates the program. [default: false]
```

## Installation

To install this tool, you'll need Rust and Cargo. You can build it directly from the source code by running the following command:

```shell
cargo build --release
```

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**. For detailed information on how to contribute, please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## Code of Conduct

Our project follows a certain standard of community behaviour. Please read our [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) to understand these guidelines.
