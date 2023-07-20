# CaptchaCam

CaptchaCam is a command line interface for controlling a camera and capturing images. It provides various options for configuring the camera settings and saving the captured images.

## Installation

To use CaptchaCam, you need to have Rust programming language and Cargo package manager installed on your system. Follow these steps to install CaptchaCam:

1. Install Rust and Cargo by following the official Rust installation guide: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install).

2. Clone the CaptchaCam repository from GitHub:

   ```
   git clone https://github.com/username/CaptchaCam.git
   ```

3. Navigate to the cloned repository:

   ```
   cd CaptchaCam
   ```

4. Build and install the application using Cargo:

   ```
   cargo install --path .
   ```

   This will compile the code and install CaptchaCam on your system.

## Usage

Once you have installed CaptchaCam, you can use it to control your camera and capture images. Here are some examples of how to use CaptchaCam:

- Capture an image from the default camera with no delay:

  ```
  captchacam
  ```

- Capture an image from a specific camera device with a 5-second delay:

  ```
  captchacam --devnum 1 --delay 5
  ```

- List all available cameras:

  ```
  captchacam --devlist
  ```

- Enable debug mode to get additional logging information:

  ```
  captchacam --debug
  ```

For more information on available options, you can use the `--help` flag:

```
captchacam --help
```

## Dependencies

CaptchaCam depends on the following external crates:

- `chrono` (version: "0.4") - For working with dates and times.
- `clap` (version: "3.0") - For parsing command line arguments.
- `nokhwa` (version: "0.2") - For camera control and image capturing.
- `tracing` (version: "0.1") - For application logging.

The dependencies will be automatically fetched and managed by Cargo during the installation process.

## Contributing

If you want to contribute to CaptchaCam, you can do so by following these steps:

1. Fork the CaptchaCam repository on GitHub.

2. Clone your forked repository to your local machine:

   ```
   git clone https://github.com/your-username/CaptchaCam.git
   ```

3. Create a new branch for your changes:

   ```
   git checkout -b feature/my-new-feature
   ```

4. Make your changes and commit them with descriptive commit messages.

5. Push your changes to your forked repository:

   ```
   git push origin feature/my-new-feature
   ```

6. Open a pull request on the main CaptchaCam repository.

Please make sure to follow the [code style guidelines](https://github.com/username/CaptchaCam/blob/main/CODE_STYLE_GUIDELINES.md) and write tests for your changes to ensure the code quality.
