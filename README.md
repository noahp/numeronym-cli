# numeronym-cli

A Rust CLI tool that generates numeronym abbreviations because apparently we needed to make text even less readable than it already is.

## What is this abomination?

A numeronym is when you take a perfectly good word and replace the middle letters with a number representing how many letters you brutally removed. Because why write "internationalization" when you can write the much more "intuitive" "i18n"? It's like abbreviations, but with extra steps and mathematical anxiety.

Examples of this linguistic vandalism:
- `internationalization` → `i18n` (18 letters between 'i' and 'n')
- `accessibility` → `a11y` (11 letters between 'a' and 'y')
- `kubernetes` → `k8s` (8 letters between 'k' and 's')

## Installation

```bash
cargo build --release
# The binary will be at target/release/numeronym-cli
```

Or install it globally:
```bash
cargo install --path .
```

## Usage

Pass one or more words as arguments. The tool preserves punctuation and whitespace because even chaos needs some order:

```bash
❯ numeronym-cli "it's super fun making idiotic abbreviations!"
zsh: command not found: numeronym-cli
```

Wait, you need to actually build it first. Try again:

```bash
❯ ./target/release/numeronym-cli "it's super fun making idiotic abbreviations!"
i1s s3r f1n m5g i5c a11s!
```

```bash
❯ numeronym-cli internationalization accessibility kubernetes
i18n a11y k8s
```

```bash
❯ numeronym-cli "Hello, world!"
H3o, w3d!
```

## Why does this exist?

Great question! I ask myself the same thing every day. Apparently the tech industry decided that normal abbreviations weren't confusing enough, so we needed to add arithmetic to reading comprehension.

But hey, at least now you can make your documentation 15% more cryptic while still technically using "real words." Your future self will thank you when trying to remember what `l10n` means at 2 AM.

## Features

- ✅ Converts words to numeronyms (unfortunately)
- ✅ Preserves punctuation (the only good thing here)
- ✅ Handles multiple words in a single command
- ✅ Makes your text objectively worse to read
- ✅ Adds unnecessary complexity to simple abbreviations

## License

MIT - Because even terrible ideas deserve freedom.
