[releases]: https://github.com/eduardobaginskicosta/devns/releases
[docker]: https://hub.docker.com/r/baginskistudio/devns
[repo]: https://github.com/eduardobaginskicosta/domainnamesystem

[citation]: https://docs.github.com/github/creating-cloning-and-archiving-repositories/creating-a-repository-on-github/about-citation-files
[rust]: https://rust-lang.org/tools/install/
[pwsh]: https://learn.microsoft.com/en-us/powershell/scripting/install/linux-overview
[wsl]: https://learn.microsoft.com/en-us/windows/wsl/install

[social_insta]: https://www.instagram.com/eduardobaginskicosta/
[social_yt]: https://www.youtube.com/@baginskistudio
[social_in]: https://www.linkedin.com/in/eduardobaginskicosta/

# DevNS (Development Name Server)

**DevNS** is a DNS server written in **Rust**, designed to provide a lightweight,
efficient, and reliable solution for development environments. It is the sucessor
to the [**domainnamesystem**][repo] project, introducting significant improvements
in performance, stability, protocol compliance, and overall architecture.

This project was created to address limitations identified in its predecessor,
including packet parsing inconsistencies and compatibility issues on system that
enforce stricter adherence to **RFC 1034** and **RFC 1035*** specifications. The
result is a cleaner, more robust implementation focused on pratical use in modern
development workflows.

> **Important:** DevNS is intended for development, testing, and internal
> infrastructure environments. Although it has been designed to handle
> substantial request volumes, it should not be considered production-ready
> at this stage.

**Quick Start (Docker):**
```bash
git clone https://github.com/eduardobaginskicosta/devns-docker
```

<!-- = = = -->

## 📦 Local Execution and Docker Deployment

Unlike **domainnamesystem**, which was primary development as an academic and
experimental project, **DevNS** focuses on pratical usage scenarios where
developers an teams require custom DNS resolution for local services,
internal domains, testing environments, and development infrastructure.

The project can be used in two different ways:

### Local Execution

Precompiled binaries for **Linux** and **Windows** are available through the
project's [GitHub releases][releases] page. Simply download the appropriate
package, extract the files, and run the executable.

If you prefer building from source, refer to the local compilation section below.

### Docker Container

DevNS is also available as an [official Docker image][docker]:
```bash
docker pull baginskistudio/devns
```
Detailed instructions for building and deploying the container can be found in the
Docker section of this documentation.

<!-- = = = -->

## 🦀 Building from Source

To build DevNS locally, the following dependencies are required:
- [Rustup Toolchain][rust].
- MinGW-W64 (for Windows cross-compilation).
- GIT.

If Rust is not installed on your system, download it from the official
[Rust website][rust]. Git and MinGW-W64 can be installed using your system's
package manager:
```bash
# Ubuntu, Debian, Linux Mint
sudo apt update
sudo apt install mingw-w64 git

# Fedora, Red Hat
sudo dnf install mingw64-gcc mingw64-gcc-c++ git

# Arch Linux
sudo pacman -S mingw-w64-gcc git
```

DevNS supports three build models:
| **Mode**      | **Rust Target**             | **Use Case**                                       |
|---------------|-----------------------------|----------------------------------------------------|
| Native        | Host target                 | Local development on the current system            |
| Linux (MUSL)  | `x86_64-unknown-linux-musl` | Docker, Alpine Linux, and portable static binaries |
| Windows (GNU) | `x86_64-pc-windows-gnu`     | Windows deployments                                |

If you plan to build the Linux MUSL or Windows versions, install the corresponding
[Rust][rust] targets:
```bash
# Linux x86_64 MUSL
rustup target add x86_64-unknown-linux-musl

# Windows x86_64 GNU
rustup target add x86_64-pc-windows-gnu
```

The native build uses the default target of the host machine. For example, on
Ubuntu, Debian, Fedora, Arch Linux and must Linux distributions, the native
target is typically: `x86_64-unknown-linux-gnu`.

This target is recommended for local development and testing.

The Linux MUSL target is intended primarily for containerized deployments. It
produces a portable binary compatible with minimal container images such as
**Alpine Linux** and **Scratch**, making it the recommended choice for
Docker images.

Once the dependencies are available, clone the repository:
```bash
git clone https://github.com/eduardobaginskicosta/devns.git
cd devns
```

Development is primarily targeted at **Linux** systems. Windows users are
strongly encouraged to use the [Windows Subsystem for Linux (WSL)][wsl]
alongside [PowerShell][pwsh] for a more consistent development experience.

To build the project:

```bash
# Native build (release)
./scripts/build.sh --release

# Linux x86_64 MUSL target (release)
./scripts/build.sh --linux --release

# Windows x86_64 GNU target (release)
./scripts/build.sh --windows --release
```
After a successful build, run the server with administrative privileges:
```bash
# Native Linux build
sudo ./scripts/run.sh --release

# Linux x86_64 MUSL build
sudo ./scripts/run.sh --linux --release

# Windows x86_64 GNU build
sudo ./scripts/run.sh --windows --relase
```

