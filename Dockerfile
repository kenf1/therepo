FROM python:3.12-slim

RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    software-properties-common \
    git \
    && rm -rf /var/lib/apt/lists/*

COPY ./src ./src
RUN python3 -m pip install --upgrade pip
RUN pip3 install -r ./src/requirements.txt