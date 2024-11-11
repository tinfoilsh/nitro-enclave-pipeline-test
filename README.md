# Tinfoil Enclave Attestation

## Verify Enclave Attestation

### 1. Download enclave image

```bash
TF_HELPER_VERSION=v0.0.1
curl -LO "https://github.com/tinfoilanalytics/nitro-enclave-pipeline-test/releases/download/$TF_HELPER_VERSION/tinfoil-helper-enclave-$TF_HELPER_VERSION.eif"
```

### 2. Verify Attestation (chose one method)

#### Option A: Verify in one-line with the [GitHub CLI](https://cli.github.com/)

```bash
gh attestation verify --repo tinfoilanalytics/nitro-enclave-pipeline-test tinfoil-helper-enclave-$TF_HELPER_VERSION.eif
```

#### Option B: Manual Verification

#### 2.1. Download Attestation Document

```bash
DIGEST="sha256:$(sha256sum "tinfoil-helper-enclave-$TF_HELPER_VERSION.eif" | cut -d ' ' -f 1)"
curl -sL "https://api.github.com/repos/tinfoilanalytics/nitro-enclave-pipeline-test/attestations/$DIGEST" | jq -r ".attestations[0].bundle" > attestation.jsonl
```

#### 2.2. Verify Attestation with [cosign](https://github.com/sigstore/cosign)

```bash
cosign verify-blob-attestation \
  --new-bundle-format \
  --bundle attestation.jsonl \
  --certificate-oidc-issuer="https://token.actions.githubusercontent.com" \
  --certificate-identity-regexp="^https://github.com/tinfoilanalytics/nitro-enclave-pipeline-test/.github/workflows/release.yml.?" \
  "tinfoil-helper-enclave-$TF_HELPER_VERSION.eif"
```
