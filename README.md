<p align='center'>
  <h5 align='center'>ABOUT</h5>
  <p align='center'>
    Run Minecraft natively on Apple Silicon 🙌
  </p>
</p>

<p align='center'>
  <a href='https://example.com'>
    <img src='https://img.shields.io/badge/status-ready-orange?style=for-the-badge' height='25'>
  </a>
  <a href='https://example.com'>
    <img src='https://img.shields.io/badge/build-success-orange?style=for-the-badge' height='25'>
  </a>
  <a href='https://github.com/raywenderlich/swift-style-guide'>
    <img src='https://img.shields.io/badge/code_style-swift-orange?style=for-the-badge' height='25'>
  </a>
</p>

### [Installation Steps](https://raph.codes/projects/m1necraft)

☝️

Run Minecraft natively on Apple Silicon, easily.
Optimizations to the Minecraft installation are also pre-configured to give you the best performance and stability.
Everything from the launcher to the actual game is optimized well enough such that you can literally just launch and play.

Mods are supported with this, just like the original game.

### Installation

To install this, open the Terminal app (hit command+space and then search for it), and then paste the following into the terminal:

```shell
curl -sSL https://raw.githubusercontent.com/raphtlw/m1necraft/main/installer/install.sh | bash
```

<!-- TODO: write notes about code structure -->

## Developer instructions

### Build instructions

To build m1necraft, open the project in Xcode and build.

### Creating releases

In this repository, releases are automatically built and published when a new tag is created. Create a new Git tag, push it, and a new release will automatically be built.

After the new tag has been pushed, you'll also need to update the Homebrew formula.
Increment the version in the tarball URL and push the changes.

### Appcast generation & release hosting

The script located in `scripts/update_appcast` downloads all the releases, feeds them into the generate_appcast script (by Sparkle), and corrects the XML output produced by the script so that it downloads from GitHub Releases. This means I can continue to host the releases in GitHub Actions, with analytics as an added bonus, without the need for an external server or s3 object storage.

To develop for Python in this repo, you can set up a virtual environment.

```shell
python3 -m venv .venv # create the venv
source .venv/bin/activate # activate the venv
pip install -r requirements.txt # install dependencies
```

## Attributions

The inspiration for this came from [this gist](https://gist.github.com/tanmayb123/d55b16c493326945385e815453de411a). Credits to [Tanmay Bakshi](https://github.com/tanmayb123) for doing this.

[PatrikTheDev](https://twitter.com/PatrikTheDev) for being a legend and helping me out a ton with SwiftUI and just Swift in general.

Application icons from [Maxime Nicoul](https://dribbble.com/maximenicoul).
