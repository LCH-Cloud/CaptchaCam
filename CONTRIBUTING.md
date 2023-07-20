<!-- omit in toc -->
# Contributing to CaptchaCam

First off, thanks for taking the time to contribute! ❤️

All types of contributions are encouraged and valued. See the [Table of Contents](#table-of-contents) for different ways to help and details about how this project handles them. Please make sure to read the relevant section before making your contribution. It will make it a lot easier for us maintainers and smooth out the experience for all involved. The community looks forward to your contributions. 🎉

> And if you like the project, but just don't have time to contribute, that's fine. There are other easy ways to support the project and show your appreciation, which we would also be very happy about:
> - Star the project
> - Tweet about it
> - Refer this project in your project's readme
> - Mention the project at local meetups and tell your friends/colleagues

<!-- omit in toc -->
## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [I Have a Question](#i-have-a-question)
- [I Want To Contribute](#i-want-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Enhancements](#suggesting-enhancements)
  - [Your First Code Contribution](#your-first-code-contribution)
  - [Improving The Documentation](#improving-the-documentation)
- [Styleguides](#styleguides)
  - [Commit Messages](#commit-messages)
- [Join The Project Team](#join-the-project-team)


## Code of Conduct

This project and everyone participating in it is governed by the
[CaptchaCam Code of Conduct](https://github.com/LCH-Cloud/CaptchaCam/blob/master/CODE_OF_CONDUCT.md).
By participating, you are expected to uphold this code. Please report unacceptable behavior
to <contact@lchcloud.de>.


## I Have a Question

> If you want to ask a question, we assume that you have read the available [documentation](https://github.com/LCH-Cloud/CaptchaCam/wiki).

Before you ask a question, it is best to search for existing [Issues](https://github.com/LCH-Cloud/CaptchaCam/issues) that might help you. In case you have found a suitable issue and still need clarification, you can write your question in this issue. It is also advisable to search the internet for answers first.

If you then still feel the need to ask a question and need clarification, we recommend the following:

- Open an [Issue](https://github.com/LCH-Cloud/CaptchaCam/issues/new).
- Provide as much context as you can about what you're running into.
- Provide project and platform versions (nodejs, npm, etc), depending on what seems relevant.

We will then take care of the issue as soon as possible.

<!--
You might want to create a separate issue tag for questions and include it in this description. People should then tag their issues accordingly.

Depending on how large the project is, you may want to outsource the questioning, e.g. to Stack Overflow or Gitter. You may add additional contact and information possibilities:
- IRC
- Slack
- Gitter
- Stack Overflow tag
- Blog
- FAQ
- Roadmap
- E-Mail List
- Forum
-->

## I Want To Contribute

> ### Legal Notice <!-- omit in toc -->
> When contributing to this project, you must agree that you have authored 100% of the content, that you have the necessary rights to the content and that the content you contribute may be provided under the project license.

### Reporting Bugs

<!-- omit in toc -->
#### Before Submitting a Bug Report

A good bug report shouldn't leave others needing to chase you up for more information. Therefore, we ask you to investigate carefully, collect information and describe the issue in detail in your report. Please complete the following steps in advance to help us fix any potential bug as fast as possible.

- Make sure that you are using the latest version.
- Determine if your bug is really a bug and not an error on your side e.g. using incompatible environment components/versions (Make sure that you have read the [documentation](https://github.com/LCH-Cloud/CaptchaCam/wiki). If you are looking for support, you might want to check [this section](#i-have-a-question)).
- To see if other users have experienced (and potentially already solved) the same issue you are having, check if there is not already a bug report existing for your bug or error in the [bug tracker](https://github.com/LCH-Cloud/CaptchaCam/issues?q=label%3Abug).
- Also make sure to search the internet (including Stack Overflow) to see if users outside of the GitHub community have discussed the issue.
- Collect information about the bug:
  - Stack trace (Traceback)
  - OS, Platform and Version (Windows, Linux, macOS, x86, ARM)
  - Version of the interpreter, compiler, SDK, runtime environment, package manager, depending on what seems relevant.
  - Possibly your input and the output
  - Can you reliably reproduce the issue? And can you also reproduce it with older versions?

<!-- omit in toc -->
#### How Do I Submit a Good Bug Report?

> 1. Please use [GitHub Security Advisories](https://github.com/LCH-Cloud/CaptchaCam/security/advisories) in our GitHub repository. This allows us to assess the vulnerability, discuss it, and if necessary, correct it in a private space before a public disclosure.
> 2. Alternatively, you can directly email us at security@lchcloud.de with the relevant details.
<!-- You may add a PGP key to allow the messages to be sent encrypted as well. -->

We use GitHub issues to track bugs and errors. If you run into an issue with the project:

- Open an [Issue](https://github.com/LCH-Cloud/CaptchaCam/issues/new). (Since we can't be sure at this point whether it is a bug or not, we ask you not to talk about a bug yet and not to label the issue.)
- Explain the behavior you would expect and the actual behavior.
- Please provide as much context as possible and describe the *reproduction steps* that someone else can follow to recreate the issue on their own. This usually includes your code. For good bug reports you should isolate the problem and create a reduced test case.
- Provide the information you collected in the previous section.

Once it's filed:

- The project team will label the issue accordingly.
- A team member will try to reproduce the issue with your provided steps. If there are no reproduction steps or no obvious way to reproduce the issue, the team will ask you for those steps and mark the issue as `needs-repro`. Bugs with the `needs-repro` tag will not be addressed until they are reproduced.
- If the team is able to reproduce the issue, it will be marked `needs-fix`, as well as possibly other tags (such as `critical`), and the issue will be left to be [implemented by someone](#your-first-code-contribution).

**NOTE:** We have an issue template for reporting bugs and errors. Please make sure to use it when reporting any issues. It helps us in understanding the problem and finding a solution more efficiently.


### Suggesting Enhancements

This section guides you through submitting an enhancement suggestion for CaptchaCam, **including completely new features and minor improvements to existing functionality**. Following these guidelines will help maintainers and the community to understand your suggestion and find related suggestions.

<!-- omit in toc -->
#### Before Submitting an Enhancement

- Make sure that you are using the latest version.
- Read the [documentation](https://github.com/LCH-Cloud/CaptchaCam/wiki) carefully and find out if the functionality is already covered, maybe by an individual configuration.
- Perform a [search](https://github.com/LCH-Cloud/CaptchaCam/issues) to see if the enhancement has already been suggested. If it has, add a comment to the existing issue instead of opening a new one.
- Find out whether your idea fits with the scope and aims of the project. It's up to you to make a strong case to convince the project's developers of the merits of this feature. Keep in mind that we want features that will be useful to the majority of our users and not just a small subset. If you're just targeting a minority of users, consider writing an add-on/plugin library.

<!-- omit in toc -->
#### How Do I Submit a Good Enhancement Suggestion?

Enhancement suggestions are tracked as [GitHub issues](https://github.com/LCH-Cloud/CaptchaCam/issues).

- Use a **clear and descriptive title** for the issue to identify the suggestion.
- Provide a **step-by-step description of the suggested enhancement** in as many details as possible.
- **Describe the current behavior** and **explain which behavior you expected to see instead** and why. At this point you can also tell which alternatives do not work for you.
- You may want to **include screenshots and animated GIFs** which help you demonstrate the steps or point out the part which the suggestion is related to. You can use [this tool](https://www.cockos.com/licecap/) to record GIFs on macOS and Windows, and [this tool](https://github.com/colinkeenan/silentcast) or [this tool](https://github.com/GNOME/byzanz) on Linux. <!-- this should only be included if the project has a GUI -->
- **Explain why this enhancement would be useful** to most CaptchaCam users. You may also want to point out the other projects that solved it better and which could serve as inspiration.

**NOTE:** We have an issue template for Enhancement Suggestions. Please make sure to use it when reporting any issues. It helps us in understanding the problem and finding a solution more efficiently.

#### How to Submit a Good Feature Request?

Feature requests are tracked as [GitHub issues](https://github.com/LCH-Cloud/CaptchaCam/issues).

To ensure your feature request is clear and actionable, please follow these guidelines:

1. Use a **clear and descriptive title** for the issue to clearly identify the suggested feature.
2. Provide a **step-by-step description** of the suggested enhancement in as much detail as possible.
3. **Describe the current behavior** and **explain the behavior you expected to see instead**, along with your reasons. If applicable, mention any alternative solutions you have considered and why they are not suitable.
4. Consider including **screenshots and animated GIFs** to illustrate the feature or demonstrate the steps involved.
5. **Explain why this enhancement would be beneficial** to most CaptchaCam users. You may also want to reference other projects that have implemented similar features effectively.

**NOTE:** We have an issue template for Feature Requests. Please make sure to use it when submitting your request. The template will help us better understand the problem and find a solution more efficiently.


### Your First Code Contribution

Welcome to your first code contribution to our Rust project! We appreciate your interest in contributing. This guide will help you get started with setting up your environment, IDE, and provide typical getting started instructions.

#### Environment Setup

1. Install Rust: Visit the official Rust website at [https://www.rust-lang.org](https://www.rust-lang.org) and follow the instructions to install Rust on your system.

2. Install VSCode: Download and install Visual Studio Code from [https://code.visualstudio.com](https://code.visualstudio.com).

#### Getting Started

1. Fork the Repository: Click on the "Fork" button at the top-right corner of our repository page to create a copy of the repository in your GitHub account.

2. Clone the Forked Repository: Use Git to clone your forked repository to your local machine.

   ```shell
   git clone https://github.com/your-username/your-forked-repository.git
   ```

3. Open the Project in VSCode: Launch VSCode and open the cloned repository folder.

4. Install Rust Plugin: In VSCode, install the official Rust extension by searching for "Rust" in the Extensions pane and clicking on the "Install" button.

5. Build the Project: In the terminal within VSCode, navigate to the project's root directory and build the project using the following command:

   ```shell
   cargo build
   ```

6. Run Tests: To run the tests for the project, use the following command:

   ```shell
   cargo test
   ```

7. Start Contributing: Now you are all set to make your first code contribution! Explore the codebase, find an area you'd like to work on, and make the necessary changes.

#### Submitting Your Contribution

1. Commit Your Changes: Once you have made your code changes, commit them using Git.

   ```shell
   git add .
   git commit -m "Your commit message"
   ```

2. Push Your Changes: Push your changes to your forked repository.

   ```shell
   git push origin your-branch-name
   ```

3. Create a Pull Request: Visit our project's repository on GitHub, navigate to the "Pull Requests" tab, and click on the "New Pull Request" button. Follow the prompts to submit your pull request with a clear description of the changes you made.

Thank you for your contribution! We appreciate your effort and look forward to reviewing and merging your code.

### Improving The Documentation

We highly appreciate contributions to improve the documentation of our project. Your efforts in updating, improving, and correcting the documentation are invaluable. To ensure a smooth contribution process, please follow these guidelines:

1. **Identify Areas for Improvement**: Take some time to explore the existing documentation and identify areas that need updating, improvement, or correction.

2. **Create an Issue**: Before starting your work, create an issue in our issue tracker to discuss and receive feedback on your proposed changes. This helps to ensure that your efforts align with our project's goals and avoids duplication of work.

3. **Fork the Repository**: Fork our repository to your GitHub account and create a new branch for your documentation improvements.

4. **Make Your Changes**: Update, improve, and correct the documentation in the appropriate files and directories. Ensure that your changes follow our documentation style guide, if applicable.

5. **Test Your Changes**: After making your changes, preview and test the documentation locally to ensure it is accurate and well-formatted.

6. **Submit a Pull Request**: When you are confident in your changes, submit a pull request from your forked repository to our main repository. Clearly describe the purpose and scope of your changes in the pull request description.

7. **Engage in the Review Process**: Be responsive to feedback and actively engage in the review process. Address any requested changes or concerns promptly and effectively.

8. **Celebrate Your Contribution**: Once your pull request is approved and merged, celebrate your successful contribution to improving our project's documentation!

Thank you for your dedication to improving our documentation. Your contributions play a vital role in helping others understand and utilize our project effectively.


## Styleguides

### Commit Messages

We strive to maintain clear and consistent commit messages throughout our project. When making a commit, please follow these guidelines:

1. **Be Clear and Concise**: Write commit messages that are clear, concise, and accurately describe the purpose of the commit.

2. **Use Imperative Verbs**: Begin commit messages with an imperative verb, such as "Fix," "Add," "Update," or "Refactor."

3. **Provide Context**: Provide additional context or relevant information in the commit message body if necessary. This can help others understand the changes made.

### Code Formatting

To ensure a consistent code style across our project, we use Rustfmt and Rust Clippy. Please follow these guidelines:

1. **Use Rustfmt**: Format your code using Rustfmt to maintain a consistent and standardized code style. Run Rustfmt on your code before submitting a pull request.

2. **Address Clippy Warnings**: Run Rust Clippy on your code to catch any potential issues or improvements. Address any Clippy warnings or suggestions before submitting your changes.

By following these style guidelines and utilizing Rustfmt and Rust Clippy, we can maintain a high level of code quality and consistency throughout the project.

## Join The Project Team

We welcome contributions from anyone interested in joining our project team. By becoming a member of our team, you can have a direct impact on the project's development and collaborate with other passionate individuals. Here's how you can join:

1. **Explore the Project**: Take some time to familiarize yourself with our project, its goals, and its current state. Understand the problem we are trying to solve and the technologies we are using.

2. **Identify Your Interests**: Determine which areas of the project align with your interests, skills, and expertise. This will help you make meaningful contributions that leverage your strengths.

3. **Introduce Yourself**: Reach out to our team by sending an email to [team@lchcloud.de](mailto:team@lchcloud.de). Introduce yourself, express your interest in joining the project, and share any relevant experience or background you have.

4. **Collaborate and Contribute**: Once you have connected with our team, start collaborating and contributing. This can include participating in discussions, providing feedback on existing issues and pull requests, and working on tasks assigned to you.

5. **Demonstrate Your Commitment**: Show your dedication and commitment to the project by consistently delivering high-quality work, being proactive in your contributions, and adhering to project guidelines and best practices.

6. **Earn Maintainer Status**: As you continue to contribute and demonstrate your skills and commitment, you may be invited to become a project maintainer. This role entails additional responsibilities and decision-making authority within the project.

Joining our project team is a great opportunity to enhance your skills, work on real-world challenges, and collaborate with a passionate community. We look forward to having you on board!

<!-- omit in toc -->
## Attribution
This guide is based on the **contributing-gen**. [Make your own](https://github.com/bttger/contributing-gen)!