### Configuration Directory

During startup, DevNS loads DNS zone files from a `config` directory located
relative to the current working directory (CWD). Because of this, the server
must be executed from a location where the configuration directory remains
accessible.

If zone files cannot be found, DevNS will continue operating as a forwarding
resolver, but locally defined zones will not be available.

### Verifying the Installation

By default, DevNS listens for DNS request on port `53` of the configured network
interface. The repository includes demonstration zones out of the box, including:
- `host`
- `server`
- `server.local`

These domains resolve to the local loopback address (`127.0.0.1` and `::1`).
You can verify the installation using:

```bash
# Linux & Windows
nslookup host DNS_IP
nslookup host 192.168.0.10 # example

# Linux
dig @DNS_IP host
dig @192.168.0.10 host # example
```
A successful response indicates that the DNS server is running correctly and
serving the configurated zone data.
```txt
; <<>> DiG 9.20.18-1ubuntu2.1-Ubuntu <<>> @192.168.0.10 host
; (1 server found)
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 63151
;; flags: qr rd ra; QUERY: 1, ANSWER: 2, AUTHORITY: 1, ADDITIONAL: 1

;; QUESTION SECTION:
;host.				IN	A

;; ANSWER SECTION:
host.			3600	IN	A	127.0.0.1
host.			3600	IN	AAAA	::

;; AUTHORITY SECTION:
host.			3600	IN	NS	ns1.host.

;; ADDITIONAL SECTION:
host.			3600	IN	MX	10 mail.host.

;; Query time: 0 msec
;; SERVER: 192.168.0.10#53(192.168.0.10) (UDP)
;; WHEN: Thu Jun 04 14:57:37 -03 2026
;; MSG SIZE  rcvd: 129
```

### Environment Variables

Several aspects of DevNS can be customized through environment variables. These
settings are supported by both local executions and Docker deployments.

> ` DEBUG_MODE `

Enables or disables debug logging output. Accept values:
- `0`, `FALSE`, `False`, `false` -- Disable debug mode.
- `1`, `TRUE`, `True`, `true` -- Enable debug mode.

If an invalid value is provided, debug mode remains enabled by default.

> ` MAX_MESSAGES `

Defines the maximum number of queued messages that each worker can process before
requests begin waiting for available capacity.
- Minimum: `1`
- Maximum: `10000`
- Default: `20`

> ` MAX_WORKERS `

Defines the maximum number of workers threads available to process requestes
concurrently.
- Minimum: `1`
- Maximum: `256`
- Default: `10`

> ` PORT `

Specifies the network port used by the DNS server.
- Minimum: `53`
- Maximum: `9000`
- Default: `53`

> ` DNS_SERVERS `

Define the upstream DNS servers used when DevNS is unable to answer a query from
its local zones. Multiple IPv4 address can be provided using semicolon-separated
values:
```txt
1.1.1.1
1.1.1.1;1.0.0.1
1.1.1.1;1.0.0.1;8.8.8.8
```

If no value is supplied, DevNS falls back to its built-in default resolver
configuration.

> All environment variables have safe defaults defined internally. Invalid
> values are ignored and replaced automatically. The active configuration can
> always be verified through the startup logs.

The repository also includes a collection of example DNS zones inside the
[`config`](./config) directory. These examples demonstrate common development
scenarios, local domains, and DNS overrides for selected public services.

<!-- = = = -->

## 🐋 Building and Running with Docker

DevNS can be deployed using Docker for a simplified installation an management
experience. The repository includes a helper script named
[`docker.sh`](./scripts/docker.sh), which automates the complete build and
deployment workflow.

Available commands:
```bash
# Build Rust binaries, create the Docker image and start the container
./scripts/docker.sh

# Build Rust binaries and create the Docker image only
./scripts/docker.sh --build-only

# Stop and remove the running container
./scripts/docker.sh --down
```

Running the script without additional parameters performs a complete release build,
creates the Docker image locally, and launches the container automatically.
By default, the DNS service is exposed through port `53` on the host machine.

Once the container is running, you can verify the installation using the same
commands shown in the local execution section:

```bash
# Linux and Windows
nslookup host DNS_IP
nslookup host 192.168.0.10 # example

# Linux
dig @DNS_IP host
dig @192.168.0.10 host # example
```
If the configuration remains unchanged, the included demonstration zones should return loopback addresses as expected.
```txt
; <<>> DiG 9.20.18-1ubuntu2.1-Ubuntu <<>> @192.168.0.10 host
; (1 server found)
;; global options: +cmd
;; Got answer:
;; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: 63151
;; flags: qr rd ra; QUERY: 1, ANSWER: 2, AUTHORITY: 1, ADDITIONAL: 1

;; QUESTION SECTION:
;host.				IN	A

;; ANSWER SECTION:
host.			3600	IN	A	127.0.0.1
host.			3600	IN	AAAA	::

;; AUTHORITY SECTION:
host.			3600	IN	NS	ns1.host.

;; ADDITIONAL SECTION:
host.			3600	IN	MX	10 mail.host.

;; Query time: 0 msec
;; SERVER: 192.168.0.10#53(192.168.0.10) (UDP)
;; WHEN: Thu Jun 04 14:57:37 -03 2026
;; MSG SIZE  rcvd: 129
```

