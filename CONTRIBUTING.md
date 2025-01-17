# Contributing to Scribe-Desktop

Thank you for your interest in contributing!

Please take a moment to review this document in order to make the contribution process easy and effective for everyone involved.

Following these guidelines helps to communicate that you respect the time of the developers managing and developing this open-source project. In return, and in accordance with this project's [code of conduct](https://github.com/scribe-org/Scribe-Desktop/blob/main/.github/CODE_OF_CONDUCT.md), other contributors will reciprocate that respect in addressing your issue or assessing changes and features.

If you have questions or would like to communicate with the team, please [join us in our public Matrix chat rooms](https://matrix.to/#/#scribe_community:matrix.org). We'd be happy to hear from you!

<a id="contents"></a>

# **Contents**

- [First steps as a contributor](#first-steps)
- [Learning the tech stack](#learning-the-tech-)
- [Development environment](#dev-env)
- [Issues and projects](#issues-projects)
- [Bug reports](#bug-reports)
- [Feature requests](#feature-requests)
- [Pull requests](#pull-requests)
- [Data edits](#data-edits)
- [Localization](#localization)
- [Documentation](#documentation)
- [Design](#design)

<a id="first-steps"></a>

## First steps as a contributor [`⇧`](#contents)

Thank you for your interest in contributing to Scribe-Desktop! We look forward to welcoming you to the community and working with you to build an tools for language learners to communicate effectively :) The following are some suggested steps for people interested in joining our community:

- Please join the [public Matrix chat](https://matrix.to/#/#scribe_community:matrix.org) to connect with the community
  - [Matrix](https://matrix.org/) is a network for secure, decentralized communication
  - Scribe would suggest that you use the [Element](https://element.io/) client
  - The [Desktop](https://matrix.to/#/#ScribeDesktop:matrix.org) channel would be a great place to start!
  - Feel free to introduce yourself and tell us what your interests are if you're comfortable :)
- Read through this contributing guide for all the information you need to contribute
- Look into issues marked [`good first issue`](https://github.com/scribe-org/Scribe-Desktop/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) and the [Projects board](https://github.com/orgs/scribe-org/projects/1) to get a better understanding of what you can work on
- Check out our [public designs on Figma](https://www.figma.com/file/c8945w2iyoPYVhsqW7vRn6/scribe_public_designs?type=design&node-id=405-464&mode=design&t=E3ccS9Z8MDVSizQ4-0) to understand Scribes's goals and direction
- Consider joining our [bi-weekly developer sync](https://etherpad.wikimedia.org/p/scribe-dev-sync)!

<a id="learning-the-tech-"></a>

## Learning the tech stack [`⇧`](#contents)

Scribe-Desktop is very open to contributions from people in the early stages of their coding journey! The following is a select list of documentation pages to help you understand the technologies we use.

As Scribe-Desktop is written in [Rust](https://www.rust-lang.org/), we'd suggest all contributors to have read [The Rust Book](https://doc.rust-lang.org/book/).

<details><summary>Docs for those new to programming</summary>
<p>

- [Mozilla Developer Network Learning Area](https://developer.mozilla.org/en-US/docs/Learn)
  - Doing MDN sections for HTML, CSS and JavaScript is the best ways to get into web development!
- [Open Source Guides](https://opensource.guide/)
  - Guides from GitHub about open-source software including how to start and much more!

</p>
</details>

<a id="dev-env"></a>

# Development environment [`⇧`](#contents)

1. First and foremost, please see the suggested IDE setup in the dropdown below to make sure that your editor is ready for development.

> [!IMPORTANT]
>
> <details><summary>Suggested IDE setup</summary>
>
> <p>
>
> VS Code
>
> Install the following extensions:
>
> - [qwtel.sqlite-viewer](https://marketplace.visualstudio.com/items?itemName=qwtel.sqlite-viewer)
> - [streetsidesoftware.code-spell-checker](https://marketplace.visualstudio.com/items?itemName=streetsidesoftware.code-spell-checker)
> - [rust-lang.rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
>
> </p>
> </details>
> Those new to coding or wanting to develop their skills are more than welcome to contribute! The general steps to setting up a development environment are:

2. [Fork](https://docs.github.com/en/get-started/quickstart/fork-a-repo) the [Scribe-Desktop repo](https://github.com/scribe-org/Scribe-Desktop), clone your fork, and configure the remotes:

> [!NOTE]
>
> <details><summary>Consider using SSH</summary>
>
> <p>
>
> Alternatively to using HTTPS as in the instructions below, consider SSH to interact with GitHub from the terminal. SSH allows you to connect without a user-pass authentication flow.
>
> To run git commands with SSH, remember then to substitute the HTTPS URL, `https://github.com/...`, with the SSH one, `git@github.com:...`.
>
> - e.g. Cloning now becomes `git clone git@github.com:<your-username>/Scribe-Desktop.git`
>
> GitHub also has their documentation on how to [Generate a new SSH key](https://docs.github.com/en/authentication/connecting-to-github-with-ssh/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent) 🔑
>
> </p>
> </details>

```bash
# Clone your fork of the repo into the current directory.
git clone https://github.com/<your-username>/Scribe-Desktop.git
# Navigate to the newly cloned directory.
cd Scribe-Desktop
# Assign the original repo to a remote called "upstream".
git remote add upstream https://github.com/scribe-org/Scribe-Desktop.git
```

- Now, if you run `git remote -v` you should see two remote repositories named:
  - `origin` (forked repository)
  - `upstream` (Scribe-Desktop repository)

3. Install Rust and Cargo:

- For Linux and macOS:

Run the following command in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions to complete the installation. You can find more details in the [official Rust documentation](https://doc.rust-lang.org/book/ch01-01-installation.html).

- For Windows:

Download and run [rustup-init.exe](https://rustup.rs/). Follow the instructions in the installer. For more details, visit the [Rust Windows installation guide](https://www.rust-lang.org/tools/install).

After installation, you may need to restart your terminal for the changes to take effect. Verify the installation by running:

```bash
rustc --version
cargo --version
```

4. (Suggested) Install [pre-commit](https://pre-commit.com/) and its hooks to check for and correct common errors in commits:

```bash
pip install pre-commit
pre-commit install
# pre-commit run --all-files
```

5. Run the following to spin up a local copy of the Scribe-Desktop GUI:

```bash
cd scribe
cargo build
cargo run --bin scribe
```

6. You may need to give your terminal or IDE permission for your keyboard strokes to be read
   - You may also need to restart the application and then run `cargo run --bin scribe` again

> [!NOTE]
> Feel free to contact the team in the [Desktop room on Matrix](https://matrix.to/#/#ScribeDesktop:matrix.org) if you're having problems getting your environment setup!

<a id="issues-projects"></a>

# Issues and projects [`⇧`](#contents)

The [issue tracker for Scribe-Desktop](https://github.com/scribe-org/Scribe-Desktop/issues) is the preferred channel for [bug reports](#bug-reports), [features requests](#feature-requests) and [submitting pull requests](#pull-requests). Scribe also organizes related issues into [projects](https://github.com/scribe-org/Scribe-Desktop/projects).

> [!NOTE]\
> Just because an issue is assigned on GitHub doesn't mean that the team isn't interested in your contribution! Feel free to write [in the issues](https://github.com/scribe-org/Scribe-Desktop/issues) and we can potentially reassign it to you.

Be sure to check the [`-next release-`](https://github.com/scribe-org/Scribe-Desktop/labels/-next%20release-) and [`-priority-`](https://github.com/scribe-org/Scribe-Desktop/labels/-priority-) labels in the [issues](https://github.com/scribe-org/Scribe-Desktop/issues) for those that are most important, as well as those marked [`good first issue`](https://github.com/scribe-org/Scribe-Desktop/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) that are tailored for first time contributors.

<a id="bug-reports"></a>

# Bug reports [`⇧`](#contents)

A bug is a _demonstrable problem_ that is caused by the code in the repository. Good bug reports are extremely helpful - thank you!

Guidelines for bug reports:

1. **Use the GitHub issue search** to check if the issue has already been reported.

2. **Check if the issue has been fixed** by trying to reproduce it using the latest `main` or development branch in the repository.

3. **Isolate the problem** to make sure that the code in the repository is _definitely_ responsible for the issue.

**Great Bug Reports** tend to have:

- A quick summary
- Steps to reproduce
- What you expected would happen
- What actually happens
- Notes (why this might be happening, things tried that didn't work, etc)

To make the above steps easier, the Scribe team asks that contributors report bugs using the [bug report template](https://github.com/scribe-org/Scribe-Desktop/issues/new?assignees=&labels=feature&template=bug_report.yml), with these issues further being marked with the [`bug`](https://github.com/scribe-org/Scribe-Desktop/issues?q=is%3Aopen+is%3Aissue+label%3Abug) label.

Again, thank you for your time in reporting issues!

<a id="feature-requests"></a>

# Feature requests [`⇧`](#contents)

Feature requests are more than welcome! Please take a moment to find out whether your idea fits with the scope and aims of the project. When making a suggestion, provide as much detail and context as possible, and further make clear the degree to which you would like to contribute in its development.

<a id="pull-requests"></a>

# Pull requests [`⇧`](#contents)

Good pull requests - patches, improvements and new features - are the foundation of our community making Scribe-Desktop. They should remain focused in scope and avoid containing unrelated commits. Note that all contributions to this project will be made under [the specified license](https://github.com/scribe-org/Scribe-Desktop/blob/main/LICENSE.txt) and should follow the coding indentation and style standards ([contact us](https://matrix.to/#/#scribe_community:matrix.org) if unsure).

**Please ask first** before embarking on any significant pull request (implementing features, refactoring code, etc), otherwise you risk spending a lot of time working on something that the developers might not want to merge into the project. With that being said, major additions are very appreciated!

When making a contribution, adhering to the [GitHub flow](https://guides.github.com/introduction/flow/index.html) process is the best way to get your work merged:

1. If you cloned a while ago, get the latest changes from upstream:

   ```bash
   git checkout <dev-branch>
   git pull upstream <dev-branch>
   ```

2. Create a new topic branch (off the main project development branch) to contain your feature, change, or fix:

   ```bash
   git checkout -b <topic-branch-name>
   ```

3. Commit your changes in logical chunks, and please try to adhere to [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).

> [!NOTE]
> The following are tools and methods to help you write good commit messages ✨
>
> - [commitlint](https://commitlint.io/) helps write [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/)
> - Git's [interactive rebase](https://docs.github.com/en/github/getting-started-with-github/about-git-rebase) cleans up commits

4. Locally merge (or rebase) the upstream development branch into your topic branch:

   ```bash
   git pull --rebase upstream <dev-branch>
   ```

5. Push your topic branch up to your fork:

   ```bash
   git push origin <topic-branch-name>
   ```

6. [Open a Pull Request](https://help.github.com/articles/using-pull-requests/) with a clear title and description.

Thank you in advance for your contributions!

# Data edits [`⇧`](#contents)

> [!NOTE]\
> Please see the [Wikidata and Scribe Guide](https://github.com/scribe-org/Organization/blob/main/WIKIDATAGUIDE.md) for an overview of [Wikidata](https://www.wikidata.org/) and how Scribe uses it.

Scribe does not accept direct edits to the grammar JSON files as they are sourced from [Wikidata](https://www.wikidata.org/). Edits can be discussed and the [Scribe-Data](https://github.com/scribe-org/Scribe-Data) queries will be changed and ran before an update. If there is a problem with one of the files, then the fix should be made on [Wikidata](https://www.wikidata.org/) and not on Scribe. Feel free to let us know that edits have been made by [opening a data issue](https://github.com/scribe-org/Scribe-Desktop/issues/new?assignees=&labels=data&template=data_wikidata.yml) or contacting us in the [issues for Scribe-Data](https://github.com/scribe-org/Scribe-Data/issues) and we'll be happy to integrate them!

<a id="localization"></a>

# Localization [`⇧`](#contents)

Being an app that focusses on language learning, localization plays a big part in what Scribe will eventually be. Those interested are more than welcome to join the team at [scribe-org/Scribe-i18n](https://github.com/scribe-org/Scribe-i18n) where we work on localizing all Scribe applications via [Weblate](https://weblate.org/).

<a id="documentation"></a>

# Documentation [`⇧`](#contents)

Documentation is an invaluable way to contribute to coding projects as it allows others to more easily understand the project structure and contribute. Issues related to documentation are marked with the [`documentation`](https://github.com/scribe-org/Scribe-Desktop/labels/documentation) label.

<a id="design"></a>

# Design [`⇧`](#contents)

<a href="https://www.figma.com/file/c8945w2iyoPYVhsqW7vRn6/scribe_public_designs?node-id=405%3A464"><img src="https://raw.githubusercontent.com/scribe-org/Organization/main/resources/images/figma_logo.png" height="50" alt="Public Figma Designs" align="right"></a>

Designs for Scribe are done in the [public design file in Figma](https://www.figma.com/file/c8945w2iyoPYVhsqW7vRn6/scribe_public_designs?node-id=405%3A464). Those interested in helping with Scribe's design are also welcome to share their ideas using the [design improvement](https://github.com/scribe-org/Scribe-Desktop/issues/new?assignees=&labels=design&template=design_improvement.yml) template that makes an issue marked with the [`design`](https://github.com/scribe-org/Scribe-Desktop/issues?q=is%3Aopen+is%3Aissue+label%3Adesign) label.

All branding elements such as logos, icons, colors and fonts should follow those that are set out in [scribe-org/Organization](https://github.com/scribe-org/Organization). As the project is fully open source, these elements are also open for discussion. Efforts in making Scribe products professional with a distinct and cohesive identity are much appreciated!
