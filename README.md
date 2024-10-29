# FireCrawl Zed Extension

Simple Zed Extension that adds an `/firecrawl` slash command.

For when `/fetch url` isn't enough.

## Usage

Open the Assistant (`cmd-r`):

```
/firecrawl <url>
```

That's it.

## Setup

1. Generate a [FireCrawl API key](https://www.firecrawl.dev/app/api-keys)
2. Add it to your environment:
```
export FIRECRAWL_API_KEY="fc-1234567890123456789023456789"
```
3. (Optional) Add the above line to your `~/.zshrc` or `~/.bash_profile`
4. Quit Zed.
5. Launch Zed from the CLI:
```
# Make sure you've set it correctly
echo $FIRECRAWL_API_KEY
zed
```
6. Clone this repo somewhere:
```
mkdir -p ~/source/ && cd ~/source
git clone https://github.com/notpeter/firecrawl-zed
```

6. Install the extension: `cmd-shift-x` or `ctrl-shift-x` in Zed, then click "Install Dev Extension" and select `~/source/firecrawl-zed` or wherever you cloned the repo.

See [Usage](#usage) above.

## Screenshots

<img width="525" alt="Screenshot 2024-10-28 at 20 40 00" src="https://github.com/user-attachments/assets/7364922f-7720-4f13-be33-5f77dd326538">

## Links

See also:

- [Zed Assistant: Slash Commands](https://zed.dev/docs/assistant/commands)
- [Zed Extensions: Slash Commands](https://zed.dev/docs/extensions/slash-commands)
- [Slash Commands Example Extension](https://github.com/zed-industries/zed/tree/main/extensions/slash-commands-example)
- [RFC Zed Extension](https://github.com/notpeter/rfc-zed)
