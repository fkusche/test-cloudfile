# Checking, if a cloud file is present locally

This is just a small test program.
It will display the values of the following file attributes:
- `FILE_ATTRIBUTE_PINNED`
- `FILE_ATTRIBUTE_UNPINNED`
- `FILE_ATTRIBUTE_RECALL_ON_OPEN`
- `FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS`

It seems, that a file is present locally, if `FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS` is false.

See also https://stackoverflow.com/a/61391049

The StackOverflow answer says, that this attribute is also valid for folders. However, I was not able to create a folder remotely and then have `FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS == true`. (I tried OneDrive and iCloud.) It seems that (empty) folders are always created.
