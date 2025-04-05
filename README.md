# md2ms

Convert Markdown file(s) to Shunn's [Standard Manuscript Format](https://www.shunn.net/format/story/1/) in docx format.

This is a work-in-progress and is not yet ready for general use. It is being developed for my own use, to generate manuscripts for submission to publishers based on the contents of an [Obsidian](https://obsidian.md/) vault.

## TODO

- Subcommands?
  - compile
  - Obsidian
  - wordcount
- Footnotes?
- Dynamically position the title/byline in the center of the page, based on the # of lines in the header. Something like 47 lines per page, minus header lines, divided by two.
- Ponder the idea of generating multiple manuscripts per execution, like:
	- Classic vs. Modern Manuscript format (also add a CLI flag/config)
	- Times New Roman and Courier
	- Anonymous and PII

### Obsidian Integration

Plugins needed:
- Shell commands
- Commander

Need to add metadata for the output filename, maybe?

Consider the following structure:

```bash
└── Story Name
    ├── Draft
    │   ├── metadata.md
    │   ├── scene1.md
    │   └── scene2.md
    └── Research.md
```

Either md2ms needs to be aware of structure, so when we run export against `Story Name` it looks in `Draft` to find the story and metadata and creates `Story Name.docx`, or we run the export command against the `Draft` directory and the `folder_name` variable is set to `Draft` and we create `Draft.docx`, or we explicitly use the title in the metadata to create `Story Name.docx` (which is how it currently works).


### Configuration Files

I'm considering adding support for a configuration file to set defaults for the CLI. The benefits are that it could shorten the command-line arguments and allow greater/easier customization. On the flip side, this is meant to be opinionated on purpose. By default, it should generate a manuscript that is as close to Shunn's standard format as possible.

I just need to think through how the configuration file should be used and if it's even necessary. I may hold off on a decision until I do some of the Obsidian integration. The only configuration option that might be useful globally or per-user is the output directory, which I haven't even wired up yet.

There are at least four different crates that wrap around clap to provide configuration file support.

1. [clap-serde](https://crates.io/crates/clap-serde) - over 2 years old, MIT licensed.
2. [clap-config](https://crates.io/crates/clap_config) - 8 months old, MIT licensed. Single struct, any deserializable format.
3. [clap-conf](https://crates.io/crates/clap_conf) - over 3 years old, MIT licensed.
4. [clap-config-file](https://crates.io/crates/clap-config-file) - 2 months old, MIT licensed. Single struct, yaml.
5. [confy](https://crates.io/crates/confy) - 1 year old, MIT licensed. Not clap-specific. TOML,YAML, or RON.

Or I can implement serde for the `Args` struct and parse and load the configuration file myself.

## Documentation

### PII - Author's Personally Identifiable information

Unless the `--anonymous` flag is present, the author's PII should be included in the final manuscript. This information is provided in the front matter of a Markdown document.

```bash
md2ms --pii examples/pii.md examples/novella_with_parts
```

### Anonymous Manuscripts

To strip any PII from your manuscript, use the `--anonymous` flag.

```bash
md2ms --anonymous examples/novella_with_parts
```

### Scene Breaks

There are the supported types of scene breaks that are auto-deteected:
- `* * *`
- `\#`
- `#`
- Two or more blank lines

### Classic vs. Modern manuscript Format

By default, we will generate manuscripts in Modern format. If you really want classic, you can pass the `--classic` flag.
