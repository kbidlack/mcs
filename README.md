# `mcs` - A command line Minecraft server launcher
A command line Minecraft server launcher I made in rust to learn rust and to make my life slightly easier

## Usage
It's a standard cargo project, so to install it clone the repository and run `cargo build --release` to build the binary.

### Creating a server
You can create a server with `mcs create`:
```
mcs create <name>
```
You can also provide a version with:
```
mcs create <name> --version <version>
```

### Listing servers
To see your newly made server, you can use:
```
mcs list
```

### Launching a server
To launch a server:
```
mcs launch <name>
```
And to remove it:
```
mcs remove <name>
```

### Updating a server
If you want to update a server to the latest release:
```
mcs update <name>
```
Or you can specify a version to update to:
```
mcs update <name> --version <version>
```
Note: this *can* be a previous version, but beware, as downgrading can cause major world corruptions!


### Versions
To see a list of versions you can use:
```
mcs versions
```

## How it works
All servers are stored under the `~/.mcservers` folder, and the server files are retrieved from Mojang using the [official version manifest](https://piston-meta.mojang.com/mc/game/version_manifest_v2.json).
