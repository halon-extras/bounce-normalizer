FROM --platform=linux/amd64 rockylinux:8
LABEL org.opencontainers.image.authors="support@halon.io"

ARG HALON_REPO_USER
ARG HALON_REPO_PASS

RUN yum update -y && yum install -y epel-release

RUN echo "[halon]" >> /etc/yum.repos.d/Halon.repo
RUN echo "name=Halon repository" >> /etc/yum.repos.d/Halon.repo
RUN echo "baseurl=https://${HALON_REPO_USER}:${HALON_REPO_PASS}@repo.halon.io/dists/rhel8/amd64/stable" >> /etc/yum.repos.d/Halon.repo
RUN echo "gpgcheck=1" >> /etc/yum.repos.d/Halon.repo
RUN echo "gpgkey=https://raw.githubusercontent.com/halon/changelog/master/pgp-keys/7F0A73B5.asc" >> /etc/yum.repos.d/Halon.repo
RUN echo "enabled=1" >> /etc/yum.repos.d/Halon.repo
RUN yum install -y halon-6.4.1

RUN yum install -y cmake3 make gcc-c++ rpm-build curl clang

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install cargo-3pl --version 0.1.3

RUN echo -n "CENTOS8" > /OSRELEASE.txt

COPY build.sh /build.sh
CMD ["/build.sh"]
