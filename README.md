<html>
<div align="center">
<img src="assets/logo.jpg" height=64 width=64>
<h1>Skybase</h1><h3>The next-generation NoSQL database</h3>

![GitHub Workflow Status](https://img.shields.io/github/workflow/status/skybasedb/skybase/Test%20(push)) ![GitHub Workflow Status](https://img.shields.io/github/workflow/status/skybasedb/skybase/Sync%20with%20mirrors?label=mirror%20sync) ![Status: Alpha](https://img.shields.io/badge/status-alpha-critical?style=flat-square) ![Development](https://img.shields.io/badge/development-regular-32CD32?style=flat-square) ![GitHub release (latest SemVer including pre-releases)](https://img.shields.io/github/v/release/skybasedb/skybase?include_prereleases&sort=semver&style=flat-square)
[![Docs](https://img.shields.io/badge/readthedocs-here-blueviolet?style=flat-square)](https://terrabasedb.github.io/docs)  [![Contribute Now](https://img.shields.io/badge/%F0%9F%8C%9Fcontribute-now-a94064)](https://ohsayan.github.io/thanks)

</div>
</html>

## What is Skybase?

Skybase (or SkybaseDB/SDB) is an effort to provide the best of key/value stores, document stores and columnar databases, that is, **simplicity, flexibility and queryability at scale**. The name 'Skybase' exemplifies our vision to create a database that has limitless possibilities. Skybase was previously known as TerrabaseDB.

Skybase is curently in an alpha stage, but can be used as a **performant**, **secure** and **persistent key-value store**.

## Getting started 🚀

1. Download a bundle for your platform from [here ⬇️ ](https://github.com/skybasedb/skybase/releases)
2. Unzip the bundle
3. Make the files executable (run `chmod +x sdb skysh` on *nix systems)
4. First run `sdb` to start the database server and then run `skysh` to start the interactive shell
5. Run commands like: `SET foo bar` , `GET bar` , `UPDATE cat mitten` or `DEL proprietary` 🤪 on `skysh` !

You can learn more about installation [here](https://terrabasedb.github.io/docs/Getting-Started/)

## Features

* **Insanely fast**: Scale to millions of queries per second per node
* **Key/value store**: `GET` , `SET` , `UPDATE` and [all that stuff](https://terrabasedb.github.io/docs/List-Of-Actions)
* **Snapshots**: Automated (and tunable) snapshots for stress-free backups
* **Secure**: Secure connections are built into Skybase with SSL/TLS
* **Multithreaded**: Designed to exploit all CPU cores
* **Resource friendly**: The database server doesn't need more than 1MB to run
* **Convenient**: Without the setup hassle and system-specific dependencies

(... [and more on the way](https://github.com/skybasedb/skybase/labels/roadmap))

## Clients 🔌

Until we release an official driver, you'll have to write your own clients — all you need to do is implement the simple and performant [Terrapipe protocol spec](https://docs.skybasedb.com/Protocols/terrapipe/).

## Community 👐

A project which is powered by the community believes in the power of community! If you get stuck anywhere - here are your options!
<html>
<a href="https://gitter.im/skybasedb/community"><img src="https://img.shields.io/badge/chat%20on-gitter-ed1965?logo=gitter&style=flat-square"></img>
</a><a href="https://discord.gg/vJY5sf69fR"><img src="https://img.shields.io/badge/talk-on%20discord-7289DA?logo=discord&style=flat-square"></img></a>
</html>

## Platforms 💻

![Linux supported](https://img.shields.io/badge/Linux%20x86__64-supported%20✓-228B22?style=flat-square&logo=linux) ![macOS supported](https://img.shields.io/badge/macOS%20x86__64-supported%20✓-228B22?style=flat-square&logo=apple) ![Windows supported](https://img.shields.io/badge/Windows%20x86__64-supported%20✓-228B22?style=flat-square&logo=windows)

## Versioning 

This project strictly follows semver, however, since this project is currently in the development phase (0.x.y), the API may change unpredictably

## Contributing

[![Contribute Now](https://img.shields.io/badge/%F0%9F%8C%9Fcontribute-now-a94064?style=for-the-badge)](https://ohsayan.github.io/thanks)

**Yes - we need you!** Be it a typo, a bizarre idea, a dirty bug🐞 or an amazing patch - you're welcome to contribute to Skybase! Beginner friendly issues are marked with the [<img src=https://img.shields.io/badge/L--easy-C71585>](https://github.com/skybasedb/skybase/labels/L-easy) label. Read the guide [here](./CONTRIBUTING.md).

## Contributors

You can see a full list of contributors [here](https://ohsayan.github.io/thanks)

## License

First [know your rights](https://medium.com/swlh/understanding-the-agpl-the-most-misunderstood-license-86fd1fe91275)! This project is licensed under the [AGPL-3.0 License](./LICENSE).
