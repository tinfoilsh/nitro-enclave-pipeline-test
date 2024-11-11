#!/bin/bash

REPO=tinfoilanalytics/nitro-enclave-pipeline-test

if [ -n "$1" ]; then
  VERSION=$1
else
  echo "No version provided, fetching latest..."
  VERSION=$(curl -sL "https://api.github.com/repos/$REPO/tags" | jq -r '.[0].name')
fi

ENCLAVE_FILE=tinfoil-helper-enclave-$VERSION.eif

echo "Fetching enclave version $VERSION..."
curl -sLO "https://github.com/$REPO/releases/download/$VERSION/$ENCLAVE_FILE"

SUBJECT_DIGEST="sha256:$(sha256sum "$ENCLAVE_FILE" | cut -d ' ' -f 1)"
echo "$ENCLAVE_FILE $SUBJECT_DIGEST"

echo "Fetching attestation document..."
ATT_DOC=tinfoil-helper-enclave-$VERSION-attestation.jsonl
curl -sL "https://api.github.com/repos/tinfoilanalytics/nitro-enclave-pipeline-test/attestations/$SUBJECT_DIGEST" | jq -r '.attestations[0].bundle' > "$ATT_DOC"

# The attestation document contains a reference to the transparency log entry in SigStore
echo "Transparency log: https://search.sigstore.dev?logIndex=$(jq -r ".verificationMaterial.tlogEntries[0].logIndex" "$ATT_DOC")"

echo "Verifying attestation..."
cosign verify-blob-attestation \
  --new-bundle-format \
  --bundle "$ATT_DOC" \
  --certificate-oidc-issuer="https://token.actions.githubusercontent.com" \
  --certificate-identity-regexp="^https://github.com/$REPO/.github/workflows/release.yml.?" \
  "$ENCLAVE_FILE"
