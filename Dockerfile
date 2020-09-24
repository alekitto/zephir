FROM bitnami/minideb:buster AS build-stage

RUN echo "deb http://deb.debian.org/debian buster-backports main" > /etc/apt/sources.list.d/buster-backports.list
RUN install_packages -t buster-backports cmake python3 curl make binutils g++ gcc \
    git ca-certificates python3-distutils python3-apt sed
RUN curl https://bootstrap.pypa.io/get-pip.py -o- | python3

RUN pip install conan

RUN mkdir -p /app/build
COPY CMakeLists.txt /app
COPY conanfile.txt /app

WORKDIR /app/build

RUN conan profile new default --detect && \
    conan profile update "settings.compiler.libcxx=libstdc++11" default

RUN conan install --build missing ..

COPY . /app
RUN cmake -DCMAKE_BUILD_TYPE=Release ..
RUN make -j2

# Execute tests
RUN bin/Google_Tests_run

FROM bitnami/minideb:buster AS dist-stage

COPY --from=build-stage /app/build/bin/zephir /zephir
RUN chmod 755 /zephir

CMD /zephir
EXPOSE 8091
