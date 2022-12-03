FROM ubuntu:20.04

RUN apt update -y && \
    apt install software-properties-common -y && \
    add-apt-repository ppa:deadsnakes/ppa -y && \
    apt-get -y install python3-pip;

ADD requirements.txt  ./ 

RUN pip3 install -r requirements.txt 