## 0.4.0 - 2025-01-05

- Updated Dioxus dependency to 0.6.1 and updated this crate to work with Dioxus 6. Based on a PR
  from @jprider63. GH #6.
- When attributes like `class` or `title` are not set, they are no longer included as empty strings
  in the generated HTML, e.g. `class=""`.
- Similarly, the `IconButton` no longer includes an empty `<span></span>` when it does not have any
  child elements.

## 0.3.0 - 2023-08-26

- Update Dioxus dependency to 0.4.0. Based on GH #4 from @atty303.

## 0.2.0 - 2023-01-21

- Update to heroicons v2.0.13. Note that this is a major version bump for the heroicons, and many
  icons in previous releases no longer exist. See
  https://github.com/tailwindlabs/heroicons/releases/tag/v2.0.0 for more details. GH #3. Implemented
  by James Parker.

## 0.1.4 - 2022-04-21

- Fix handling of attributes to be compatible with all versions of Dioxus. GH #2. Fixed by Jon
  Kelley.

## 0.1.3 - 2022-03-12

- Regenerated icons code with the latest heroicons release, v1.0.6. This release has a few changes
  to the solid icons compared to the previous release of dioxus-heroicons.

## 0.1.2 - 2022-03-11

- Update Dioxus dependency to 0.2.0.

## 0.1.1 - 2022-02-04

- The `Icon` component was entirely ignoring its `fill` property and always using `"currentColor"`
  when it was not disabled.

## 0.1.0 - 2022-01-31

- Initial release upon an unsuspecting world.
