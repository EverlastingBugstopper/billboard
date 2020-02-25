# Changelog

## 🐛 0.0.2-beta

### Fixes

- **Properly display `Boxx` when content includes ANSI escapes - [EverlastingBugstopper], [issue/3] [pull/4]**

  A common use case for eventual `Boxx` consumers will be displaying boxxed content with colors. Strings include [ANSI escapes](http://ascii-table.com/ansi-escape-sequences.php) which are then interpreted by terminals. Since there is more information, they require more bytes, which means the visual width must be calculated independently from the byte width. `Boxx` now strips ANSI escapes from strings when calculating the padding and margin for the content.

  [EverlastingBugstopper]: https://github.com/EverlastingBugstopper
  [pull/4]: https://github.com/EverlastingBugstopper/boxx/pull/4
  [issue/3]: https://github.com/EverlastingBugstopper/boxx/issues/3

### Maintenance

- Some examples were added to the documentation.
