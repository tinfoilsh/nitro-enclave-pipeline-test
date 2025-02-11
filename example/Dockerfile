FROM ghcr.io/tinfoilsh/nitro-attestation-shim:v0.0.8 AS shim
FROM nginxdemos/hello

COPY --from=shim /nitro-attestation-shim /nitro-attestation-shim

ENV NITRO_SHIM_PORT=6000
ENV NITRO_SHIM_UPSTREAM_PORT=80

ENTRYPOINT ["/nitro-attestation-shim", "nginx", "-g", "daemon off;"]
