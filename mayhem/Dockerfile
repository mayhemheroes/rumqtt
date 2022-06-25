# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir rumqttc/fuzz && \
    mv rumqttc/fuzz/target/x86_64-unknown-linux-gnu/release/mqttbytes_read /mqttbytes_read && \
    echo done

RUN echo building non-instrumented harnesses && \
    cargo +nightly -Z sparse-registry fuzz build --fuzz-dir rumqttc/fuzz --disable-instrumentation && \
    mv rumqttc/fuzz/target/x86_64-unknown-linux-gnu/release/mqttbytes_read /mqttbytes_read_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /mqttbytes_read  /mqttbytes_read_no_inst /