All environment variables documented in the previous section are fully supported
when running DevNS inside a Docker container.

> The provided [`docker-compose.yaml`](./docker/docker-compose.yaml) already
> includes sensible defaults for most development scenarios. When customizing
> values, pay close attention to spelling and formatting, as invalid entries
> automatically fall back to the built-in defaults.

<!-- = = = -->

## ⚙️ DNS Zone Configuration

DevNS uses a straightforward zone-file format designed to be easy to read,
maintain, and extend.

All DNS zones are loaded from files whose names begin with `zone.` inside the
config directory. Each zone file can define one or more domains, including their
associated records and metadata.

Example:
```zone
@ ZONE: example.dev, example.local
@ TTL: 3600
@ NS: localhost
@ MX: mail.localhost
@ A: $LOCALHOST
@ AAAA: $LOCALHOST
```

The example above creates the domains `example.dev` and `example.local`, both
resolving to the IPv4 and IPv6 loopback addresses.

The special keyword `$LOCALHOST` is automatically expanded to the appropriate
loopback values and is recommended whenever local development targets are required.
By default, development-oriented zones use relatively short cache lifetimes to
reduce propagation delays during testing.

Multiple IPv4 and IPv6 addresses can also be declared:
```zone
@ A: 172.66.147.243, 104.20.23.154
```

Zone files are discovered recursively throughout the configuration directory
structure, allowing larger deployments to organize configurations across multiple
folders without affecting functionality.

Bem como, pode-se declarar que a **zona** não possua endereços IPV4 ou IPV6
deixando os campos em branco. Importante: ao menos um dos campos deve ser declarado,
caso contrário, será retornado um erro pois o **devns** responderá com dados vazios,
algo que os resolvedores como `dig` e `nslookup` não aceitam, pois ele não repassará
para os **lookups** responderem.

### Zone Discovery Behavior

When DevNS starts, it searches for zone files relative to the process working
directory. For example, if the server is started from `/`, DevNS will attempt to
load configuration files from `/config`.

If no valid zones are found, the server continues operating as a forwarding
resolver, delegating requests to upstream DNS providers whenever necessary.

### Native Domain Blocking

DevNS includes a built-in mechanism for blocking domains directly through zone
definitions. A zone can be configured to return invalid or null addresses,
effectively preventing successful resolution:

```zone
@ A: 0.0.0.0
@ AAAA: 0:0:0:0:0:0:0:0
```
Or with the special keyword `$BLOCK`:
```zone
@ A: $BLOCK
@ AAAA: $BLOCK
```

This feature can be used for local content filtering, development testing, parental
control scenarios, or custom DNS-based restrictions. Keep in mind that
**DNS-over-HTTPS (DoH)** traffic bypasses traditional DNS resolution paths and
therefore cannot be controlled through standard DNS zone overrides alone.

<!-- = = = -->

## 📃 License and Contributions

DevNS is distributed under the terms os the **BSD-3-Clause License**.

For a complete licensing information, please refer to the [**LICENSE**](./LICENSE)
file included in the repository.

Contributions of all sizes are welcome. If you would like to participate in the
development of the project, consider:
- Reporting bugs or unexpected behavior through GitHub Issues.
- Suggesting improvements or new features.
- Submitting feedback from real-world usage.
- Supporting the project by **starring** the repository.

Every contribution helps improve the project and is greatly appreciated.

<!-- = = = -->

## 🤝 Support the Project and Follow My Work

If DevNS has ben usedul to you, your team, or your organization, please consider
supporting its continued development.

You can also follow my work through the following platforms:
- [**Instagram**][social_insta] -- Personal updates, behind-the-scenes content, and
  ongoing projects.
- [**YouTube**][social_yt] -- Development-related content, technical projects, and
  demonstrations.
- [**LinkedIn**][social_in] -- Professional updates, technical articles, and
  industry-related discussions.

Your support, feedback, and engagement help keep projeccts like DevNS actively
maintained and continuously envolving.

<!-- = = = -->

## 👨‍🏫 Citation

If you use DevNS in academic work, research projects, technical reports, or
publications, please consider citing the project appropriately.

For additional information regarding citation files on GitHub, see:
[About CITATION Files][citation].

### APA Format
```APA
Baginski Costa, E. (2026). DevNS (Development Name System) (Version 0.1.0) [Computer software]. https://github.com/eduardobaginskicosta/devns
```

### BibTeX Format
```BibTeX
@software{Baginski_Costa_DevNS_Development_Name_2026,
author = {Baginski Costa, Eduardo},
month = may,
title = {{DevNS (Development Name System)}},
url = {https://github.com/eduardobaginskicosta/devns},
version = {0.1.0},
year = {2026}
}
```
