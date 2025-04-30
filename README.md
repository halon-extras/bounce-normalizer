# Bounce normalizer plugin

The bounce-normalizer plugin is a powerful extension for Halon email platforms that standardizes and normalizes email bounce messages (diagnostic codes). It processes smtp error messages and converts them into a consistent, normalized format.

This plugin helps email administrators by:
- Standardizing different bounce message formats from various email providers
- Enabling more consistent bounce handling and reporting

## Bounce Message Normalization Example

Here's an example showing how the bounce-normalizer plugin transforms a Google SMTP bounce message:

### Input Message (Original Bounce)
```
550 5.7.1 The email account that you tried to reach does not exist. Please try double-checking the recipient's email address for typos or unnecessary spaces. For more information, go to https://support.google.com/mail/?p=NoSuchUser 38308e7fff4ca-317d13bdb9esi41173941fa.224 - gsmtp
```

### Output Message (After Normalization)
```
550 5_7_1 the email account that you tried to reach does not exist please try double checking the recipient's email address for typos or unnecessary spaces for more information go to __url__ google __uuid__ gsmtp
```

### Key Transformations
The plugin performs several normalizing operations:
- Transforms all text to lowercase
- Removes unnecessary punctuation
- Replaces URLs with the placeholder `__url__`
- Replaces unique identifiers/UUIDs with `__uuid__`
- Converts SMTP status codes from dot notation (`5.7.1`) to underscore format (`5_7_1`)

**Note:** This example demonstrates only the most common transformations. The plugin may perform additional normalization operations depending on the specific bounce message format and content.

This normalization makes it easier to categorize, process, and respond to bounce messages consistently, regardless of their original formatting or source server.

## Installation

Follow the [instructions](https://docs.halon.io/manual/comp_install.html#installation) in our manual to add our package repository and then run the below command.

### Ubuntu

```
apt-get install halon-extras-bounce-normalizer
```

### RHEL

```
yum install halon-extras-bounce-normalizer
```

## Exported functions

These functions needs to be [imported](https://docs.halon.io/hsl/structures.html#import) from the `extras://bounce-normalizer` module path.

### bounce_normalizer(diagnosticcode)

Normalize a bounce message (diagnosticcode).

**Params**

- diagnosticcode `string` - The message to normalize

**Returns**

Return the bounce message (diagnosticcode) normalized.

**Example**

```
import { bounce_normalizer } from "extras://bounce-normalizer";
echo bounce_normalizer("550 5.7.1 The email account that you tried to reach does not exist. Please try double-checking the recipient's email address for typos or unnecessary spaces. For more information, go to  https://support.google.com/mail/?p=NoSuchUser 38308e7fff4ca-317d13bdb9esi41173941fa.224 - gsmtp");
```