FROM ubuntu:24.04

COPY dependencies.sh /
RUN sh dependencies.sh

ENV TZ=Europe/Moscow
ENV LANG C.UTF-8
ENV LC_ALL C.UTF-8
ENV PATH="/root/.cargo/bin:${PATH}"
ENV CARGO_HOME="/root/.cargo"

# Source cargo environment on shell startup
RUN echo 'source /root/.cargo/env' >> /root/.bashrc
