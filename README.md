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
- Simple formatting: bold, italics, underline, and strikethrough.
- Disable Smart Quotes?
  - Looks like they're disabled by default in the generated docx. "Correcting" quotes is an auto-correct feature that should be turned off in Word itself's AutoCorrect settings.
- ~~Strip/handle links~~
	- Hyperlinks, hashtags, and other simple formatting. What to do with them? Pretty sure that’s getting parsed and inserted into the docx without reformat. Should probably have config options for default behavior, which would be to strip the links and apply the formatting.
	- Technically possible, maybe, would be to format links as footnotes. That should be opt-in, though. This is fiction, so there are very little footnotes as a standard.
	- This should help with cross-referencing stuff in a story, so I can link documents and not worry about that bleeding into the manuscript.
- Position the title/byline in the center of the page, based on the # of lines in the header. Something like 47 lines per page, minus header lines, divided by two.
- Support anonymous manuscripts. Don’t put PII information into a manuscript when not wanted.
- Ponder the idea of generating multiple manuscripts per execution, like:
	- Classic vs. Modern Manuscript format (also add a CLI flag/config)
	- Times New Roman and Courier
	- Anonymous and PII
- Custom header - some people may want phone number, or professional affiliations like SFWA. I think I can just base that on presence in the metadata. Should I add a flag to accept the path to a separate metadata document that can track author information that won’t necessarily be in the story metadata? Like, author, short_author, affiliations, and other header information doesn’t need to be repeated in the metadata of a story.

## Documentation

### PII - Author's Personally Identifiable information

Unless the `--anonymous`
### Scene Breaks

There are three types of scene breaks that are auto-deteected:
- * * *
- \#
- Two or more blank lines
