# furry-tribble

A tool to get the AQI

## To make it up and running

go to the [Air Quality Programmatic APIs](https://aqicn.org/data-platform/token/) and get yourself a Token.

After you get the token, you have to create a new file in the [/src](.\src) called token.rs
and add the token in the new file. Like so.

```rust
    pub const AQI_TOKEN :&str = "<YOUR_TOKEN>";
```

---

## BUILDING

```terminal
    git clone https://github.com/Eshanatnight/furry-tribble.git
```

```terminal
    cargo b --release
```

---

## Interface

```terminal
    USAGE:
        aqi.exe <SUBCOMMAND>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    SUBCOMMANDS:
        help      Prints this message or the help of the given subcommand(s)
        info      Get the Info by suppling the <url>
        search    Search for a place(Station) by name
```

---

### Subcommands

|Name   |Argument|Description                |
|-------|--------|---------------------------|
|info   |url     |url of the station to view |
|search |terms   |name of the place to search|
