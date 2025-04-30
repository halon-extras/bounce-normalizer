# Bounce normalizer plugin

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
echo bounce_normalizer("550 5.7.1 5.1.1 The email account that you tried to reach does not exist. Please try double-checking the recipient's email address for typos or unnecessary spaces. For more information, go to  https://support.google.com/mail/?p=NoSuchUser 38308e7fff4ca-317d13bdb9esi41173941fa.224 - gsmtp");
```