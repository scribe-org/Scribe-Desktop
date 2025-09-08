<div align="center">
  <a href="https://github.com/scribe-org/Scribe-Desktop"><img src="https://raw.githubusercontent.com/scribe-org/Organization/main/logo/ScribeGitHubOrgBanner.png" width=1024 alt="Scribe Logo"></a>
</div>

[![platforms](https://img.shields.io/static/v1?message=Linux%20│%20macOS%20│%20Windows&logo=windows-terminal&color=0183DC&logoColor=white&label=%20)](https://github.com/scribe-org/Scribe-Desktop)
[![issues](https://img.shields.io/github/issues/scribe-org/Scribe-Desktop?label=%20&logo=github)](https://github.com/scribe-org/Scribe-Desktop/issues)
[![rust](https://img.shields.io/badge/Rust%201.80-CE412B.svg?logo=rust&logoColor=ffffff)](https://github.com/scribe-org/Scribe-Desktop/blob/main/CONTRIBUTING.md)
[![license](https://img.shields.io/github/license/scribe-org/Scribe-Desktop.svg?label=%20)](https://github.com/scribe-org/Scribe-Desktop/blob/main/LICENSE.txt)
[![coc](https://img.shields.io/badge/Contributor%20Covenant-ff69b4.svg)](https://github.com/scribe-org/Scribe-Desktop/blob/main/.github/CODE_OF_CONDUCT.md)
[![weblate](https://img.shields.io/badge/Weblate-144D3F.svg?logo=weblate&logoColor=ffffff)](https://hosted.weblate.org/projects/scribe/scribe-i18n)
[![mastodon](https://img.shields.io/badge/Mastodon-6364FF.svg?logo=mastodon&logoColor=ffffff)](https://wikis.world/@scribe)
[![matrix](https://img.shields.io/badge/Matrix-000000.svg?logo=matrix&logoColor=ffffff)](https://matrix.to/#/#scribe_community:matrix.org)

<!-- <a href='https://microsoft.com/store/apps/windows'><img alt='Get it from Microsoft' src='https://raw.githubusercontent.com/scribe-org/Scribe-Desktop/main/.github/resources/images/microsoft_store_badge.png' height='60px'/></a>
<a href='https://apps.apple.com/app/scribe-language-keyboards/id1596613886'><img alt='Available on the App Store' src='https://raw.githubusercontent.com/scribe-org/Scribe-Desktop/main/.github/resources/images/app_store_badge.png' height='60px'/></a>
<a href='https://flathub.org/home'><img src='https://raw.githubusercontent.com/scribe-org/Scribe-Desktop/main/.github/resources/images/flathub_badge.png' alt='Download on Flathub' height='60px' /></a> -->

### Typing GUI for language learners on Windows, Mac and Linux

#### Planned port of [Scribe-iOS](https://github.com/scribe-org/Scribe-iOS): see [Issues](https://github.com/scribe-org/Scribe-Desktop/issues)

**Scribe-Desktop** is a language learning interface for Windows, Mac and Linux operating systems. The Scribe GUI provides needed information as the user types and serves as a field for input commands. Features include translation **`(beta)`**, verb conjugation and word annotation that give users the tools needed to communicate with confidence.

Scribe is fully open-source and does not collect usage data or ask for system access. Feature data is sourced from [Wikidata](https://www.wikidata.org/) and stored in-app, meaning Scribe is a highly responsive experience that does not require an internet connection.

> [!NOTE]\
> The [contributing](#contributing) section has information for those interested, with the articles and presentations in [featured by](#featured-by) also being good resources for learning more about Scribe.

Also available on [iOS](https://github.com/scribe-org/Scribe-iOS), [Android](https://github.com/scribe-org/Scribe-Android) (WIP) and for the data processes see [Scribe-Data](https://github.com/scribe-org/Scribe-Data).

Check out Scribe's [architecture diagrams](https://github.com/scribe-org/Organization/blob/main/ARCHITECTURE.md) for an overview of the organization including our applications, services and processes. It depicts the projects that [Scribe](https://github.com/scribe-org) is developing as well as the relationships between them and the external systems with which they interact.s

<a id="contents"></a>

# **Contents**

<!-- - [Setup](#setup)
  - [Base Functionality](#base-functionality) -->

- [Preview Images](#preview-images)
- [Contributing](#contributing)
- [Environment Setup](#environment-setup)
- [Featured By](#featured-by)

<a id="preview-images"></a>

# Preview Images [`⇧`](#contents)

The current WIP Scribe-Desktop UI is:

<div align="center">
  <br>
  <a href="https://github.com/scribe-org/Scribe-Desktop/blob/main/.github/resources/images/scribe_desktop_base_view.png"><img width="500" src="https://raw.githubusercontent.com/scribe-org/Scribe-Desktop/main/.github/resources/images/scribe_desktop_base_view.png" alt="Scribe-Desktop base view"></a>
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  <a href="https://github.com/scribe-org/Scribe-Desktop/blob/main/.github/resources/images/scribe_desktop_command_selection.png"><img width="500" src="https://raw.githubusercontent.com/scribe-org/Scribe-Desktop/main/.github/resources/images/scribe_desktop_command_selection.png" alt="Scribe-Desktop command selection"></a>
  &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  <br>
</div>

<a id="contributing"></a>

# Contributing [`⇧`](#contents)

<a href="https://matrix.to/#/#scribe_community:matrix.org">
  <img src="https://raw.githubusercontent.com/scribe-org/Organization/main/resources/images/logos/MatrixLogoGrey.png" width="175" alt="Public Matrix Chat" align="right">
</a>

Scribe uses [Matrix](https://matrix.org/) for communications. You're more than welcome to [join us in our public chat rooms](https://matrix.to/#/#scribe_community:matrix.org) to share ideas, ask questions or just say hi :) We'd suggest that you use the [Element](https://element.io/) client and [Element X](https://element.io/app) for a mobile app.

Please see the [contribution guidelines](https://github.com/scribe-org/Scribe-Desktop/blob/main/CONTRIBUTING.md) if you are interested in contributing to Scribe-Desktop. Work that is in progress or could be implemented is tracked in the [issues](https://github.com/scribe-org/Scribe-Desktop/issues) and [projects](https://github.com/scribe-org/Scribe-Desktop/projects).

> [!NOTE]\
> Just because an issue is assigned on GitHub doesn't mean the team isn't open to your contribution! Feel free to write [in the issues](https://github.com/scribe-org/Scribe-Desktop/issues) and we can potentially reassign it to you.

Those interested can further check the [`-next release-`](https://github.com/scribe-org/Scribe-Desktop/labels/-next%20release-) and [`-priority-`](https://github.com/scribe-org/Scribe-Desktop/labels/-priority-) labels in the [issues](https://github.com/scribe-org/Scribe-Desktop/issues) for those that are most important, as well as those marked [`good first issue`](https://github.com/scribe-org/Scribe-Desktop/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) that are tailored for first-time contributors.

### Ways to Help [`⇧`](#contents)

- [Reporting bugs](https://github.com/scribe-org/Scribe-Desktop/issues/new?assignees=&labels=bug&template=bug_report.yml) as they're found 🐞
- Working on [new features](https://github.com/scribe-org/Scribe-Desktop/issues?q=is%3Aissue+is%3Aopen+label%3Afeature) ✨
- [Localization](https://github.com/scribe-org/Scribe-i18n) for the app and App Store via our [Weblate project](https://hosted.weblate.org/projects/scribe/scribe-i18n) 🌐
- [Documentation](https://github.com/scribe-org/Scribe-Desktop/issues?q=is%3Aissue+is%3Aopen+label%3Adocumentation) for onboarding and project cohesion 📝
- Adding language data to [Scribe-Data](https://github.com/scribe-org/Scribe-Data/issues) via [Wikidata](https://www.wikidata.org/)! 🗃️

### Road Map [`⇧`](#contents)

The Scribe road map can be followed in the organization's [project board](https://github.com/orgs/scribe-org/projects/1) where we list the most important issues along with their priority, status and an indication of which sub projects they're included in (if applicable).

> [!NOTE]\
> Consider joining our [bi-weekly developer syncs](https://etherpad.wikimedia.org/p/scribe-dev-sync)!

### Designs [`⇧`](#contents)

<a href="https://www.figma.com/file/c8945w2iyoPYVhsqW7vRn6/scribe_public_designs?node-id=405%3A464">
  <img src="https://raw.githubusercontent.com/scribe-org/Organization/main/resources/images/logos/FigmaLogo.png" width="100" alt="Public Figma Designs" align="right">
</a>

The [designs for Scribe](https://www.figma.com/file/c8945w2iyoPYVhsqW7vRn6/scribe_public_designs?node-id=405%3A464) are made using [Figma](https://www.figma.com). Those with interest in contributing can [open a design issue](https://github.com/scribe-org/Scribe-Desktop/issues/new?assignees=&labels=design&template=design_improvement.yml) to make suggestions! Design related issues are marked with the [`design`](https://github.com/scribe-org/Scribe-Desktop/issues?q=is%3Aopen+is%3Aissue+label%3Adesign) label.

### Data Edits [`⇧`](#contents)

> [!NOTE]\
> Please see the [Wikidata and Scribe Guide](https://github.com/scribe-org/Organization/blob/main/WIKIDATAGUIDE.md) for an overview of [Wikidata](https://www.wikidata.org/) and how Scribe uses it.

Scribe does not accept direct edits to the grammar JSON files as they are sourced from [Wikidata](https://www.wikidata.org/). Edits can be discussed and the queries themselves will be changed and ran before an update. If there is a problem with one of the files, then the fix should be made on [Wikidata](https://www.wikidata.org/) and not on Scribe. Feel free to let us know that edits have been made by [opening a data issue](https://github.com/scribe-org/Scribe-Desktop/issues/new?assignees=&labels=data&template=data_wikidata.yml) or contacting us in the [issues for Scribe-Data](https://github.com/scribe-org/Scribe-Data/issues) and we'll be happy to integrate them!

<a id="environment-setup"></a>

# Environment Setup [`⇧`](#contents)

Those new to coding or wanting to develop their skills are more than welcome to contribute! The general steps to setting up a development environment are:

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
cargo run
```

6. You may need to give your terminal or IDE permission for your keyboard strokes to be read
   - You may also need to restart the application and then run `cargo run` again

> [!NOTE]
> Feel free to contact the team in the [Desktop room on Matrix](https://matrix.to/#/#ScribeDesktop:matrix.org) if you're having problems getting your environment setup!

<a id="featured-by"></a>

# Featured By [`⇧`](#contents)

Please see the [blog posts page on our website](https://scri.be/docs/about/blog-posts) for a list of articles on Scribe, and feel free to open a pull request to add one that you've written at [scribe-org/scri.be](github.com/scribe-org/scri.be)!

### Organizations

The following organizations have supported the development of Scribe projects through various programs. Thank you all! 💙

<div align="center">
  <br>
    <a href="https://tech-news.wikimedia.de/en/2022/03/18/lexicographical-data-for-language-learners-the-wikidata-based-app-scribe/"><img width="180" src="https://raw.githubusercontent.com/scribe-org/Organization/main/resources/images/logos/WikimediaDeutschlandLogo.png" alt="Wikimedia Deutschland logo linking to an article on Scribe in the tech news blog."></a>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
    <a href="https://www.mediawiki.org/wiki/New_Developers#Scribe"><img width="180" src="https://raw.githubusercontent.com/scribe-org/Organization/main/resources/images/logos/WikimediaFoundationLogo.png" alt="Wikimedia Foundation logo linking to the MediaWiki new developers page."></a>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  <br>
</div>

<div align="center">
  <br>
    <a href="https://summerofcode.withgoogle.com/"><img width="140" src="https://raw.githubusercontent.com/scribe-org/Organization/main/resources/images/logos/GSoCLogo.png" alt="Google Summer of Code logo linking to its website."></a>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
    <a href="https://www.outreachy.org/"><img width="350" src="https://raw.githubusercontent.com/scribe-org/Organization/main/resources/images/logos/OutreachyLogo.png" alt="Outreachy logo linking to its website."></a>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  <br>
</div>

# Powered By [`⇧`](#contents)

### Contributors

Many thanks to all the [Scribe-Desktop contributors](https://github.com/scribe-org/Scribe-Desktop/graphs/contributors)! 🚀

<a href="https://github.com/scribe-org/Scribe-Desktop/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=scribe-org/Scribe-Desktop" />
</a>

### Code and Dependencies

The Scribe community would like to thank all the great software that made Scribe-Desktop's development possible.

### Wikimedia Communities

<div align="center">
  <br>
    <a href="https://www.wikidata.org/">
      <img width="240" src="https://raw.githubusercontent.com/scribe-org/Organization/main/resources/images/logos/WikidataLogo.png" alt="Wikidata logo">
    </a>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
    <a href="https://www.wikipedia.org/">
      <img width="160" src="https://raw.githubusercontent.com/scribe-org/Organization/main/resources/images/logos/WikipediaLogo.png" alt="Wikipedia logo">
    </a>
    &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
  <br>
</div>
