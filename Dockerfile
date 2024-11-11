FROM amazonlinux@sha256:ade8904b6915ab42d0c6ef10d9686921b4ac022b20d9a4c448446fbf02db9f9d
RUN dnf install aws-nitro-enclaves-cli aws-nitro-enclaves-cli-devel -y
ENTRYPOINT ["/bin/bash", "-c", "nitro-cli build-enclave --output-file /output/${EIF_FILE} --docker-uri ${DOCKER_IMAGE} && nitro-cli describe-eif --eif-path /output/${EIF_FILE} > /output/${INFO_FILE}"]
