FROM ubuntu:focal

ENV HOME /root
ENV DEBIAN_FRONTEND=noninteractive 

# Dependencies
RUN apt-get update \
 && apt-get install -y lsb-release wget software-properties-common gnupg git build-essential libtool autoconf unzip libssl-dev

# CMAKE
RUN wget https://cmake.org/files/v3.27/cmake-3.27.9.tar.gz \
 && tar -xzvf cmake-3.27.9.tar.gz \
 && cd cmake-3.27.9/ \
 && ./bootstrap \
 && make -j$(nproc) \
 && make install

# LLVM
RUN wget https://apt.llvm.org/llvm.sh \
 && chmod +x ./llvm.sh \
 && ./llvm.sh 17

RUN apt-get update \
 && apt-get install -y libtbb-dev libc++-17-dev libc++abi-17-dev

CMD [ "/bin/bash" ]

WORKDIR $HOME
