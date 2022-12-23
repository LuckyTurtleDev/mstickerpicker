FROM alpine as builder

ARG RLOTTIE_VERSION=0.2

# install rlottie
RUN apk add cmake g++ gcc libc-dev ninja patch wget \
 && wget -q "https://github.com/Samsung/rlottie/archive/refs/tags/v$RLOTTIE_VERSION.tar.gz" \
 && tar xfz v$RLOTTIE_VERSION.tar.gz \
 && cd rlottie-$RLOTTIE_VERSION \
 && wget -q "https://aur.archlinux.org/cgit/aur.git/plain/0001-add-missing-include.patch?h=rlottie" --output-document=0001-add-missing-include.patch \
 && patch --strip=0 --input=0001-add-missing-include.patch \
 && mkdir -p build \
 && cd build \
 && cmake .. \
      -G Ninja \
      -DCMAKE_INSTALL_PREFIX=/usr \
      -DCMAKE_BUILD_TYPE=MinSizeRel \
      -DBUILD_SHARED_LIBS=OFF \
 && ninja \
 && ninja install

# unwanted files excluded by .dockerignore
COPY . /build

# compile mstickerpicher
RUN apk add cargo clang-dev pkgconf \
 && cd /build \
 && ls \
 && SQLX_OFFLINE=true cargo install --path . --locked --root /output




FROM alpine
COPY --from=builder /output/bin/mstickerpicker /mstickerpicker
COPY src/templates/ templates
RUN apk add libgcc
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_LOG_LEVEL=normal
CMD ["/mstickerpicker"]
USER 1000:1000