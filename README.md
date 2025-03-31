# md2ms

Convert Markdown file(s) to Shunn's [Standard Manuscript Format](https://www.shunn.net/format/story/1/) in docx format.

This is a work-in-progress and is not yet ready for general use. It is being developed for my own use, to generate manuscripts for submission to publishers based on the contents of an [Obsidian](https://obsidian.md/) vault.

## TODO

- ~~auto-detecting/inserting and formatting scene breaks~~
- ~~Multiple Markdown documents~~
- ~~Multiple font support, i.e., Times New Roman and Courier.~~
	- ~~Let the user specify the font~~
- Add a configuration file (local, global, and CLI flag) that defines standard behavior.
	- Font(s) to use
	- Output location
- Obsidian integration
- ~~Simple formatting: bold, italics, and strikethrough~~
- ~~Strip/handle links~~
	- Hyperlinks, hashtags, and other simple formatting. What to do with them? Pretty sure that’s getting parsed and inserted into the docx without reformat. Should probably have config options for default behavior, which would be to strip the links and apply the formatting.
- Footnotes?
- Dynamically position the title/byline in the center of the page, based on the # of lines in the header. Something like 47 lines per page, minus header lines, divided by two.
- ~~Support anonymous manuscripts. Don’t put PII information into a manuscript when not wanted~~
- Ponder the idea of generating multiple manuscripts per execution, like:
	- Classic vs. Modern Manuscript format (also add a CLI flag/config)
	- Times New Roman and Courier
	- Anonymous and PII

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
- * * *
- \#
- #
- Two or more blank lines
