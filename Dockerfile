FROM ubuntu:latest
LABEL authors="otechdo"

ENTRYPOINT ["top", "-b"]