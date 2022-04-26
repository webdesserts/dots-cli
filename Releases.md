# Publishing Release

## Major Releases
- Create corrisponding release branch (e.g. v1.0.0)
- Merge PRs into the release branch as they're completed
- Bump version in the release branch
- Create Release PR
- Make sure all included PRs are properly tagged
- Create Draft Release
- Run `cargo publish --dry` on **each** crate
- Merge to `master`
- Run `cargo publish` on **each** crate
- Publish github release
