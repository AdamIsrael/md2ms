# md2ms

`md2ms` is a command-line tool to convert Markdown file(s) to Shunn's [Standard Manuscript Format](https://www.shunn.net/format/story/1/) in `docx` format. It also provides an integration with [Obsidian](https://obsidian.md/) via the [ShellCommands](https://github.com/Taitava/obsidian-shellcommands) and [Commander](https://github.com/phibr0/obsidian-commander) plugins so that you can easily generate manuscripts or view your word count from your Obsidian vault.

This was partially inspired by [Tobias Buckell](https://bsky.app/profile/tobiasbuckell.bsky.social)'s [usage of Obsidian](https://bsky.app/profile/tobiasbuckell.bsky.social/post/3ljobdnprxs27) to write, track and [analyze](https://bsky.app/profile/tobiasbuckell.bsky.social/post/3ljoab7k34c2a) his fiction work.

I've been using Obsidian for a Personal Knowledge Management system, and want to use it as a cross-platform writing application, but the difficulty of generating manuscripts from Markdown files was a barrier.

`md2ms` is a work-in-progress but is currently in a functional state. It is an opinionated tool, based on my personal preferences, but I think it covers at least 80% of use-cases. Given the path to Markdown file(s) in an Obsidian vault, it will generate four manuscripts:

- Classic Manuscript (Courier New) Format w/Personally Identifying Information (PII)
- Classic Manuscript (Courier New) Format w/o Personally Identifying Information (PII)
- Modern Manuscript (Times New Roman)  Format w/Personally Identifying Information (PII)
- Modern Manuscript (Times New Roman) Format w/o Personally Identifying Information (PII)

A vast majority of publishers prefer Standard Manuscript Format, so this quickly generates manuscripts ready for submission.

## Installation

TBD

## Usage (Obsidian)

To enable the integration with Obsidian, you can use the following command:

```
md2ms obsidian /path/to/obsidian/vault
```

This will download, install, and activate the two required Obsidian plugins, which will then be configured for use with `md2ms`. You can run the command multiple times to update the plugins or to reinstall missing shortcuts.

**NOTE**: You will need to re-open your vault after running this command in order for the changes to take effect.

In Obsidian, right-click on the folder containing your Markdown files and choose "Export to Standard Manuscript Format".

## Usage (CLI)

```bash
md2ms compile ~/path/to/vault/Writing/Fiction/Short/Template/Draft \
--pii ~/Documents/Writing/PII.md \
--output-dir ~/path/to/Writing/Drafts
```

## Personally Identifying Information (PII)

Most manuscripts require your personal information, such as legal name, address, email address, etc. You will need to create a `PII.md` file in the root of your vault or writing folder, with the following metadata:

```yaml
---
legal_name: John Q. Doe
address1: 123 Main Street
address2: Apt. 1408
city: Anytown
state: IL
country: USA
postal_code: 55555
phone: 1-555-867-5309
email: john@jqpublic.com
affiliations:
  - SFWA
  - HWA
---
```

## Story Structure

`md2ms` is designed to be flexible when it comes to story structure. You could have a single Markdown document containing your entire story, a Markdown document per scene, a folder per chapter, or even a folder to separate acts in a longer work.

Each Markdown file should contain some metadata:


You should also create a `metadata` Markdown for the story itself, which informs `md2ms` of pertinent information when compiling the manuscript:

```yaml
---
author: "Adam Israel"
title: "The Great Canadian Short Story"
short_title: "Canadian"
short_author: "Israel"
content_warnings:
  - "violence"
  - "death"
  - "gore"
  - "blood"
include:
- scene1.md
- scene2.md
---
```

Most of this is self-explanatory, but the `include` block is special. It lists the Markdown files that make up your manuscript. This allows you to keep research, notes, reader feedback, etc. in the same folder as your manuscript.

## TODO:

- Footnotes: Strip them out or format them properly?
- Comments: Strip them out.
- Content Warnings: Figure out how to best add a CW block to the manuscript.

## Obsidian Integration Details

Consider the following structure:

```
The Great Canadian Short Story
├── Draft
│   ├── metadata.md
│   ├── scene1.md
│   └── scene2.md
└── Research
    └── Outline.md
```

In this contrived example, we have two scenes and metadata in the `Draft` folder. This is what `md2ms` will use when compiling the manuscript.

Here's another example, of a [three-act novella](./examples/novella_with_parts/):

```
.
├── Act 1
│   ├── Chapter 1
│   │   ├── scene 1.md
│   │   ├── scene 2.md
│   │   └── scene 3.md
│   ├── Chapter 2
│   │   ├── scene 1.md
│   │   ├── scene 2.md
│   │   └── scene 3.md
│   ├── Chapter 3
│   │   ├── scene 1.md
│   │   ├── scene 2.md
│   │   └── scene 3.md
│   └── metadata.md
├── Act 2
│   ├── Chapter 4
│   │   ├── scene 1.md
│   │   ├── scene 2.md
│   │   └── scene 3.md
│   ├── Chapter 5
│   │   ├── scene 1.md
│   │   ├── scene 2.md
│   │   └── scene 3.md
│   ├── Chapter 6
│   │   ├── scene 1.md
│   │   ├── scene 2.md
│   │   └── scene 3.md
│   └── metadata.md
├── Act 3
│   ├── Chapter 7
│   │   ├── scene 1.md
│   │   ├── scene 2.md
│   │   └── scene 3.md
│   ├── Chapter 8
│   │   ├── scene 1.md
│   │   ├── scene 2.md
│   │   └── scene 3.md
│   ├── Chapter 9
│   │   ├── scene 1.md
│   │   ├── scene 2.md
│   │   └── scene 3.md
│   └── metadata.md
├── metadata.md
└── PII.md
```

## Bugs

- Remove extra scene breaks (`#`) after the title, after a header ("Act 1"), and at the end of a chapter.

### Configuration Files

Right now there is no configuration file for `md2ms`. All options are passed via command-line arguments.

### Scene Breaks

There are the supported types of scene breaks that are auto-deteected:
- `* * *`
- `\#`
- `#`
- Two or more blank lines
