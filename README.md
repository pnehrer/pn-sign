# PubNub Request Signature Utility

Library and command-line utility for signing PubNub API requests using a shared secret.

See [PubNub Acccess Manager](https://www.pubnub.com/docs/pubnub-rest-api-documentation#pubnub-access-manager-pam) documentation for additional details.

## Installation

```bash
cargo install pn-sign
```

## Usage

See:

```bash
pn-sign --help
```

## Example

To sign and execute a [Grant API](https://www.pubnub.com/docs/pubnub-rest-api-documentation#pubnub-access-manager-pam-pubnub-access-manager-get) request:

```bash
pn-sign 'https://ps.pndsn.com/v2/auth/grant/sub-key/sub-c-c10cec8f-80f3-4ab6-8ab5-cb5b49e010c7?auth=auth-key-1&uuid=admin-1&target-uuid=user-1&ttl=15&g=1&u=1&d=1' --sub sub-c-c10cec8f-80f3-4ab6-8ab5-cb5b49e010c7 --pub pub-c-0f9faa0e-52b4-407c-b87a-d2e13409de13 --sec sec-c-Nzk2ZTU2MTYtYzBmYS00ZGI0LWE3MmEtYTk5NzYxM2NkYTNkCg --curl | xargs curl -i
```
