FROM ubuntu:20.04

RUN apt update -y && apt upgrade -y
RUN apt install build-essential -y

RUN apt-get install curl -y
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
